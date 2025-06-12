<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import type { AnalyticsData, Assessment } from '$lib/types';
	import { seqtaFetch } from '../../utils/netUtil';

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
			// Fetch upcoming and past assessments for all active subjects
			const studentId = 69;
			// Fetch subjects
			const classesRes = await seqtaFetch('/seqta/student/load/subjects?', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json; charset=utf-8' },
				body: {}
			});
			const classesResJson = JSON.parse(classesRes);
			const activeClass = classesResJson.payload.find((c: any) => c.active);
			const activeSubjects = activeClass ? activeClass.subjects : [];
			const activeCodes = activeSubjects.map((s: any) => s.code);

			// Fetch upcoming assessments
			const assessmentsRes = await seqtaFetch('/seqta/student/assessment/list/upcoming?', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json; charset=utf-8' },
				body: { student: studentId }
			});
			const upcomingAssessments = JSON.parse(assessmentsRes).payload;

			// Fetch past assessments for each active subject
			const pastAssessmentsPromises = activeSubjects.map((subject: any) =>
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

		// Generate bar paths
		barPaths = Object.entries(gradeRanges).map(([range, count], index) => {
			const x = padding + index * (barWidth + barSpacing);
			const barHeight = count * yScale;
			const path = `M ${x} ${height - padding} h ${barWidth} v -${barHeight} h -${barWidth} Z`;
			return { path, count, status: range };
		});

		yLabels = Array.from({ length: maxCount + 1 }, (_, i) => i.toString());
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
</script>

<div class="container mx-auto px-4 py-8">
	<h1 class="text-3xl font-bold mb-8">Analytics Dashboard</h1>

	{#if loading}
		<div class="flex items-center justify-center h-64">
			<div class="animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600"></div>
		</div>
	{:else if showGrabData}
		<div class="flex flex-col items-center justify-center h-64 gap-4">
			<button class="px-6 py-3 bg-indigo-600 text-white rounded-lg shadow hover:bg-indigo-700 transition" on:click={grabData}>
				Grab Data
			</button>
			{#if error}
				<div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded-lg">{error}</div>
			{/if}
		</div>
	{:else if analyticsData}
		<div class="flex justify-end mb-4">
			<button class="px-4 py-2 bg-red-600 text-white rounded hover:bg-red-700 transition" on:click={deleteAnalytics}>
				Delete Data
			</button>
		</div>
		<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 mb-8">
			<h2 class="text-xl font-semibold mb-4">Grade Distribution</h2>
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
						<g class="transition-all duration-300 hover:opacity-80">
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
								{status}%
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

		<div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
			<h2 class="text-xl font-semibold mb-4">Raw Data</h2>
			<div class="overflow-x-auto">
				<table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
					<thead>
						<tr>
							<th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
								Subject
							</th>
							<th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
								Title
							</th>
							<th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
								Grade
							</th>
							<th class="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
								Due Date
							</th>
						</tr>
					</thead>
					<tbody class="divide-y divide-gray-200 dark:divide-gray-700">
						{#each analyticsData as assessment}
							<tr>
								<td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100">
									{assessment.subject}
								</td>
								<td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100">
									{assessment.title}
								</td>
								<td class="px-6 py-4 whitespace-nowrap text-sm">
									{#if assessment.finalGrade !== undefined}
										<span
											class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full {assessment.finalGrade >= 80
												? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200'
												: assessment.finalGrade >= 60
												? 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200'
												: 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200'}"
										>
											{assessment.finalGrade}%
										</span>
									{:else}
										<span class="text-gray-500">Not graded</span>
									{/if}
								</td>
								<td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100">
									{new Date(assessment.due).toLocaleDateString()}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	{:else}
		<div class="text-center text-gray-500 dark:text-gray-400">
			No analytics data available
		</div>
	{/if}
</div> 