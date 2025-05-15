<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	
	import { seqtaFetch } from '../utils/seqtaFetch';

	const studentId = 69; // literally changes nothing but was used in the original seqta code.
  
	let currentSelectedDate: Date = new Date();
  
	let lessons = $state<any[]>([]);
	let lessonColours = $state<any[]>([]);
	let upcomingAssessments = $state<any[]>([]);
	let activeSubjects = $state<any[]>([]);
	let notices = $state<any[]>([]);
  
	let subjectFilters: Record<string, boolean> = {};
  
	let loadingLessons = $state<boolean>(true);
	let loadingAssessments = $state<boolean>(true);
	let loadingNotices = $state<boolean>(true);

	const filteredAssessments = $derived(upcomingAssessments.filter((a: any) => subjectFilters[a.code]));
  
	let lessonInterval: ReturnType<typeof setInterval> | null = null;
    
	function formatDate(date: Date): string {
	  const y = date.getFullYear();
	  const m = (date.getMonth() + 1).toString().padStart(2, '0');
	  const d = date.getDate().toString().padStart(2, '0');
	  return `${y}-${m}-${d}`;
	}
  
	async function loadLessonColours() {
	  if (lessonColours.length) return lessonColours;
	  const res = await seqtaFetch('/seqta/student/load/prefs?', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json; charset=utf-8' },
		body: { request: 'userPrefs', asArray: true, user: studentId }
	  });
	  lessonColours = JSON.parse(res).payload;
	  return lessonColours;
	}
  
	async function loadLessons() {
	  loadingLessons = true;
	  const dateStr = formatDate(currentSelectedDate);
  
	  const res = await seqtaFetch('/seqta/student/load/timetable?', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: { from: dateStr, until: dateStr, student: studentId }
	  });
  
	  const colours = await loadLessonColours();
  
	  lessons = JSON.parse(res).payload.items
		.sort((a: any, b: any) => a.from.localeCompare(b.from))
		.map((lesson: any) => {
		  const colourPrefName = `timetable.subject.colour.${lesson.code}`;
		  const subjectColour = colours.find((c: any) => c.name === colourPrefName);
  
		  lesson.colour = subjectColour ? `${subjectColour.value}` : `transparent`;
  
		  lesson.from = lesson.from.substring(0, 5);
		  lesson.until = lesson.until.substring(0, 5);
    
		  lesson.attendanceTitle = lesson.attendance ? lesson.attendance.label : ' ';
		  return lesson;
		});
  
	  loadingLessons = false;
  
	  // start / reset live updater
	  if (lessonInterval) clearInterval(lessonInterval);
	  checkCurrentLessons();
	  lessonInterval = setInterval(checkCurrentLessons, 60_000);
	}
  
	function checkCurrentLessons() {
	  const now = new Date();
	  lessons = lessons.map((l: any) => {
		const [sh, sm] = l.from.split(':').map(Number);
		const [eh, em] = l.until.split(':').map(Number);
  
		const start = new Date(currentSelectedDate);
		start.setHours(sh, sm, 0, 0);
		const end = new Date(currentSelectedDate);
		end.setHours(eh, em, 0, 0);
  
		l.active = now >= start && now <= end && now.toDateString() === currentSelectedDate.toDateString();
		return l;
	  });
	}
  
	async function loadAssessments() {
	  loadingAssessments = true;
  
	  const [assessmentsRes, classesRes] = await Promise.all([
		seqtaFetch('/seqta/student/assessment/list/upcoming?', {
		  method: 'POST',
		  headers: { 'Content-Type': 'application/json; charset=utf-8' },
		  body: { student: studentId }
		}),
		seqtaFetch('/seqta/student/load/subjects?', {
		  method: 'POST',
		  headers: { 'Content-Type': 'application/json; charset=utf-8' },
		  body: {}
		})
	  ]);
  
	  const colours = await loadLessonColours();
  
	  const classesResJson = JSON.parse(classesRes);
	  const activeClass = classesResJson.payload.find((c: any) => c.active);
	  activeSubjects = activeClass ? activeClass.subjects : [];
  
	  // Initialize subject filters on first run
	  activeSubjects.forEach((s: any) => {
		if (!(s.code in subjectFilters)) subjectFilters[s.code] = true;
	  });
  
	  const activeCodes = activeSubjects.map((s: any) => s.code);
  
	  upcomingAssessments = JSON.parse(assessmentsRes).payload
		.filter((a: any) => activeCodes.includes(a.code))
		.filter((a: any) => new Date(a.due) >= new Date())
		.map((a: any) => {
		  const prefName = `timetable.subject.colour.${a.code}`;
		  const c = colours.find((p: any) => p.name === prefName);
		  a.colour = c ? c.value : '#8e8e8e';
		  return a;
		})
		.sort((a: any, b: any) => (a.due < b.due ? -1 : 1));
  
	  loadingAssessments = false;
	}
  
	async function loadNotices(dateStr: string) {
	  loadingNotices = true;
  
	  const prefsRes = await seqtaFetch('/seqta/student/load/prefs?', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: { asArray: true, request: 'userPrefs' }
	  });

	  const prefsResJson = JSON.parse(prefsRes);
	  const filters = prefsResJson.payload.find((p: any) => p.name === 'notices.filters');
	  const labelArray = filters ? filters.value.split(' ') : [];
  
	  const res = await seqtaFetch('/seqta/student/load/notices?', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: { date: dateStr }
	  });
  
	  notices = res.payload?.filter((n: any) => labelArray.includes(JSON.stringify(n.label)));
	  loadingNotices = false;
	}
    
	function prevDay() {
	  currentSelectedDate = new Date(currentSelectedDate.valueOf() - 86_400_000);
	  loadLessons();
	}
  
	function nextDay() {
	  currentSelectedDate = new Date(currentSelectedDate.valueOf() + 86_400_000);
	  loadLessons();
	}
  
	function buildAssessmentURL(programmeID: number, metaID: number, itemID?: number) {
	  const base = `../#?page=/assessments/${programmeID}:${metaID}`;
	  return itemID ? `${base}&item=${itemID}` : base;
	}

	function getStatusBadge(status: string, due: string) {
		const dueDate = new Date(due);
		const now = new Date();
		
		if (status === 'MARKS_RELEASED') {
			return { text: 'Marked', color: 'bg-green-500' };
		} else if (dueDate < now) {
			return { text: 'Overdue', color: 'bg-red-500' };
		} else if (dueDate.getTime() - now.getTime() < 7 * 24 * 60 * 60 * 1000) { // Within 7 days
			return { text: 'Due Soon', color: 'bg-yellow-500' };
		} else {
			return { text: 'Upcoming', color: 'bg-blue-500' };
		}
	}

	onMount(async () => {
	  await Promise.all([loadLessons(), loadAssessments(), loadNotices(formatDate(new Date()))]);
	});
  
	onDestroy(() => {
	  if (lessonInterval) clearInterval(lessonInterval);
	});
    
	function lessonsSubtitle() {
	  const today = new Date();
	  const diff = ~~((today.getTime() - currentSelectedDate.getTime()) / 86_400_000);
	  if (diff === 0) return "Today's Lessons";
	  if (diff === -1) return "Tomorrow's Lessons";
	  if (diff === 1) return "Yesterday's Lessons";
	  return currentSelectedDate.toLocaleDateString('en-AU', { weekday: 'short', year: 'numeric', month: 'numeric', day: 'numeric' });
	}
  </script>
