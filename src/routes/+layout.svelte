<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { emit, listen } from '@tauri-apps/api/event'
	import { seqtaFetch } from '../utils/netUtil';
	import { cache } from '../utils/cache';

	import { onMount, onDestroy } from 'svelte';
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
		Bars3,
		XMark
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

	let unlisten: (() => void) | undefined;
	onMount(async () => {
		unlisten = await listen<string>('reload', () => {
			location.reload();
		});
	});

	onDestroy(() => {
		if (unlisten) unlisten();
	});

	async function startLogin() {
		if (!seqtaUrl) return;
		await invoke('create_login_window', { url: seqtaUrl });

		// Poll every 1s until the cookie is saved (login window closes itself)
		const timer = setInterval(async () => {
			const sessionExists = await invoke<boolean>('check_session_exists');
			if (sessionExists) {
				clearInterval(timer);
				needsSetup.set(false);
				await loadUserInfo();
			}
		}, 1000);

		// Clear interval after 5 minutes to prevent infinite polling
		setTimeout(() => clearInterval(timer), 5 * 60 * 1000);
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
		{ label: 'Timetable', icon: CalendarDays, path: '/timetable' },
		{ label: 'Analytics', icon: ChartBar, path: '/analytics' }
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
							alt=""
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
				<a
					href="/analytics"
					class="flex items-center gap-4 px-5 py-3 text-lg font-semibold rounded-xl bg-base-200 hover:accent-bg transition-all duration-200 shadow-sm transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 accent-ring"
					class:accent-bg={$page.url.pathname.startsWith('/analytics')}
				>
					<Icon src={ChartBar} class="w-7 h-7" />
					<span>Analytics</span>
				</a>
			</nav>
		</aside>

		<!-- Main Content -->
		<main class="flex-1 transition-all duration-300" class:ml-64={sidebarOpen}>
			<div class="{sidebarOpen ? 'container mx-auto' : 'w-full'} p-6 transition-all duration-300">
				{#if $needsSetup}
					<div class="min-h-screen flex items-center justify-center p-4 bg-gradient-to-br from-gray-50 to-gray-100 dark:from-slate-900 dark:to-slate-800">
						<div class="w-full max-w-5xl bg-white dark:bg-slate-900 rounded-2xl shadow-2xl flex flex-col md:flex-row overflow-hidden animate-fade-in-up border border-gray-200 dark:border-slate-800">
							<!-- Left side - Image and branding -->
							<div class="hidden md:block md:w-2/3 relative">
								<div class="absolute inset-0 bg-gradient-to-br from-indigo-500/20 to-purple-500/20 dark:from-indigo-500/10 dark:to-purple-500/10"></div>
								<img src="/images/signin.jpg" alt="Sign in" class="object-cover w-full h-full min-h-[600px]" />
								<div class="absolute inset-0 bg-gradient-to-t from-black/50 to-transparent flex flex-col justify-end p-8">
									<h1 class="text-4xl font-bold text-white mb-4">Welcome to DesQTA</h1>
									<p class="text-lg text-gray-200">Experience SEQTA Learn like never before with our powerful desktop application</p>
								</div>
							</div>

							<!-- Right side - Login form -->
							<div class="w-full md:w-1/3 flex flex-col justify-center p-8 md:p-12">
								<div class="md:hidden mb-8 text-center">
									<h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">Welcome to DesQTA</h1>
									<p class="text-gray-600 dark:text-gray-300">Experience SEQTA Learn like never before with our powerful desktop application</p>
						</div>

							<div class="space-y-6">
									<div>
										<label for="seqta-url" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
											SEQTA URL
										</label>
										<div class="relative">
											<div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
												<svg class="h-5 w-5 text-gray-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
													<path fill-rule="evenodd" d="M12.586 4.586a2 2 0 112.828 2.828l-3 3a2 2 0 01-2.828 0 1 1 0 00-1.414 1.414 4 4 0 005.656 0l3-3a4 4 0 00-5.656-5.656l-1.5 1.5a1 1 0 101.414 1.414l1.5-1.5zm-5 5a2 2 0 012.828 0 1 1 0 101.414-1.414 4 4 0 00-5.656 0l-3 3a4 4 0 105.656 5.656l1.5-1.5a1 1 0 10-1.414-1.414l-1.5 1.5a2 2 0 11-2.828-2.828l3-3z" clip-rule="evenodd" />
												</svg>
											</div>
								<input
												id="seqta-url"
									type="text"
									bind:value={seqtaUrl}
												placeholder="your-school.seqta.com.au"
												class="w-full pl-10 pr-4 py-3 rounded-lg bg-gray-50 dark:bg-slate-800 text-gray-900 dark:text-white border border-gray-300 dark:border-slate-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-colors"
											/>
										</div>
										<p class="mt-2 text-sm text-gray-500 dark:text-gray-400">
											Enter your school's SEQTA URL to get started
										</p>
									</div>

								<button
										class="w-full py-3 rounded-lg bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-700 hover:to-purple-700 text-white font-semibold text-lg shadow-lg transition-all duration-200 transform hover:scale-[1.02] active:scale-[0.98] disabled:opacity-60 disabled:cursor-not-allowed disabled:hover:scale-100"
									onclick={startLogin}
									disabled={!seqtaUrl}
								>
									Sign In
								</button>

									<div class="text-center">
										<p class="text-sm text-gray-600 dark:text-gray-400">
											Need help? <a href="https://github.com/betterseqta/desqta" target="_blank" rel="noopener noreferrer" class="text-indigo-600 dark:text-indigo-400 hover:underline">Visit GitHub</a>
										</p>
									</div>
								</div>
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

	@keyframes dropdown-fade-in {
		0% { opacity: 0; transform: translateY(-10px); }
		100% { opacity: 1; transform: translateY(0); }
	}
	.dropdown-animate-in {
		animation: dropdown-fade-in 0.18s cubic-bezier(0.4,0,0.2,1);
	}

	/* Konami Code Easter Egg Animation */
	.konami-mode {
		animation: rainbow 5s linear infinite;
	}

	@keyframes rainbow {
		0% { filter: hue-rotate(0deg); }
		100% { filter: hue-rotate(360deg); }
	}
</style>

<!-- Mobile Menu Overlay -->
{#if isMobileMenuOpen}
	<div
		class="fixed inset-0 bg-black/50 z-40"
		onclick={() => (isMobileMenuOpen = false)}
		onkeydown={(e) => e.key === 'Escape' && (isMobileMenuOpen = false)}
		role="button"
		tabindex="0"
		aria-label="Close mobile menu"
	></div>
{/if}

<!-- About Modal -->
{#if showAboutModal}
	<div
		class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center p-4"
		onclick={() => (showAboutModal = false)}
		onkeydown={(e) => e.key === 'Escape' && (showAboutModal = false)}
		role="dialog"
		aria-modal="true"
		aria-label="About Modal"
		tabindex="0"
	>
		<div
			class="bg-white dark:bg-slate-900 rounded-2xl shadow-2xl w-full max-w-2xl overflow-hidden animate-fade-in-up"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
			role="document"
		>
			<div class="relative">
				<!-- Easter Egg Trigger Area -->
				<div 
					class="absolute top-0 right-0 w-16 h-16 cursor-pointer opacity-0"
					onclick={(e) => {
						e.stopPropagation();
						const audio = new Audio('/sounds/konami.mp3');
						audio.play();
						document.body.classList.add('konami-mode');
						setTimeout(() => document.body.classList.remove('konami-mode'), 5000);
					}}
				></div>
				
				<div class="p-8">
					<div class="flex justify-between items-start mb-8">
						<div>
							<h2 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">About DesQTA</h2>
							<p class="text-gray-600 dark:text-gray-400">Your SEQTA Learn Desktop Experience</p>
						</div>
					<button
							class="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 rounded-lg transition-colors hover:scale-110 active:scale-95"
						onclick={() => (showAboutModal = false)}
						aria-label="Close modal"
					>
						<Icon src={XMark} class="w-6 h-6" />
					</button>
				</div>

					<div class="grid grid-cols-1 md:grid-cols-2 gap-8 items-stretch">
						<div class="flex flex-col gap-6 h-full">
							<div class="bg-gray-50 dark:bg-slate-800/50 rounded-xl p-6 flex-1 flex flex-col justify-between min-h-[160px]">
								<h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">Version Information</h3>
								<div class="space-y-3">
									<div class="flex items-center justify-between">
										<span class="text-gray-600 dark:text-gray-400">Version</span>
										<span class="font-medium text-gray-900 dark:text-white">1.0.0</span>
									</div>
									<div class="flex items-center justify-between">
										<span class="text-gray-600 dark:text-gray-400">Build Date</span>
										<span class="font-medium text-gray-900 dark:text-white">{new Date().toLocaleDateString()}</span>
									</div>
									<div class="flex items-center justify-between">
										<span class="text-gray-600 dark:text-gray-400">Platform</span>
										<span class="font-medium text-gray-900 dark:text-white">Desktop</span>
									</div>
								</div>
							</div>
							<div class="bg-gray-50 dark:bg-slate-800/50 rounded-xl p-6 flex-1 flex flex-col justify-between min-h-[160px]">
								<h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">Built With</h3>
								<div class="grid grid-cols-2 gap-4">
									<div class="flex items-center space-x-3">
										<div class="w-8 h-8 rounded-lg bg-[#FF3E00] flex items-center justify-center text-white font-bold">S</div>
										<span class="text-gray-900 dark:text-white">SvelteKit</span>
									</div>
									<div class="flex items-center space-x-3">
										<div class="w-8 h-8 rounded-lg bg-[#FFC131] flex items-center justify-center text-white font-bold">T</div>
										<span class="text-gray-900 dark:text-white">Tauri</span>
									</div>
									<div class="flex items-center space-x-3">
										<div class="w-8 h-8 rounded-lg bg-[#06B6D4] flex items-center justify-center text-white font-bold">T</div>
										<span class="text-gray-900 dark:text-white">TailwindCSS</span>
									</div>
									<div class="flex items-center space-x-3">
										<div class="w-8 h-8 rounded-lg bg-[#764ABC] flex items-center justify-center text-white font-bold">R</div>
										<span class="text-gray-900 dark:text-white">Rust</span>
									</div>
								</div>
							</div>
						</div>
						<div class="flex flex-col gap-6 h-full">
							<div class="bg-gray-50 dark:bg-slate-800/50 rounded-xl p-6 flex-1 flex flex-col justify-between min-h-[160px]">
								<h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">Features</h3>
								<ul class="space-y-3 max-h-40 overflow-auto pr-2">
									<li class="flex items-center space-x-3">
										<Icon src={BookOpen} class="w-5 h-5 text-indigo-500" />
										<span class="text-gray-900 dark:text-white">Course Management</span>
									</li>
									<li class="flex items-center space-x-3">
										<Icon src={ClipboardDocumentList} class="w-5 h-5 text-indigo-500" />
										<span class="text-gray-900 dark:text-white">Assessment Tracking</span>
									</li>
									<li class="flex items-center space-x-3">
										<Icon src={CalendarDays} class="w-5 h-5 text-indigo-500" />
										<span class="text-gray-900 dark:text-white">Timetable Integration</span>
									</li>
									<li class="flex items-center space-x-3">
										<Icon src={ChatBubbleLeftRight} class="w-5 h-5 text-indigo-500" />
										<span class="text-gray-900 dark:text-white">Direqt Messaging</span>
									</li>
									<li class="flex items-center space-x-3">
										<Icon src={Bell} class="w-5 h-5 text-indigo-500" />
										<span class="text-gray-900 dark:text-white">Real-time Notifications</span>
									</li>
									<li class="flex items-center space-x-3">
										<Icon src={DocumentText} class="w-5 h-5 text-indigo-500" />
										<span class="text-gray-900 dark:text-white">Notice Board</span>
									</li>
					</ul>
							</div>
							<div class="bg-gray-50 dark:bg-slate-800/50 rounded-xl p-6 flex-1 flex flex-col justify-between min-h-[160px]">
								<h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">Links</h3>
								<div class="space-y-3">
									<a href="https://github.com/betterseqta/desqta" target="_blank" rel="noopener noreferrer" 
										class="flex items-center space-x-3 text-gray-900 dark:text-white hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors">
										<svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
											<path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd" />
										</svg>
										<span>GitHub Repository</span>
									</a>
									<a href="https://desqta.betterseqta.org" target="_blank" rel="noopener noreferrer"
										class="flex items-center space-x-3 text-gray-900 dark:text-white hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors">
										<Icon src={GlobeAlt} class="w-5 h-5" />
										<span>Website</span>
									</a>
								</div>
							</div>
						</div>
					</div>

					<div class="mt-8 pt-6 border-t border-gray-200 dark:border-slate-700">
						<p class="text-center text-gray-600 dark:text-gray-400">
							© 2025 DesQTA. Licensed under MIT License.
						</p>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}
