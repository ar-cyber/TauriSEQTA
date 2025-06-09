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

	// Add state for dropdown and about modal
	let showUserDropdown = $state(false);
	let showAboutModal = $state(false);
	let aboutClosing = $state(false);

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
		displayName?: string;
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
				class="p-3 rounded-xl bg-base-200 hover:bg-gray-300 dark:hover:bg-slate-700 hover:text-gray-900 dark:hover:text-white shadow-md transition-all duration-200 transform hover:scale-110 active:scale-95 focus:outline-none focus:ring-2 focus:ring-gray-400"
				onclick={() => (sidebarOpen = !sidebarOpen)}
				aria-label="Toggle sidebar"
			>
				<Icon src={Bars3} class="w-6 h-6" />
			</button>
			<h1 class="text-xl font-semibold">DesQTA</h1>
			{#if weatherEnabled && weatherData}
				<div class="flex items-center gap-2 px-3 py-1 rounded-lg bg-base-200/80 dark:bg-slate-800/60 shadow text-sm font-medium ml-2">
					<span class="flex items-center gap-1">
						<svg width="22" height="22" fill="none" viewBox="0 0 24 24">
							{#if weatherData.weathercode === 0}
								<!-- Clear -->
								<circle cx="12" cy="12" r="8" fill="#facc15" />
							{:else if weatherData.weathercode === 1 || weatherData.weathercode === 2}
								<!-- Partly Cloudy -->
								<ellipse cx="12" cy="15" rx="7" ry="4" fill="#a3a3a3" />
								<circle cx="16" cy="10" r="5" fill="#facc15" />
							{:else}
								<!-- Cloudy/Other -->
								<ellipse cx="12" cy="15" rx="7" ry="4" fill="#a3a3a3" />
							{/if}
						</svg>
						<span>{Math.round(weatherData.temperature)}°C</span>
					</span>
					<span class="hidden sm:inline text-gray-600 dark:text-gray-300">{weatherData.location}</span>
				</div>
			{/if}
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
				<div class="relative">
					<button
						class="flex items-center gap-2 px-3 py-1.5 rounded-full bg-base-200 hover:bg-base-300 shadow transition-all duration-200 focus:outline-none focus:ring-2 accent-ring"
						onclick={() => showUserDropdown = !showUserDropdown}
						aria-label="User menu"
						tabindex="0"
					>
						<img
							src={`https://api.dicebear.com/7.x/identicon/svg?seed=${userInfo.userName}`}
							alt="Profile picture"
							class="w-8 h-8 rounded-full border border-gray-300 dark:border-slate-700 shadow-sm object-cover"
						/>
						<span class="hidden md:inline font-semibold text-gray-900 dark:text-white">
							{userInfo.userDesc || userInfo.userName}
						</span>
					</button>
					{#if showUserDropdown}
						<div class="absolute right-0 mt-2 w-48 bg-white dark:bg-slate-900 rounded-xl shadow-lg border border-gray-200 dark:border-slate-700 z-50 dropdown-animate-in">
							<button
								class="w-full text-left px-5 py-3 rounded-t-xl hover:bg-base-200 transition-colors transition-transform duration-200 text-gray-700 dark:text-gray-200 hover:scale-105 active:scale-95"
								onclick={() => { showUserDropdown = false; showAboutModal = true; }}
							>
								About
							</button>
							<button
								class="w-full text-left px-5 py-3 rounded-b-xl hover:bg-base-200 transition-colors transition-transform duration-200 text-gray-700 dark:text-gray-200 hover:scale-105 active:scale-95"
								onclick={() => { showUserDropdown = false; handleLogout(); }}
							>
								Sign out
							</button>
						</div>
					{/if}
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

	@keyframes dropdown-fade-in {
		0% { opacity: 0; transform: translateY(-10px); }
		100% { opacity: 1; transform: translateY(0); }
	}
	@keyframes dropdown-fade-out {
		0% { opacity: 1; transform: translateY(0); }
		100% { opacity: 0; transform: translateY(-10px); }
	}
	.dropdown-animate-in {
		animation: dropdown-fade-in 0.18s cubic-bezier(0.4,0,0.2,1);
	}
	.dropdown-animate-out {
		animation: dropdown-fade-out 0.15s cubic-bezier(0.4,0,0.2,1);
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

<!-- About Modal -->
{#if showAboutModal || aboutClosing}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 backdrop-blur-sm"
		tabindex="0" role="dialog" aria-modal="true"
		class:animate-fade-in={showAboutModal && !aboutClosing}
		class:animate-fade-out={aboutClosing}
		onclick={() => { aboutClosing = true; setTimeout(() => { showAboutModal = false; aboutClosing = false; }, 200); }}>
		<div class="bg-white dark:bg-slate-900 rounded-2xl shadow-2xl p-8 max-w-md w-full relative"
			onclick={(e) => e.stopPropagation()}
			class:animate-fade-in={showAboutModal && !aboutClosing}
			class:animate-fade-out={aboutClosing}
		>
			<button class="absolute top-4 right-4 p-2 rounded-lg hover:bg-base-200 transition-colors" onclick={() => { aboutClosing = true; setTimeout(() => { showAboutModal = false; aboutClosing = false; }, 200); }} aria-label="Close About">
				<svg width="24" height="24" fill="none" viewBox="0 0 24 24"><path d="M6 6l12 12M6 18L18 6" stroke="currentColor" stroke-width="2" stroke-linecap="round"/></svg>
			</button>
			<h2 class="text-2xl font-bold mb-2 text-gray-900 dark:text-white">About DesQTA</h2>
			<p class="mb-4 text-gray-700 dark:text-slate-300">DesQTA is a modern, beautiful SEQTA client for desktop, designed for productivity and delight. Built with Svelte, Tauri, and love.</p>
			<div class="text-sm text-gray-500 dark:text-slate-400">Version 1.0.0<br/>© 2025 DesQTA Team</div>
		</div>
	</div>
{/if} 
