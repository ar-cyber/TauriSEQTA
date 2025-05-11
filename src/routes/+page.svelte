<script>
	// Placeholder data for dashboard
	const integrations = [
		{ name: 'Outlook', icon: '📅' },
		{ name: 'Office365', icon: '🏢' },
		{ name: 'Google', icon: '🌐' }
	];

	const lessons = [];

	const assessments = [
		{
			date: 'Tomorrow',
			day: 'Monday 12 May',
			title: 'TauriSEQTA',
			subject: 'Year 10 sss'
		}
	];
</script>

<div class="space-y-6">
	<!-- Integrations -->
	<div class="flex gap-6">
		{#each integrations as integration}
			<div
				class="flex-1 flex items-center justify-center py-4 shadow text-lg font-semibold"
				style="background: var(--surface); color: var(--text); border-radius: 1rem;"
			>
				<span class="text-2xl mr-2">{integration.icon}</span> {integration.name}
			</div>
		{/each}
	</div>

	<!-- Today's Lessons -->
	<div style="background: var(--surface); border-radius: 1rem;" class="shadow p-0">
		<div class="px-6 py-3 border-b flex items-center justify-between rounded-t-2xl" style="border-color: var(--surface-alt);">
			<span style="color: var(--text);" class="font-semibold text-lg">Today's Lessons</span>
			<div class="flex gap-2">
				<button class="w-8 h-8 flex items-center justify-center rounded-full hover:bg-[color:var(--surface-alt)]" style="color: var(--text);">&#60;</button>
				<button class="w-8 h-8 flex items-center justify-center rounded-full hover:bg-[color:var(--surface-alt)]" style="color: var(--text);">&#62;</button>
			</div>
		</div>
		<div class="flex flex-col items-center justify-center py-12">
			<div style="background: var(--background); color: var(--surface);" class="rounded-full w-24 h-24 flex items-center justify-center mb-4">
				<span class="text-6xl">Q</span>
			</div>
			<p style="color: var(--text);" class="text-2xl">No lessons available.</p>
		</div>
	</div>

	<!-- Upcoming Assessments -->
	<div style="background: var(--surface); border-radius: 1rem;" class="shadow p-0">
		<div class="px-6 py-3 border-b flex items-center justify-between rounded-t-2xl" style="border-color: var(--surface-alt);">
			<span style="color: var(--text);" class="font-semibold text-lg">Upcoming Assessments</span>
			<div class="flex gap-2 flex-wrap">
				{#each ['10COM','10EIF','10ESH1','10FOH1','10HSY'] as className}
					<span style="background: var(--surface-alt); color: var(--text);" class="text-xs px-3 py-1 rounded-full">{className}</span>
				{/each}
			</div>
		</div>
		<div class="space-y-4 p-6">
			{#each assessments as a}
				<div style="background: var(--surface-alt); border: 1px solid var(--surface-alt);" class="rounded-xl p-4 flex items-center gap-4">
					<div style="background: var(--background); color: var(--surface);" class="rounded-lg w-16 h-16 flex items-center justify-center">
						<span class="text-3xl">📄</span>
					</div>
					<div class="flex-1">
						<div style="color: var(--text);" class="font-bold text-lg">{a.date}</div>
						<div style="color: var(--text-muted);" class="text-sm">{a.day}</div>
						<div class="mt-2">
							<span class="block text-xs font-semibold uppercase" style="color: var(--text-muted);">{a.subject}</span>
							<span class="block text-base font-semibold" style="color: var(--text);">{a.title}</span>
						</div>
					</div>
				</div>
			{/each}
		</div>
	</div>
</div>
