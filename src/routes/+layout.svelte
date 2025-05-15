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

<div class="h-screen bg-[var(--surface)]" style="color: var(--text);">
	<!-- Top Bar -->
	<header class="flex fixed top-0 right-0 left-0 justify-between items-center place-items-center px-8 h-12" style="background: var(--surface); color: var(--text);">
		<div class="flex items-center">
			<img src="/32x32.png" alt="DesQTA Logo" class="mr-3 w-8 h-8 select-none" draggable="false" />
			<span class="text-lg font-bold tracking-wide">DesQTA</span>
		</div>
		<div class="flex gap-4 items-center">
			{#if userInfo}
				<div class="flex items-center gap-3 px-3 py-1 rounded-lg min-w-[320px]" style="background: transparent;">
					<!-- Avatar with initials -->
					<div class="flex justify-center items-center w-8 h-8 text-base font-bold text-white bg-blue-600 rounded-full select-none">
						{userInfo.userDesc?.split(' ').map((n: string) => n[0]).join('').slice(0,2)}
					</div>
					<div class="flex flex-col flex-1 min-w-0">
						<div class="flex gap-2 items-center">
							<span class="text-base font-semibold truncate">{userInfo.userDesc}</span>
							<span class="px-2 py-0.5 text-xs tracking-wide text-blue-400 uppercase rounded border bg-blue-500/10 border-blue-400/20">{userInfo.type}</span>
						</div>
						<div class="flex items-center gap-2 text-xs text-[var(--text-muted)] min-w-0">
							<span class="truncate" title={userInfo.email}>{userInfo.email}</span>
							<span>•</span>
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
					class="px-4 py-1 rounded-lg font-semibold hover:scale-[1.02] transition"
					style="background: var(--surface-alt); color: var(--text);"
				>
					Logout
				</button>
			{/if}
		</div>
	</header>

	<div class="flex pt-12 h-full">
		<!-- Sidebar -->
		<aside
			class="flex overflow-y-scroll flex-col px-2 pb-6 space-y-2 w-64 h-full"
			style="background: var(--surface); color: var(--text);"
		>
			{#each menu as item}
				<a href={item.path} class="flex items-center px-4 py-3 rounded hover:bg-[color:var(--surface-alt)] transition-transform duration-300 hover:scale-105 group">
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
		</aside>

		<!-- Main Content -->
		<main
			class="overflow-y-scroll flex-1 w-full h-full overflow-clip rounded-tl-2xl"
			style="background: var(--background); color: var(--text);"
		>
			{@render children()}
		</main>
	</div>

	<!-- First‑run overlay -->
	{#if $needsSetup}
		<div class="flex fixed inset-0 z-50 justify-center items-center bg-black bg-opacity-50">
			<div class="bg-[color:var(--surface)] rounded-2xl shadow-xl p-8 w-full h-full flex flex-col justify-center items-center space-y-5">
				<h2 class="text-xl font-bold" style="color: var(--text);">Connect to your SEQTA instance</h2>
				<p class="text-sm" style="color: var(--text-muted);">
					Enter the full URL to your school's SEQTA page, then sign in in the window that opens. We'll
					securely save your session cookie.
				</p>
				<div class="flex items-center">
					<Icon src={GlobeAlt} class="mr-2 w-5 h-5" />
				<input
					type="text"
					bind:value={seqtaUrl}
					placeholder="https://schoolname.seqta.com"
						class="px-3 py-2 w-full max-w-xl rounded-lg border outline-none focus:ring"
					style="background: var(--surface-alt); color: var(--text); border-color: var(--surface-alt);"
				/>
				</div>
				<button
					onclick={startLogin}
					class="flex justify-center items-center py-2 w-full max-w-md font-semibold rounded-lg transition-transform duration-300 hover:scale-105"
					style="background: #2563eb; color: white;"
				>
					<Icon src={ArrowRightOnRectangle} class="mr-2 w-5 h-5" />
					Sign in
				</button>
			</div>
		</div>
	{/if}
</div> 