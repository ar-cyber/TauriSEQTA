<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { emit, listen } from '@tauri-apps/api/event'
	import { seqtaFetch } from '../utils/netUtil';
	import { cache } from '../utils/cache';

	import { onMount } from 'svelte';
	import '../app.css';
	import { accentColor, loadAccentColor, theme, loadTheme } from '../lib/stores/theme';
	import {
		Icon,
		Home,
		Newspaper,
		UserGroup,
		ClipboardDocumentList,
		BookOpen,
		Squares2x2,
		ChatBubbleLeftRight,
		DocumentText,
		AcademicCap,
		Bell,
		RectangleStack,
		ArrowLeftStartOnRectangle,
		ChartBar,
		Cog6Tooth,
		CalendarDays,
		GlobeAlt,
		ArrowRightOnRectangle,
		Bars3
	} from 'svelte-hero-icons';

	import { writable } from 'svelte/store';
	export const needsSetup = writable(false);

	let seqtaUrl = $state<string>('');
	let userInfo = $state<UserInfo>();
	let { children } = $props();

	let weatherEnabled = $state(false);
	let forceUseLocation = $state(true);
	let weatherCity = $state('');
	let weatherCountry = $state('');
	let weatherData: any = $state(null);
	let loadingWeather = $state(false);
	let weatherError = $state('');

	let isMobileMenuOpen = $state(false);
	let isMobile = $state(false);

	// Add hovered and selected state for login UI
	let hovered = $state<'extension' | 'web' | ''>('');
	let selected = $state<'extension' | 'web' | ''>('');

	async function checkSession() {
		const sessionExists = await invoke<boolean>('check_session_exists');
		needsSetup.set(!sessionExists);
		if (sessionExists) {
			loadUserInfo();
		}
	}

	onMount(checkSession);

	listen<string>('reload', (event) => {
		location.reload();
		checkSession();
	})

	async function startLogin() {
		if (!seqtaUrl) return;
		await invoke('create_login_window', { url: seqtaUrl });

		// Poll every 1.5 s until the cookie is saved (login window closes itself)
		const timer = setInterval(async () => {
			const sessionExists = await invoke<boolean>('check_session_exists');
			needsSetup.set(!sessionExists);
			if (sessionExists) clearInterval(timer);
		}, 1500);
	}

	async function getAPIData(url: string, parameters: Map<string, string>) {
		return await invoke('get_api_data', { url, parameters: Object.fromEntries(parameters) });
	}

	async function postAPIData(url: string, data: Map<string, string>) {
		return await invoke('post_api_data', { url, data: Object.fromEntries(data) });
	}

	async function handleLogout() {
		const success = await invoke('logout');
		if (success) {
			await checkSession();
		}
	}

	interface UserInfo {
		clientIP: string;
		email: string;
		id: number;
		lastAccessedTime: number;
		meta: {
			code: string;
			governmentID: string;
		};
		personUUID: string;
		saml: [{
			autologin: boolean;
			label: string;
			method: string;
			request: string;
			sigalg: URL;
			signature: string;
			slo: boolean;
			url: URL
		}];
		status: string;
		type: string;
		userCode: string;
		userDesc: string;
		userName: string;
	}

	async function loadUserInfo() {
		try {
			// Check cache first
			const cachedUserInfo = cache.get<UserInfo>('userInfo');
			if (cachedUserInfo) {
				userInfo = cachedUserInfo;
				return;
			}

			const res = await seqtaFetch('/seqta/student/login?', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json; charset=utf-8' },
				body: {}
			});
			userInfo = JSON.parse(res).payload;
			
			// Cache the user info for 5 minutes
			cache.set('userInfo', userInfo);
		} catch (e) {
			console.error('Failed to load user info:', e);
		}
	}

	async function loadWeatherSettings() {
		try {
			const settings = await invoke<{ weather_enabled: boolean, force_use_location: boolean, weather_city: string, weather_country?: string }>('get_settings');
			weatherEnabled = settings.weather_enabled ?? false;
			weatherCity = settings.weather_city ?? '';
			weatherCountry = settings.weather_country ?? '';
			forceUseLocation = settings.force_use_location ?? false;
		} catch (e) {
			weatherEnabled = false;
			weatherCity = '';
			weatherCountry = '';
		}
	}

	async function fetchWeatherWithIP() {
		if (!weatherEnabled) {
			weatherData = null;
			return;
		}

		// Check cache first
		const cachedWeather = cache.get<any>('weather');
		if (cachedWeather) {
			weatherData = cachedWeather;
			return;
		}

		loadingWeather = true;
		weatherError = '';

		try {
			let latitude: number, longitude: number, name: string, country: string;

			// Try IP-based geolocation
			try {
				const ipRes = await fetch('http://ip-api.com/json/');
				const ipJson = await ipRes.json();
				if (ipJson.status !== 'success') throw new Error('IP Geolocation failed');

				latitude = ipJson.lat;
				longitude = ipJson.lon;
				name = ipJson.city;
				country = ipJson.country;
			} catch (geoError) {
				// Fallback to manual location
				const geoRes = await fetch(`https://geocoding-api.open-meteo.com/v1/search?name=${encodeURIComponent(weatherCity)}&countryCode=${encodeURIComponent(weatherCountry)}&count=1&language=en&format=json`);
				const geoJson = await geoRes.json();
				if (!geoJson.results || !geoJson.results.length) throw new Error('Location not found via fallback');

				({ latitude, longitude, name, country } = geoJson.results[0]);
			}

			const weatherRes = await fetch(`https://api.open-meteo.com/v1/forecast?latitude=${latitude}&longitude=${longitude}&current_weather=true&timezone=auto`);
			const weatherJson = await weatherRes.json();

			weatherData = {
				...weatherJson.current_weather,
				location: name,
				country
			};

			// Cache weather data for 15 minutes
			cache.set('weather', weatherData, 15 * 60 * 1000);
		} catch (e) {
			console.log(e)
			weatherError = `Failed to load weather: ${e}`;
			weatherData = null;
		} finally {
			loadingWeather = false;
		}
	}

	async function fetchWeather() {
		if (!weatherEnabled || !weatherCity) {
			weatherData = null;
			return;
		}

		// Check cache first
		const cachedWeather = cache.get<any>('weather');
		if (cachedWeather) {
			weatherData = cachedWeather;
			return;
		}

		loadingWeather = true;
		weatherError = '';
		try {
			console.log(`https://geocoding-api.open-meteo.com/v1/search?name=${encodeURIComponent(weatherCity)}&countryCode=${encodeURIComponent(weatherCountry)}&count=1&language=en&format=json`);
			const geoRes = await fetch(`https://geocoding-api.open-meteo.com/v1/search?name=${encodeURIComponent(weatherCity)}&countryCode=${encodeURIComponent(weatherCountry)}&count=1&language=en&format=json`);
			const geoJson = await geoRes.json();
			if (!geoJson.results || !geoJson.results.length) throw new Error('Location not found');
			const { latitude, longitude, name, country } = geoJson.results[0];
			const weatherRes = await fetch(`https://api.open-meteo.com/v1/forecast?latitude=${latitude}&longitude=${longitude}&current_weather=true&timezone=auto`);
			const weatherJson = await weatherRes.json();
			weatherData = {
				...weatherJson.current_weather,
				location: name,
				country
			};

			// Cache weather data for 15 minutes
			cache.set('weather', weatherData, 15 * 60 * 1000);
		} catch (e) {
			console.log(e)
			weatherError = `Failed to load weather: ${e}`;
			weatherData = null;
		} finally {
			loadingWeather = false;
		}
	}

	$effect(() => {
		// Update the root element's data attributes with the current accent color
		document.documentElement.setAttribute('data-accent-color', '');
		document.documentElement.style.setProperty('--accent-color-value', $accentColor);
		// Theme application is now handled by the theme store
	});

	onMount(async () => {
		await Promise.all([
			checkSession(),
			loadWeatherSettings(),
			loadAccentColor(),
			loadTheme()
		]);
		if (weatherEnabled) {
			if (forceUseLocation) fetchWeather();
			else fetchWeatherWithIP();
		}
	});

	onMount(() => {
		const checkMobile = () => {
			isMobile = window.innerWidth < 768;
			if (!isMobile) isMobileMenuOpen = false;
		};
		
		checkMobile();
		window.addEventListener('resize', checkMobile);
		
		return () => {
			window.removeEventListener('resize', checkMobile);
		};
	});

	/* Sidebar menu items */
	const menu = [
		{ label: 'Home', icon: Home, path: '/' },
		{ label: 'News', icon: Newspaper, path: '/news' },
		{ label: 'Welcome', icon: UserGroup, path: '/welcome' },
		{ label: 'Assessments', icon: ClipboardDocumentList, hasSub: true, path: '/assessments' },
		{ label: 'Courses', icon: BookOpen, hasSub: true, path: '/courses' },
		{ label: 'Dashboard', icon: Squares2x2, path: '/dashboard' },
		{ label: 'Direqt Messages', icon: ChatBubbleLeftRight, path: '/direqt-messages' },
		{ label: 'Notices', icon: Bell, path: '/notices' },
		{ label: 'Reports', icon: ChartBar, path: '/reports' },
		{ label: 'Settings', icon: Cog6Tooth, path: '/settings' },
		{ label: 'Timetable', icon: CalendarDays, path: '/timetable' }
	];
