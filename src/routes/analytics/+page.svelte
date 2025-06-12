<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { AnalyticsData, Assessment } from '$lib/types';
	import { seqtaFetch } from '../../utils/netUtil';
	import { Icon, ChevronDown, ChevronRight } from 'svelte-hero-icons';
	import { fade, slide, scale } from 'svelte/transition';

	let analyticsData: AnalyticsData | null = null;
	let loading = true;
	let error: string | null = null;
	let showGrabData = false;

	// Graph dimensions
	const width = 800;
	const height = 400;
	const padding = 40;
	const barWidth = 60;
	const barSpacing = 20;

	let barPaths: { path: string; count: number; status: string }[] = [];
	let yLabels: string[] = [];

	const studentId = 69;

	let expandedSubjects: Record<string, boolean> = {};

	function isValidDate(dateStr: string): boolean {
		const date = new Date(dateStr);
		return date instanceof Date && !isNaN(date.getTime());
	}

	function parseAssessment(data: any): Assessment | null {
		try {
			if (!data || typeof data !== 'object') return null;
			
			const assessment: Assessment = {
				id: Number(data.id),
				title: String(data.title || ''),
				subject: String(data.subject || ''),
				status: String(data.status || 'PENDING') as 'OVERDUE' | 'MARKS_RELEASED' | 'PENDING',
				due: String(data.due || ''),
				code: String(data.code || ''),
				metaclassID: Number(data.metaclassID),
				programmeID: Number(data.programmeID),
				graded: Boolean(data.graded),
				overdue: Boolean(data.overdue),
				hasFeedback: Boolean(data.hasFeedback),
				expectationsEnabled: Boolean(data.expectationsEnabled),
				expectationsCompleted: Boolean(data.expectationsCompleted),
				reflectionsEnabled: Boolean(data.reflectionsEnabled),
				reflectionsCompleted: Boolean(data.reflectionsCompleted),
				availability: String(data.availability || ''),
				finalGrade: data.finalGrade ? Number(data.finalGrade) : undefined
			};

			// Validate required fields
			if (!assessment.id || !assessment.title || !assessment.subject || !isValidDate(assessment.due)) {
				return null;
			}

			return assessment;
		} catch (e) {
			console.error('Error parsing assessment:', e);
			return null;
		}
	}

	async function deleteAnalytics() {
		const confirmed = window.confirm('Are you sure you want to delete all analytics data?');
		if (!confirmed) return;
		try {
			await invoke('delete_analytics');
			analyticsData = null;
			showGrabData = true;
		} catch (e) {
			error = 'Failed to delete analytics data';
		}
	}

	async function grabData() {
		loading = true;
		error = null;
		try {
			// Fetch all folders and all subjects (not just active)
			const classesRes = await seqtaFetch('/seqta/student/load/subjects?', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json; charset=utf-8' },
				body: {}
			});
			const data = JSON.parse(classesRes);
			const folders = data.payload;
			const allSubjects = folders.flatMap((f: any) => f.subjects);

			// Remove duplicate subjects by programme+metaclass
			const uniqueSubjectsMap = new Map();
			allSubjects.forEach((s: any) => {
				const key = `${s.programme}-${s.metaclass}`;
				if (!uniqueSubjectsMap.has(key)) uniqueSubjectsMap.set(key, s);
			});
			const uniqueSubjects = Array.from(uniqueSubjectsMap.values());

			// Fetch upcoming assessments for current active subjects (optional, can be skipped if you want only past)
			const activeFolder = folders.find((f: any) => f.active === 1);
			const activeSubjects = activeFolder ? activeFolder.subjects : [];
			const assessmentsRes = await seqtaFetch('/seqta/student/assessment/list/upcoming?', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json; charset=utf-8' },
				body: { student: studentId }
			});
			const upcomingAssessments = JSON.parse(assessmentsRes).payload;

			// Fetch past assessments for every subject ever
			const pastAssessmentsPromises = uniqueSubjects.map((subject: any) =>
				seqtaFetch('/seqta/student/assessment/list/past?', {
					method: 'POST',
					headers: { 'Content-Type': 'application/json; charset=utf-8' },
					body: {
						programme: subject.programme,
						metaclass: subject.metaclass,
						student: studentId
					}
				})
			);
			const pastAssessmentsResponses = await Promise.all(pastAssessmentsPromises);
			const pastAssessments = pastAssessmentsResponses
				.map(res => JSON.parse(res).payload.tasks || [])
				.flat();

			// Combine and process all assessments
			const allAssessments = [
				...upcomingAssessments,
				...pastAssessments
			];

			// Remove duplicates by id
			const uniqueAssessmentsMap = new Map();
			allAssessments.forEach((a: any) => {
				if (!uniqueAssessmentsMap.has(a.id)) {
					uniqueAssessmentsMap.set(a.id, a);
				}
			});
			const uniqueAssessments = Array.from(uniqueAssessmentsMap.values());

			// Add finalGrade if marks are released
			const processedAssessments = uniqueAssessments.map((a: any) => {
				let finalGrade = undefined;
				if (a.status === 'MARKS_RELEASED') {
					if (a.criteria && a.criteria[0]?.results?.percentage !== undefined) {
						finalGrade = a.criteria[0].results.percentage;
					} else if (a.results && a.results.percentage !== undefined) {
						finalGrade = a.results.percentage;
					}
				}
				return { ...a, finalGrade };
			});

			// Save to analytics.json via Tauri
			await invoke('save_analytics', { data: JSON.stringify(processedAssessments) });
			window.location.reload();
		} catch (e) {
			error = 'Failed to grab and save analytics data.';
			loading = false;
		}
	}

	onMount(async () => {
		try {
			console.log('Fetching analytics data...');
			const response = await invoke<string>('load_analytics');
			console.log('Received raw data:', response);
			const parsedData = JSON.parse(response);
			const rawAssessments = Array.isArray(parsedData) ? parsedData : Object.values(parsedData);
			const validAssessments = rawAssessments
				.map(parseAssessment)
				.filter((assessment): assessment is Assessment => assessment !== null);
			console.log('Valid assessments:', validAssessments);
			analyticsData = validAssessments;
			showGrabData = false;
			processData();
		} catch (e) {
			console.error('Error loading analytics:', e);
			error = e instanceof Error ? e.message : 'Failed to load analytics data';
			showGrabData = true;
		} finally {
			loading = false;
		}
	});

	function processData() {
		console.log('Processing data:', analyticsData);
		if (!analyticsData || !Array.isArray(analyticsData) || analyticsData.length === 0) {
			console.log('No assessments data available');
			return;
		}

		// Filter for assessments with grades and group them into ranges
		const gradedAssessments = analyticsData.filter(a => a.finalGrade !== undefined);
		const gradeRanges = {
			'90-100': 0,
			'80-89': 0,
			'70-79': 0,
			'60-69': 0,
			'50-59': 0,
			'0-49': 0
		};

		gradedAssessments.forEach(assessment => {
			const grade = assessment.finalGrade!;
			if (grade >= 90) gradeRanges['90-100']++;
			else if (grade >= 80) gradeRanges['80-89']++;
			else if (grade >= 70) gradeRanges['70-79']++;
			else if (grade >= 60) gradeRanges['60-69']++;
			else if (grade >= 50) gradeRanges['50-59']++;
			else gradeRanges['0-49']++;
		});

		const maxCount = Math.max(...Object.values(gradeRanges), 1);
		const yScale = (height - padding * 2) / maxCount;

		// Generate y-axis labels at intervals of 10
		const yStep = Math.max(10, Math.ceil(maxCount / 10) * 10 / 10); // At least 10, or enough to keep ~10 labels
		yLabels = [];
		for (let i = 0; i <= maxCount; i += yStep) {
			yLabels.push(i.toString());
		}
		if (yLabels[yLabels.length - 1] !== maxCount.toString()) {
			yLabels.push(maxCount.toString());
		}

		// Generate bar paths
		barPaths = Object.entries(gradeRanges).map(([range, count], index) => {
			const x = padding + index * (barWidth + barSpacing);
			const barHeight = count * yScale;
			const path = `M ${x} ${height - padding} h ${barWidth} v -${barHeight} h -${barWidth} Z`;
			return { path, count, status: range };
		});
	}

	function getStatusColor(status: string): string {
		const grade = parseInt(status.split('-')[0]);
		if (grade >= 90) return 'rgb(34, 197, 94)'; // green
		if (grade >= 80) return 'rgb(34, 197, 94)'; // green
		if (grade >= 70) return 'rgb(234, 179, 8)'; // yellow
		if (grade >= 60) return 'rgb(234, 179, 8)'; // yellow
		if (grade >= 50) return 'rgb(239, 68, 68)'; // red
		return 'rgb(239, 68, 68)'; // red
	}

	function getLetterGrade(percentage: number | undefined): string {
		if (percentage === undefined) return '';
		if (percentage >= 90) return 'A+';
		if (percentage >= 85) return 'A';
		if (percentage >= 80) return 'A-';
		if (percentage >= 75) return 'B+';
		if (percentage >= 70) return 'B';
		if (percentage >= 65) return 'B-';
		if (percentage >= 60) return 'C+';
		if (percentage >= 55) return 'C';
		if (percentage >= 50) return 'C-';
		if (percentage >= 40) return 'D';
		return 'E';
	}

	function toggleSubject(subject: string) {
		expandedSubjects[subject] = !expandedSubjects[subject];
	}

	// Helper function to group by subject
	function groupBySubject(data: AnalyticsData | null): Record<string, Assessment[]> {
		const grouped: Record<string, Assessment[]> = {};
		if (!data) return grouped;
		for (const a of data) {
			if (!grouped[a.subject]) grouped[a.subject] = [];
			grouped[a.subject].push(a);
		}
		return grouped;
	}