<div class="p-8 mx-auto max-w-7xl">
	<div class="space-y-6">
		<div class="flex gap-6">
			{#each [{ name: 'Outlook', icon: '📅', url: 'https://outlook.office.com' }, { name: 'Office365', icon: '🏢', url: 'https://office365.com' }, { name: 'Google', icon: '🌐', url: 'https://google.com' }] as integration}
			<a href={integration.url} target="_blank" class="flex flex-1 justify-center items-center py-4 text-lg font-semibold rounded-2xl shadow transition-transform duration-300 hover:scale-105" style="background: var(--surface); color: var(--text);">
				<span class="mr-2 text-2xl">{integration.icon}</span> {integration.name}
			</a>
			{/each}
		</div>
		
		<div class="rounded-2xl shadow bg-[var(--surface)]">
			<div class="flex justify-between items-center px-6 py-3 rounded-t-2xl border-b" style="border-color: var(--surface-alt);">
			<span class="text-lg font-semibold" style="color: var(--text);">{lessonsSubtitle()}</span>
			<div class="flex gap-2">
				<button onclick={prevDay} class="w-8 h-8 flex items-center justify-center rounded-full hover:bg-[color:var(--surface-alt)] transition-transform duration-300 hover:scale-110" style="color: var(--text);">&#60;</button>
				<button onclick={nextDay} class="w-8 h-8 flex items-center justify-center rounded-full hover:bg-[color:var(--surface-alt)] transition-transform duration-300 hover:scale-110" style="color: var(--text);">&#62;</button>
			</div>
			</div>
		
			{#if loadingLessons}
			<div class="flex flex-col justify-center items-center py-12"><p style="color:var(--text-muted);">Loading lessons…</p></div>
			{:else if lessons.length === 0}
			<div class="flex flex-col justify-center items-center py-12">
				<div class="flex justify-center items-center mb-4 w-24 h-24 rounded-full" style="background: var(--background); color: var(--surface);">
				<span class="text-6xl">Q</span>
				</div>
				<p class="text-2xl" style="color: var(--text);">No lessons available.</p>
			</div>
			{:else}
			<div class="flex overflow-x-auto gap-4 px-4 py-6">
				{#each lessons as lesson, i}
				<div class="relative flex flex-col min-w-[170px] max-w-[210px] bg-[var(--surface-alt)] rounded-xl shadow-lg border-l-8 transition-transform duration-300 hover:scale-105" style="border-color: {lesson.colour};">
					<div class="flex flex-col flex-1 gap-1 px-4 pt-4 pb-2">
					<div class="flex justify-between items-center">
						<span class="text-base font-bold" style="color: var(--text);">{lesson.description}</span>
						{#if lesson.active}
						<span class="px-2 py-0.5 ml-2 text-xs text-white bg-green-600 rounded">Now</span>
						{/if}
					</div>
					<div class="text-sm" style="color: var(--text-muted);">{lesson.staff}</div>
					<div class="text-xs" style="color: var(--text-muted);">{lesson.room}</div>
					<div class="mt-2 font-mono text-sm" style="color: var(--text);">{lesson.from} – {lesson.until}</div>
					{#if lesson.attendanceTitle && lesson.attendanceTitle.trim()}
						<div class="mt-1 text-xs italic" style="color: var(--text-muted);">{lesson.attendanceTitle}</div>
					{/if}
					</div>
					{#if lesson.programmeID !== 0}
					<div class="flex gap-2 px-4 pt-1 pb-3">
						<button class="transition-transform duration-300 hover:scale-110" aria-label="View Assessment" onclick={() => (location.href = buildAssessmentURL(lesson.programmeID, lesson.metaID))}>
						<svg viewBox="0 0 24 24" style="width:22px;height:22px;"><path d="M6 20H13V22H6C4.89 22 4 21.11 4 20V4C4 2.9 4.89 2 6 2H18C19.11 2 20 2.9 20 4V12.54L18.5 11.72L18 12V4H13V12L10.5 9.75L8 12V4H6V20M24 17L18.5 14L13 17L18.5 20L24 17M15 19.09V21.09L18.5 23L22 21.09V19.09L18.5 21L15 19.09Z" fill="currentColor"></path></svg>
						</button>
						<button class="transition-transform duration-300 hover:scale-110" aria-label="View Course" onclick={() => (location.href = `../#?page=/courses/${lesson.programmeID}:${lesson.metaID}`)}>
						<svg viewBox="0 0 24 24" style="width:22px;height:22px;"><path d="M19 1L14 6V17L19 12.5V1M21 5V18.5C19.9 18.15 18.7 18 17.5 18C15.8 18 13.35 18.65 12 19.5V6C10.55 4.9 8.45 4.5 6.5 4.5C4.55 4.5 2.45 4.9 1 6V20.65C1 20.9 1.25 21.15 1.5 21.15C1.6 21.15 1.65 21.1 1.75 21.1C3.1 20.45 5.05 20 6.5 20C8.45 20 10.55 20.4 12 21.5C13.35 20.65 15.8 20 17.5 20C19.15 20 20.85 20.3 22.25 21.05C22.35 21.1 22.4 21.1 22.5 21.1C22.75 21.1 23 20.85 23 20.6V6C22.4 5.55 21.75 5.25 21 5M10 18.41C8.75 18.09 7.5 18 6.5 18C5.44 18 4.18 18.19 3 18.5V7.13C3.91 6.73 5.14 6.5 6.5 6.5C7.86 6.5 9.09 6.73 10 7.13V18.41Z" fill="currentColor"></path></svg>
						</button>
					</div>
					{/if}
				</div>
				{/each}
			</div>
			{/if}
		</div>
		
		<div class="rounded-2xl shadow bg-[var(--surface)]">
				<div class="flex justify-between items-center px-6 py-3 rounded-t-2xl border-b" style="border-color: var(--surface-alt);">
			<span class="text-lg font-semibold" style="color: var(--text);">Upcoming Assessments</span>
			<div class="flex flex-wrap gap-2" id="upcoming-filters">
				{#each activeSubjects as subj}
				<label class="upcoming-checkbox-container" style={subj.colour}>
					<input type="checkbox" bind:checked={subjectFilters[subj.code]} />
					<span class="upcoming-checkmark"></span>
					{subj.code}
				</label>
				{/each}
			</div>
			</div>
		
			{#if loadingAssessments}
			<div class="flex flex-col justify-center items-center py-12"><p style="color:var(--text-muted);">Loading assessments…</p></div>
			{:else if filteredAssessments.length === 0}
			<div class="flex flex-col justify-center items-center py-12"><p style="color:var(--text);">Nothing coming up 🎉</p></div>
			{:else}
			<div class="p-6 space-y-4" id="upcoming-items">
				{#each filteredAssessments as a}
				<div class="flex gap-4 items-center p-4 rounded-xl transition-transform duration-300 upcoming-assessment hover:scale-105" style="background: var(--surface-alt); border: 1px solid var(--surface-alt); border-left: 8px solid {a.colour};">
					<div class="flex justify-center items-center w-16 h-16 rounded-lg" style="background: var(--background); color: var(--surface);">
					<span class="text-3xl">📄</span>
					</div>
					<div class="flex-1">
					<div class="flex gap-2 items-center">
						<div class="text-lg font-bold" style="color: var(--text);">
						{new Date(a.due).toLocaleDateString('en-AU', { weekday: 'short', month: 'short', day: 'numeric' })}
						</div>
						<span class="px-2 py-0.5 rounded text-xs text-white {getStatusBadge(a.status, a.due).color}">
						{getStatusBadge(a.status, a.due).text}
						</span>
					</div>
					<div class="mt-2">
						<span class="block text-xs font-semibold uppercase" style="color: var(--text-muted);">{a.subject}</span>
						<span class="block text-base font-semibold" style="color: var(--text);">{a.title}</span>
					</div>
					</div>
				</div>
				{/each}
			</div>
			{/if}
		</div>
	</div>
</div>