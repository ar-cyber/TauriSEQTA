<script lang="ts">
	import { onMount } from 'svelte';
	import { seqtaFetch } from '../../utils/seqtaFetch';

	const studentId = 69;
  
	let upcomingAssessments = $state<any[]>([]);
	let activeSubjects = $state<any[]>([]);
	let lessonColours = $state<any[]>([]);
	let loadingAssessments = $state<boolean>(true);
	let selectedTab = $state<'list' | 'board'>('list');
	let subjectFilters: Record<string, boolean> = {};

	const filteredAssessments = $derived(upcomingAssessments.filter((a: any) => subjectFilters[a.code]));
  
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
			.map((a: any) => {
				const prefName = `timetable.subject.colour.${a.code}`;
				const c = colours.find((p: any) => p.name === prefName);
				a.colour = c ? c.value : '#8e8e8e';
				// Get the metaclass from the subject
				const subject = activeSubjects.find((s: any) => s.code === a.code);
				a.metaclass = subject?.metaclass;
				return a;
			})
			.sort((a: any, b: any) => new Date(b.due).getTime() - new Date(a.due).getTime());

		loadingAssessments = false;
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

	function scrollToSubject(event: MouseEvent, subjectCode: string) {
		event.preventDefault();
		const element = document.getElementById(`subject-${subjectCode}`);
		if (element) {
			element.scrollIntoView({ behavior: 'smooth', block: 'start' });
			// Add highlight class
			element.classList.add('highlight-subject');
			// Remove highlight class after animation
			setTimeout(() => {
				element.classList.remove('highlight-subject');
			}, 1500);
		}
	}

	onMount(loadAssessments);
</script>