</script>

<div class="container mx-auto px-4 py-8">
	<h1 class="text-3xl font-bold mb-8">Analytics Dashboard</h1>

	{#if loading}
		<div class="flex items-center justify-center h-64">
			<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600"></div>
		</div>
	{:else if showGrabData}
		<div class="flex flex-col items-center justify-center h-64 gap-4">
			<button class="px-6 py-3 bg-indigo-600 text-white rounded-lg shadow transition-all duration-200 transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 accent-ring hover:bg-indigo-700" on:click={grabData}>
				Grab Data
			</button>
			{#if error}
				<div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg">{error}</div>
			{/if}
		</div>
	{:else if analyticsData}
		<div class="flex justify-end mb-4">
			<button class="px-4 py-2 bg-red-600 text-white rounded transition-all duration-200 transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 accent-ring hover:bg-red-700" on:click={deleteAnalytics}>
				Delete Data
			</button>
		</div>
		<div class="bg-white/80 dark:bg-slate-900/80 rounded-2xl shadow-xl p-8 mb-8 border border-gray-200 dark:border-slate-700" in:fade={{ duration: 400 }}>
			<h2 class="text-2xl font-bold mb-6 text-gray-900 dark:text-white flex items-center gap-2">
				<span class="inline-block w-6 h-6 bg-gradient-to-tr from-indigo-500 to-blue-400 rounded-full flex items-center justify-center text-white shadow">
					<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17v-6a2 2 0 012-2h2a2 2 0 012 2v6m-6 0h6"/></svg>
				</span>
				Grade Distribution
			</h2>
			<div class="overflow-x-auto">
				<svg {width} {height} class="min-w-[800px]">
					<!-- Grid lines -->
					{#each yLabels as _, i}
						<line
							x1={padding}
							y1={height - padding - (i * (height - padding * 2) / (yLabels.length - 1))}
							x2={width - padding}
							y2={height - padding - (i * (height - padding * 2) / (yLabels.length - 1))}
							class="stroke-gray-200 dark:stroke-gray-700"
							stroke-width="1"
						/>
					{/each}

					<!-- Y-axis labels -->
					{#each yLabels as label, i}
						<text
							x={padding - 10}
							y={height - padding - (i * (height - padding * 2) / (yLabels.length - 1))}
							class="fill-gray-500 dark:fill-gray-400 text-xs"
							text-anchor="end"
							dominant-baseline="middle"
						>
							{label}
						</text>
					{/each}

					<!-- Bars -->
					{#each barPaths as { path, count, status }, i}
						<g class="transition-all duration-300 hover:opacity-80" in:scale={{ duration: 350, delay: i * 60 }}>
							<path
								d={path}
								fill={getStatusColor(status)}
								class="transition-all duration-300"
							/>
							<text
								x={padding + i * (barWidth + barSpacing) + barWidth / 2}
								y={height - padding + 20}
								class="fill-gray-500 dark:fill-gray-400 text-xs"
								text-anchor="middle"
							>
								{status}% {getLetterGrade((() => { const [min] = status.split('-'); return Number(min); })())}
							</text>
							<text
								x={padding + i * (barWidth + barSpacing) + barWidth / 2}
								y={height - padding - count * ((height - padding * 2) / Math.max(...yLabels.map(Number), 1)) - 10}
								class="fill-gray-900 dark:fill-gray-100 text-sm font-medium"
								text-anchor="middle"
							>
								{count}
							</text>
						</g>
					{/each}
				</svg>
			</div>
		</div>

		<div class="bg-white/80 dark:bg-slate-900/80 rounded-2xl shadow-xl p-8 border border-gray-200 dark:border-slate-700" in:fade={{ duration: 400, delay: 100 }}>
			<h2 class="text-2xl font-bold mb-6 text-gray-900 dark:text-white flex items-center gap-2">
				<span class="inline-block w-6 h-6 bg-gradient-to-tr from-indigo-500 to-blue-400 rounded-full flex items-center justify-center text-white shadow">
					<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/></svg>
				</span>
				Raw Data
			</h2>
			<div class="overflow-x-auto">
				{#each Object.entries(groupBySubject(analyticsData)) as [subject, assessments]}
					<div class="mb-4 border border-gray-200 dark:border-slate-700 rounded-xl overflow-hidden bg-gray-50/80 dark:bg-slate-800/80" in:slide={{ duration: 350 }}>
						<button class="w-full flex items-center justify-between px-6 py-3 bg-gray-100 dark:bg-slate-700 hover:bg-gray-200 dark:hover:bg-slate-600 transition-all duration-200 transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 accent-ring font-semibold text-left text-lg" on:click={() => toggleSubject(subject)}>
							<span class="flex items-center gap-2">
								{#if expandedSubjects[subject]}
									<Icon src={ChevronDown} class="w-5 h-5 text-indigo-500" />
								{:else}
									<Icon src={ChevronRight} class="w-5 h-5 text-indigo-500" />
								{/if}
								<span>{subject}</span>
							</span>
						</button>
						{#if expandedSubjects[subject]}
							<div transition:fade={{ duration: 250 }}>
								<table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
									<thead>
										<tr>
											<th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Title</th>
											<th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Grade</th>
											<th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Due Date</th>
										</tr>
									</thead>
									<tbody class="divide-y divide-gray-200 dark:divide-gray-700">
										{#each assessments as assessment}
											<tr>
												<td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100">{assessment.title}</td>
												<td class="px-6 py-4 whitespace-nowrap text-sm">
													{#if assessment.finalGrade !== undefined}
														<span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full {assessment.finalGrade >= 80
															? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200'
															: assessment.finalGrade >= 60
															? 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200'
															: 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200'}">
															{assessment.finalGrade}% {getLetterGrade(assessment.finalGrade)}
														</span>
													{:else}
														<span class="text-gray-500">Not graded</span>
													{/if}
												</td>
												<td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100">{new Date(assessment.due).toLocaleDateString()}</td>
											</tr>
										{/each}
									</tbody>
								</table>
							</div>
						{/if}
					</div>
				{/each}
			</div>
		</div>
	{:else}
		<div class="text-center text-gray-500 dark:text-gray-400">
			No analytics data available
		</div>
	{/if}
</div> 