<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { emit, listen } from '@tauri-apps/api/event'
	import { seqtaFetch } from '../utils/seqtaFetch';

	import { onMount } from 'svelte';
	import '../app.css';
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
		ArrowRightOnRectangle
	} from 'svelte-hero-icons';

	import { writable } from 'svelte/store';
	export const needsSetup = writable(false);

	let seqtaUrl = $state<string>('');
	let userInfo = $state<UserInfo>();
	let { children } = $props();

	let weatherEnabled = $state(false);
	let weatherLocation = $state('');
	let weatherData: any = $state(null);
	let loadingWeather = $state(false);
	let weatherError = $state('');

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
			const res = await seqtaFetch('/seqta/student/login?', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json; charset=utf-8' },
				body: {}
			});
			userInfo = JSON.parse(res).payload;
		} catch (e) {
			console.error('Failed to load user info:', e);
		}
	}

	async function loadWeatherSettings() {
		try {
			const settings = await invoke<{ weather_enabled: boolean, weather_location: string }>('get_settings');
			weatherEnabled = settings.weather_enabled ?? false;
			weatherLocation = settings.weather_location ?? '';
		} catch (e) {
			weatherEnabled = false;
			weatherLocation = '';
		}
	}

	async function fetchWeather() {
		if (!weatherEnabled || !weatherLocation) {
			weatherData = null;
			return;
		}
		loadingWeather = true;
		weatherError = '';
		try {
			const geoRes = await fetch(`https://geocoding-api.open-meteo.com/v1/search?name=${encodeURIComponent(weatherLocation)}&count=1&language=en&format=json`);
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
		} catch (e) {
			weatherError = 'Failed to load weather.';
			weatherData = null;
		} finally {
			loadingWeather = false;
		}
	}

	onMount(async () => {
		await loadWeatherSettings();
		if (weatherEnabled && weatherLocation) fetchWeather();
	});

	$effect(() => {
		if (weatherEnabled && weatherLocation) fetchWeather();
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
		{ label: 'Documents', icon: DocumentText, path: '/documents' },
		{ label: 'Notices', icon: Bell, path: '/notices' },
		{ label: 'Reports', icon: ChartBar, path: '/reports' },
		{ label: 'Settings', icon: Cog6Tooth, path: '/settings' },
		{ label: 'Timetable', icon: CalendarDays, path: '/timetable' }
	];
</script>

<div class="flex pt-2 h-screen bg-slate-900">
	<aside
		class="flex flex-col justify-between px-2 pb-2 space-y-2 w-64 h-full"
	>
		<div class="flex overflow-y-scroll flex-col gap-2 h-full">
			<div class="flex sticky top-0 items-center px-4 pt-4 pb-2 w-full bg-slate-900">
				<img src="/32x32.png" alt="DesQTA Logo" class="mr-3 w-8 h-8 select-none" draggable="false" />
				<span class="text-lg font-bold tracking-wide">DesQTA</span>
			</div>
			{#each menu as item}
				<a href={item.path} class="flex items-center px-4 py-3 rounded transition-transform duration-300 hover:bg-slate-800 hover:scale-[1.03] group">
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
			{#if weatherEnabled && weatherLocation}
				<div class="my-4 mx-2 rounded-2xl shadow bg-gradient-to-br from-blue-900 to-blue-700 text-white p-4 flex flex-col justify-center animate-fade-in">
					{#if loadingWeather}
						<div>Loading weather…</div>
					{:else if weatherError}
						<div class="text-red-400">{weatherError}</div>
					{:else if weatherData}
						<div class="flex flex-col gap-1">
							<div class="text-base font-bold flex items-center gap-2">
								<span>Weather in {weatherData.location}, {weatherData.country}</span>
							</div>
							<div class="flex items-center gap-2 mt-2">
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
						<div class="flex justify-center items-center w-8 h-8 text-base font-bold text-white bg-blue-600 rounded-full select-none">
							{userInfo.userDesc?.split(' ').map((n: string) => n[0]).join('').slice(0,2)}
						</div>
						<div class="flex flex-col flex-1 min-w-0">
							<div class="flex gap-2 items-center">
								<span class="text-base font-semibold truncate">{userInfo.userDesc}</span>
							</div>
							<div class="flex gap-2 items-center min-w-0 text-xs text-slate-400">
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
						class="px-2 py-1 font-semibold rounded-lg transition text-zinc-400 hover:text-zinc-50"
					>
						<Icon src={ArrowLeftStartOnRectangle} class="size-4" />
					</button>
				{/if}
			</div>
		</div>
	</aside>

	<!-- Main Content -->
	<main class="overflow-y-scroll flex-1 w-full h-full overflow-clip rounded-tl-2xl bg-slate-950">
		{@render children()}
	</main>

	<!-- First‑run overlay -->
	{#if $needsSetup}
		<div class="flex fixed inset-0 z-50 justify-center items-center bg-black bg-opacity-50">
			<div class="flex flex-col justify-center items-center p-8 space-y-5 max-w-xl rounded-2xl shadow-xl bg-slate-900">
				<h2 class="text-xl font-bold">Connect to your SEQTA instance</h2>
				<p class="text-sm">
					Enter the full URL to your school's SEQTA page, then sign in in the window that opens. We'll
					securely save your session cookie.
				</p>
				<div class="flex items-center w-full">
					<Icon src={GlobeAlt} class="mr-2 w-5 h-5" />
					<input
						type="text"
						bind:value={seqtaUrl}
						placeholder="https://schoolname.seqta.com"
						class="px-3 py-2 w-full rounded-lg border outline-none focus:ring border-slate-800 bg-slate-800/40"
					/>
				</div>
				<button
					onclick={startLogin}
					class="flex justify-center items-center py-2 w-full font-semibold rounded-lg transition-transform duration-300 hover:scale-105"
					style="background: #2563eb; color: white;"
				>
					<Icon src={ArrowRightOnRectangle} class="mr-2 w-5 h-5" />
					Sign in
				</button>
			</div>
		</div>
	{/if}
</div> 