</script>

<div class="flex flex-col pt-2 h-screen md:flex-row">
	<!-- Mobile Menu Button -->
	<button
		class="fixed top-4 right-4 z-50 p-2 bg-white rounded-lg md:hidden hover:bg-gray-100 dark:bg-slate-800 dark:hover:bg-slate-700"
		onclick={() => isMobileMenuOpen = !isMobileMenuOpen}
	>
		<Icon src={Bars3} class="w-6 h-6" />
	</button>

	<!-- Sidebar -->
	<aside
		class="flex fixed z-40 flex-col justify-between px-2 pb-2 space-y-2 w-full h-full transition-transform duration-300 ease-in-out transform md:w-64 md:relative"
		class:translate-x-0={isMobileMenuOpen || !isMobile}
		class:-translate-x-full={!isMobileMenuOpen && isMobile}
	>
		<div class="flex overflow-y-scroll flex-col gap-2 h-full">
			<div class="flex sticky top-0 items-center px-4 pt-4 pb-2 w-full bg-white dark:bg-slate-900">
				<img src="/32x32.png" alt="DesQTA Logo" class="mr-3 w-8 h-8 select-none" draggable="false" />
				<span class="text-lg font-bold tracking-wide">DesQTA</span>
			</div>
			{#each menu as item}
				<a 
					href={item.path} 
					class="flex items-center px-4 py-3 rounded transition-transform duration-300 hover:bg-gray-100 dark:hover:bg-slate-800 hover:scale-[1.03] group"
					onclick={() => isMobile && (isMobileMenuOpen = false)}
				>
					<Icon src={item.icon} class="mr-4 w-6 h-6" />
					<span class="text-base font-bold">{item.label}</span>
					{#if item.hasSub}
						<svg
							class="ml-auto w-4 h-4"
							fill="none"
							stroke="currentColor"
							stroke-width="2"
							viewBox="0 0 24 24"
						>
							<path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
						</svg>
					{/if}
				</a>
			{/each}
			{#if weatherEnabled}
				<div class="flex flex-col justify-center p-4 mx-2 my-4 text-white rounded-2xl shadow animate-fade-in" style="background: linear-gradient(120deg, var(--accent-color-value) 0%, color-mix(in srgb, var(--accent-color-value) 70%, black) 100%);">
					{#if loadingWeather}
						<div>Loading weather…</div>
					{:else if weatherError}
						<div class="text-red-400">{weatherError}</div>
					{:else if weatherData}
						<div class="flex flex-col gap-1">
							<div class="flex gap-2 items-center text-base font-bold">
								<span>Weather in {weatherData.location}, {weatherData.country}</span>
							</div>
							<div class="flex gap-2 items-center mt-2">
								<span class="text-2xl font-bold">{Math.round(weatherData.temperature)}°C</span>
								<span class="text-lg">{weatherData.weathercode === 0 ? '☀️' : weatherData.weathercode < 4 ? '🌤️' : weatherData.weathercode < 45 ? '☁️' : '🌧️'}</span>
								<span class="text-xs">{weatherData.windspeed} km/h wind</span>
							</div>
						</div>
					{/if}
				</div>
			{/if}
		</div>
		<div>
			<div class="flex justify-between">
				{#if userInfo}
					<div class="flex gap-3 items-center px-2 py-1 bg-transparent rounded-lg">
						<!-- Avatar with initials -->
						<div class="flex justify-center items-center w-8 h-8 text-base font-bold text-white rounded-full select-none" style="background-color: var(--accent-color-value);">
							{userInfo.userDesc?.split(' ').map((n: string) => n[0]).join('').slice(0,2)}
						</div>
						<div class="flex flex-col flex-1 min-w-0">
							<div class="flex gap-2 items-center">
								<span class="text-base font-semibold truncate">{userInfo.userDesc}</span>
							</div>
							<div class="flex gap-2 items-center min-w-0 text-xs text-gray-500 dark:text-slate-400">
								<span class="font-mono">{userInfo.userCode}</span>
								<span>•</span>
								<span class="font-mono">{userInfo.meta.governmentID}</span>
							</div>
						</div>
					</div>
				{/if}
				{#if !$needsSetup}
					<button 
						onclick={handleLogout}
						class="px-2 py-1 font-semibold text-gray-600 rounded-lg transition hover:text-gray-800 dark:text-zinc-400 dark:hover:text-zinc-50"
					>
						<Icon src={ArrowLeftStartOnRectangle} class="size-4" />
					</button>
				{/if}
			</div>
		</div>
	</aside>

	<!-- Main Content -->
	<main class="overflow-y-auto flex-1 bg-gray-50 rounded-tl-2xl dark:bg-slate-950">
		{#if $needsSetup}
			<div class="signin-container">
				<div class="signin-form-panel">
					<div class="signin-form-content">
						<img src="/32x32.png" alt="Logo" class="signin-logo" />
						<h2 class="signin-title">Sign in to your account</h2>
						<p class="signin-desc">Don't have an account? Ask an admin to create one for you.</p>
						<form onsubmit={startLogin} class="signin-form">
							<label for="seqtaUrl">SEQTA URL</label>
							<input id="seqtaUrl" type="text" bind:value={seqtaUrl} placeholder="https://schoolname.seqta.com" class="signin-input" autocomplete="username" required />
							<!-- If you want username/password fields instead, replace the above input with those fields and bind as needed -->
							<div class="signin-options">
								<label class="remember-me">
									<input type="checkbox" /> Remember me
								</label>
								<a href="#" class="forgot-password">Forgot password?</a>
							</div>
							<button type="submit" class="signin-btn">
								Sign in
							</button>
						</form>
					</div>
				</div>
				<div class="signin-image-panel"></div>
			</div>
			<style>
				.signin-container {
					display: flex;
					height: 100vh;
					width: 100vw;
				}
				.signin-form-panel {
					background: #232323;
					color: #fff;
					width: 100%;
					max-width: 480px;
					min-width: 320px;
					display: flex;
					align-items: center;
					justify-content: center;
					padding: 0 2rem;
				}
				.signin-form-content {
					width: 100%;
					max-width: 340px;
					margin: 0 auto;
					display: flex;
					flex-direction: column;
					gap: 1.5rem;
				}
				.signin-logo {
					width: 36px;
					margin-bottom: 2rem;
				}
				.signin-title {
					font-size: 1.6rem;
					font-weight: 700;
					margin-bottom: 0.5rem;
				}
				.signin-desc {
					color: #b0b0b0;
					font-size: 1rem;
					margin-bottom: 1.5rem;
				}
				.signin-form label {
					font-size: 1rem;
					margin-bottom: 0.25rem;
					margin-top: 1rem;
				}
				.signin-input {
					width: 100%;
					padding: 0.7rem 0.9rem;
					border-radius: 6px;
					border: none;
					background: #f3f4f6;
					color: #232323;
					font-size: 1rem;
					margin-bottom: 0.5rem;
				}
				.signin-options {
					display: flex;
					justify-content: space-between;
					align-items: center;
					margin: 1rem 0;
				}
				.remember-me {
					display: flex;
					align-items: center;
					font-size: 0.95rem;
					color: #b0b0b0;
				}
				.forgot-password {
					color: #3b82f6;
					text-decoration: none;
					font-size: 0.95rem;
				}
				.signin-btn {
					width: 100%;
					background: #2563eb;
					color: #fff;
					font-weight: 600;
					font-size: 1.1rem;
					border: none;
					border-radius: 6px;
					padding: 0.8rem 0;
					margin-top: 0.5rem;
					cursor: pointer;
					transition: background 0.2s;
				}
				.signin-btn:hover {
					background: #1d4ed8;
				}
				.signin-image-panel {
					flex: 1;
					background: url('/images/signin.jpg') center center / cover no-repeat;
					min-width: 0;
					min-height: 100vh;
				}
				@media (max-width: 900px) {
					.signin-container {
						flex-direction: column;
					}
					.signin-image-panel {
						display: none;
					}
					.signin-form-panel {
						max-width: 100vw;
						min-width: 0;
					}
				}
			</style>
		{:else}
			{@render children()}
		{/if}
	</main>
</div>

<!-- Mobile Menu Overlay -->
{#if isMobile && isMobileMenuOpen}
	<div 
		class="fixed inset-0 z-30 bg-black bg-opacity-50 dark:bg-black dark:bg-opacity-50"
		onclick={() => isMobileMenuOpen = false}
		role="button"
		tabindex="0"
		onkeydown={(e) => e.key === 'Enter' && (isMobileMenuOpen = false)}
	></div>
{/if} 
