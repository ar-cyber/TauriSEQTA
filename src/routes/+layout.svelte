<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import '../app.css';
	import { Icon, Home, Newspaper, UserGroup, ClipboardDocumentList, BookOpen, Squares2x2, ChatBubbleLeftRight, DocumentText, AcademicCap, Bell, RectangleStack, ChartBar, Cog6Tooth, CalendarDays } from 'svelte-hero-icons';
	
	async function getAPIData(url: string, parameters: Map<string, string>) { // Send a GET request to a url with parameters
		// Takes a url string and a map (dictionary) of key value pairs
		let message = await invoke('get_api_data', {url: url, parameters: Object.fromEntries(parameters)}).then((message) => {return message});
		return await message;
	}

	async function postAPIData(url: string, data: Map<string, string>) { // Send a POST request to a URL with data in the form of JSON
		// Takes a url string and a map (dictionary) of key value pairs
		let message = await invoke('post_api_data', {url: url, data: Object.fromEntries(data)}).then((message) => {return message});
		return await message;
	}

	async function loop() { // test loop to check if Rust API is working, loops every second
		while (true) {

			await new Promise(resolve => setTimeout(resolve, 1000));
			let map1 = new Map<string, string>();
			map1.set("hi", "hi");

			let data = await getAPIData('https://httpbin.org/ip', map1).then((data) => {
				return data
			});

			console.log(data);
		}
	}

	loop();

	// Sidebar menu items
	const menu = [
		{ label: 'Home', icon: Home, path: '/' },
		{ label: 'News', icon: Newspaper, path: '/news' },
		{ label: 'Welcome', icon: UserGroup, path: '/welcome' },
		{ label: 'Assessments', icon: ClipboardDocumentList, hasSub: true, path: '/assessments' },
		{ label: 'Courses', icon: BookOpen, hasSub: true, path: '/courses' },
		{ label: 'Dashboard', icon: Squares2x2, path: '/dashboard' },
		{ label: 'Direqt Messages', icon: ChatBubbleLeftRight, path: '/direqt-messages' },
		{ label: 'Documents', icon: DocumentText, path: '/documents' },
		{ label: 'myEdOnline', icon: AcademicCap, path: '/myedonline' },
		{ label: 'Notices', icon: Bell, path: '/notices' },
		{ label: 'Portals', icon: RectangleStack, hasSub: true, path: '/portals' },
		{ label: 'Reports', icon: ChartBar, path: '/reports' },
		{ label: 'Settings', icon: Cog6Tooth, path: '/settings' },
		{ label: 'Timetable', icon: CalendarDays, path: '/timetable' }
	];
</script>

<div class="min-h-screen" style="background: var(--background); color: var(--text);">

	<!-- Top Bar -->
	<header class="fixed top-0 left-0 right-0 h-12" style="background: var(--surface); color: var(--text);">
		<span class="font-bold text-lg tracking-wide px-8">TauriSEQTA</span>
	</header>
	<div class="flex pt-12 min-h-screen">
		<!-- Sidebar -->
		<aside class="w-64 flex flex-col py-6 px-2 space-y-2 min-h-screen" style="background: var(--surface); color: var(--text);">
			{#each menu as item}
				<a href={item.path} class="flex items-center px-4 py-3 rounded hover:bg-[color:var(--surface-alt)] transition group">
					<Icon src={item.icon} class="mr-4 w-6 h-6" />
					<span class="font-bold text-base">{item.label}</span>
					{#if item.hasSub}
						<svg class="ml-auto w-4 h-4" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7"/></svg>
					{/if}
				</a>
			{/each}
		</aside>
		<!-- Main Content -->
		<div class="flex-1 p-8" style="background: var(--background); color: var(--text);">
			<main class="max-w-7xl mx-auto">
				<slot />
			</main>
		</div>
	</div>
</div> 