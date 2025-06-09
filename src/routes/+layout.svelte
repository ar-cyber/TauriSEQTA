<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { emit, listen } from '@tauri-apps/api/event'
	import { seqtaFetch } from '../utils/netUtil';
	import { cache } from '../utils/cache';

	import { onMount } from 'svelte';
	import '../app.css';
	import { page } from '$app/stores';
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

	let sidebarOpen = $state(true);
	let isDarkMode = $derived($theme === 'dark');
	let notifications = $state([]);
	let unreadNotifications = $state(0);

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

<div class="min-h-screen bg-base-100 flex flex-col" class:dark={isDarkMode}>
	<!-- Top Bar -->
	<header class="h-16 fixed top-0 left-0 right-0 z-50 flex items-center justify-between px-4 border-b border-base-300 backdrop-blur-md bg-white/70 dark:bg-slate-900/60 shadow-sm">
		<div class="flex items-center space-x-4">
	<button
				class="p-3 rounded-xl bg-base-200 hover:accent-bg shadow-md transition-all duration-200 transform hover:scale-110 active:scale-95 focus:outline-none focus:ring-2 accent-ring"
				onclick={() => (sidebarOpen = !sidebarOpen)}
				aria-label="Toggle sidebar"
	>
		<Icon src={Bars3} class="w-6 h-6" />
	</button>
			<h1 class="text-xl font-semibold">DesQTA</h1>
		</div>
		
		<div class="flex items-center space-x-4">
			<button
				class="p-2 hover:bg-base-300 rounded-lg transition-colors relative"
				onclick={() => (notifications = [])}
			>
				<Icon src={Bell} class="w-6 h-6" />
				{#if unreadNotifications > 0}
					<span class="absolute -top-1 -right-1 bg-primary text-primary-content text-xs rounded-full w-5 h-5 flex items-center justify-center">
						{unreadNotifications}
					</span>
				{/if}
			</button>
			
			{#if userInfo}
				<div class="flex items-center space-x-2">
					<img
						src={`https://api.dicebear.com/7.x/identicon/svg?seed=${userInfo.userName}`}
						alt="Profile picture"
						class="w-8 h-8 rounded-full border border-gray-300 dark:border-slate-700 shadow-sm object-cover"
					/>
					<span class="hidden md:inline">{userInfo.userName}</span>
					<button
						class="ml-2 p-2 rounded-lg hover:accent-bg focus:accent-bg active:accent-bg transition-colors transition-transform duration-200 text-gray-500 dark:text-gray-300 focus:outline-none focus:ring-2 accent-ring transform hover:scale-105 active:scale-95"
						onclick={handleLogout}
						aria-label="Logout"
						title="Logout"
					>
						<Icon src={ArrowRightOnRectangle} class="w-6 h-6" />
					</button>
				</div>
			{/if}
		</div>
	</header>

	<div class="flex flex-1 pt-16">
		<!-- Sidebar -->
		<aside
			class="bg-base-200 border-r border-base-300 w-64 fixed left-0 top-16 bottom-0 transition-transform duration-300 ease-in-out z-40"
			class:translate-x-[-100%]={!sidebarOpen}
		>
			<nav class="p-4 space-y-2">
				<a
					href="/"
					class="flex items-center gap-4 px-5 py-3 text-lg font-semibold rounded-xl bg-base-200 hover:accent-bg transition-all duration-200 shadow-sm transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 accent-ring"
					class:accent-bg={$page.url.pathname === '/'}
				>
					<Icon src={Home} class="w-7 h-7" />
					<span>Dashboard</span>
				</a>
				
				<a
					href="/courses"
					class="flex items-center gap-4 px-5 py-3 text-lg font-semibold rounded-xl bg-base-200 hover:accent-bg transition-all duration-200 shadow-sm transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 accent-ring"
					class:accent-bg={$page.url.pathname.startsWith('/courses')}
				>
					<Icon src={BookOpen} class="w-7 h-7" />
					<span>Courses</span>
				</a>
				
				<a
					href="/assessments"
					class="flex items-center gap-4 px-5 py-3 text-lg font-semibold rounded-xl bg-base-200 hover:accent-bg transition-all duration-200 shadow-sm transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 accent-ring"
					class:accent-bg={$page.url.pathname.startsWith('/assessments')}
				>
					<Icon src={ClipboardDocumentList} class="w-7 h-7" />
					<span>Assessments</span>
				</a>
				
				<a
					href="/timetable"
					class="flex items-center gap-4 px-5 py-3 text-lg font-semibold rounded-xl bg-base-200 hover:accent-bg transition-all duration-200 shadow-sm transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 accent-ring"
					class:accent-bg={$page.url.pathname.startsWith('/timetable')}
				>
					<Icon src={CalendarDays} class="w-7 h-7" />
					<span>Timetable</span>
				</a>
				
				<a
					href="/direqt-messages"
					class="flex items-center gap-4 px-5 py-3 text-lg font-semibold rounded-xl bg-base-200 hover:accent-bg transition-all duration-200 shadow-sm transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 accent-ring"
					class:accent-bg={$page.url.pathname.startsWith('/direqt-messages')}
				>
					<Icon src={ChatBubbleLeftRight} class="w-7 h-7" />
					<span>Messages</span>
				</a>
				
				<a
					href="/notices"
					class="flex items-center gap-4 px-5 py-3 text-lg font-semibold rounded-xl bg-base-200 hover:accent-bg transition-all duration-200 shadow-sm transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 accent-ring"
					class:accent-bg={$page.url.pathname.startsWith('/notices')}
				>
					<Icon src={DocumentText} class="w-7 h-7" />
					<span>Notices</span>
				</a>
				
				<a
					href="/news"
					class="flex items-center gap-4 px-5 py-3 text-lg font-semibold rounded-xl bg-base-200 hover:accent-bg transition-all duration-200 shadow-sm transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 accent-ring"
					class:accent-bg={$page.url.pathname.startsWith('/news')}
				>
					<Icon src={Newspaper} class="w-7 h-7" />
					<span>News</span>
				</a>
				
				<a
					href="/reports"
					class="flex items-center gap-4 px-5 py-3 text-lg font-semibold rounded-xl bg-base-200 hover:accent-bg transition-all duration-200 shadow-sm transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 accent-ring"
					class:accent-bg={$page.url.pathname.startsWith('/reports')}
				>
					<Icon src={ChartBar} class="w-7 h-7" />
					<span>Reports</span>
				</a>
				
				<a
					href="/settings"
					class="flex items-center gap-4 px-5 py-3 text-lg font-semibold rounded-xl bg-base-200 hover:accent-bg transition-all duration-200 shadow-sm transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 accent-ring"
					class:accent-bg={$page.url.pathname.startsWith('/settings')}
				>
					<Icon src={Cog6Tooth} class="w-7 h-7" />
					<span>Settings</span>
				</a>
			</nav>
		</aside>

		<!-- Main Content -->
		<main class="flex-1 transition-all duration-300" class:ml-64={sidebarOpen}>
			<div class="{sidebarOpen ? 'container mx-auto' : 'w-full'} p-6 transition-all duration-300">
				{#if $needsSetup}
					<div class="max-w-5xl mx-auto mt-24 bg-white dark:bg-slate-900 rounded-2xl shadow-2xl flex flex-col md:flex-row overflow-hidden animate-fade-in-up border border-gray-200 dark:border-slate-800">
						<div class="hidden md:block md:w-2/3 bg-gray-100 dark:bg-slate-800">
							<img src="/images/signin.jpg" alt="Sign in" class="object-cover w-full h-full min-h-[400px]" />
						</div>
						<div class="w-full md:w-1/3 flex flex-col justify-center p-10 md:p-16">
							<h2 class="text-4xl font-extrabold mb-2 text-gray-900 dark:text-white">Welcome to DesQTA</h2>
							<p class="mb-8 text-lg text-gray-600 dark:text-slate-300">Sign in to continue to your dashboard</p>
							<div class="space-y-6">
								<input
									type="text"
									bind:value={seqtaUrl}
									placeholder="https://your-school.seqta.com.au"
									class="w-full px-5 py-4 rounded-lg bg-gray-100 dark:bg-slate-800 text-gray-900 dark:text-white border border-gray-300 dark:border-slate-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 shadow-sm text-lg"
								/>
								<button
									class="w-full py-4 rounded-lg bg-indigo-600 hover:bg-indigo-700 text-white font-semibold text-xl shadow-lg transition-colors disabled:opacity-60 disabled:cursor-not-allowed"
									onclick={startLogin}
									disabled={!seqtaUrl}
								>
									Sign In
								</button>
							</div>
						</div>
					</div>
				{:else}
					<slot />
				{/if}
			</div>
		</main>
					</div>
				</div>

			<style>
	/* Add smooth transitions */
	.transition-transform {
		transition-property: transform;
		transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
		transition-duration: 300ms;
	}

	/* Add hover effects */
	.hover\:bg-base-300:hover {
		background-color: hsl(var(--b3));
	}

	/* Add active state styles */
	.bg-base-300 {
		background-color: hsl(var(--b3));
				}
			</style>

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