<div class="p-6 space-y-6">
	<div class="flex justify-between items-center">
		<h1 class="text-2xl font-bold" style="color: var(--text);">Assessments</h1>
		<div class="flex gap-2">
			<button 
				class="px-4 py-2 rounded-lg transition-colors {selectedTab === 'list' ? 'bg-blue-500 text-white' : 'bg-slate-800 text-slate-50'}"
				onclick={() => selectedTab = 'list'}
			>
				List View
			</button>
			<button 
				class="px-4 py-2 rounded-lg transition-colors {selectedTab === 'board' ? 'bg-blue-500 text-white' : 'bg-slate-800 text-slate-50'}"
				onclick={() => selectedTab = 'board'}
			>
				Board View
			</button>
		</div>
	</div>

	{#if loadingAssessments}
		<div class="flex justify-center items-center py-12">
			<p style="color: var(--text-muted);">Loading assessments...</p>
		</div>
	{:else if filteredAssessments.length === 0}
		<div class="flex justify-center items-center py-12">
			<p style="color: var(--text);">No upcoming assessments 🎉</p>
		</div>
	{:else if selectedTab === 'list'}
		<div class="flex gap-6">
			<!-- Quick Navigation Sidebar -->
			<div class="w-48 flex-shrink-0">
				<div class="sticky top-6 bg-slate-800 rounded-xl p-4">
					<h3 class="text-sm font-semibold mb-3" style="color: var(--text-muted);">Quick Jump</h3>
					<div class="space-y-2">
						{#each activeSubjects.filter(s => subjectFilters[s.code]) as subject}
							<a 
								href="#subject-{subject.code}"
								class="flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-slate-900 transition-colors cursor-pointer"
								style="color: var(--text);"
								onclick={(e) => scrollToSubject(e, subject.code)}
							>
								<div class="w-2 h-2 rounded-full" style="background-color: {subject.colour || '#8e8e8e'}"></div>
								<span class="text-sm truncate">{subject.code}</span>
							</a>
						{/each}
					</div>
				</div>
			</div>

			<!-- Main Content -->
			<div class="flex-1 space-y-6">
				{#each activeSubjects.filter(s => subjectFilters[s.code]) as subject}
					<div id="subject-{subject.code}" class="bg-slate-800 rounded-xl overflow-hidden">
						<div class="px-6 py-4 border-b" style="border-color: var(--surface);">
							<div class="flex items-center gap-3">
								<div class="w-3 h-3 rounded-full" style="background-color: {subject.colour || '#8e8e8e'}"></div>
								<h3 class="font-bold text-lg" style="color: var(--text);">{subject.title}</h3>
								<span class="text-sm" style="color: var(--text-muted);">({subject.code})</span>
							</div>
						</div>
						<div class="p-4 space-y-4">
							{#each filteredAssessments.filter(a => a.code === subject.code) as assessment}
								<div class="flex gap-4 items-center p-4 rounded-xl bg-slate-900 transition-transform duration-300 hover:scale-105 border-l-8" style="border-color: {assessment.colour};">
									<div class="flex justify-center items-center w-16 h-16 rounded-lg" style="background: var(--background); color: var(--surface);">
										<span class="text-3xl">📄</span>
									</div>
									<div class="flex-1">
										<div class="flex items-center gap-2">
											<div class="text-lg font-bold" style="color: var(--text);">
												{new Date(assessment.due).toLocaleDateString('en-AU', { weekday: 'short', month: 'short', day: 'numeric', year: 'numeric' })}
											</div>
											<span class="px-2 py-0.5 rounded text-xs text-white {getStatusBadge(assessment.status, assessment.due).color}">
												{getStatusBadge(assessment.status, assessment.due).text}
											</span>
										</div>
										<div class="mt-2">
											<span class="block text-base font-semibold" style="color: var(--text);">{assessment.title}</span>
										</div>
									</div>
									<div class="flex gap-2">
										<a
											href="/assessments/{assessment.id}/{assessment.metaclass}"
											class="px-3 py-1 text-sm font-medium rounded-lg bg-blue-600 hover:bg-blue-500 transition-colors"
										>
											View Details
										</a>
									</div>
								</div>
							{/each}
						</div>
					</div>
				{/each}
			</div>
		</div>
	{:else}
		<div class="flex gap-4 overflow-x-auto pb-4">
			{#each activeSubjects.filter(s => subjectFilters[s.code]) as subject}
				<div class="flex-shrink-0 w-80">
					<div class="bg-slate-800 rounded-xl p-4 mb-4 border-l-8" style="border-color: {subject.colour || '#8e8e8e'};">
						<h3 class="font-bold text-lg" style="color: var(--text);">{subject.title}</h3>
						<p class="text-sm" style="color: var(--text-muted);">{subject.code}</p>
					</div>
					<div class="space-y-4">
						{#each filteredAssessments.filter(a => a.code === subject.code) as assessment}
							<div class="bg-slate-900 rounded-xl p-4 shadow-lg border-l-8" style="border-color: {assessment.colour};">
								<div class="flex items-center gap-2">
									<div class="text-sm font-semibold" style="color: var(--text-muted);">
										{new Date(assessment.due).toLocaleDateString('en-AU', { weekday: 'short', month: 'short', day: 'numeric', year: 'numeric' })}
									</div>
									<span class="px-2 py-0.5 rounded text-xs text-white {getStatusBadge(assessment.status, assessment.due).color}">
										{getStatusBadge(assessment.status, assessment.due).text}
									</span>
								</div>
								<h4 class="font-bold mt-1" style="color: var(--text);">{assessment.title}</h4>
								<div class="flex gap-2">
									<a
										href="/assessments/{assessment.id}/{assessment.metaclass}"
										class="px-3 py-1 text-sm font-medium rounded-lg bg-blue-600 hover:bg-blue-500 transition-colors"
									>
										View Details
									</a>
								</div>
							</div>
						{/each}
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	/* Add any additional styles here */
	@keyframes highlight {
		0% {
			transform: scale(1);
			box-shadow: 0 0 0 0 rgba(59, 130, 246, 0);
		}
		50% {
			transform: scale(1.02);
			box-shadow: 0 0 0 10px rgba(59, 130, 246, 0.2);
		}
		100% {
			transform: scale(1);
			box-shadow: 0 0 0 0 rgba(59, 130, 246, 0);
		}
	}

	.highlight-subject {
		animation: highlight 1.5s ease-out;
	}
</style> 