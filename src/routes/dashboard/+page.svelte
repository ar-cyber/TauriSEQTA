<script lang="ts">
    import { onMount } from 'svelte';
    import { seqtaFetch } from '../../utils/seqtaFetch';

    interface HomeworkItem {
        meta: number;
        id: number;
        title: string;
        items: string[];
    }

    interface HomeworkResponse {
        payload: HomeworkItem[];
        status: string;
    }

    let homeworkData: HomeworkResponse | null = null;
    let error: string | null = null;
    let loading = true;

    async function fetchHomeworkData() {
        try {
            loading = true;
            error = null;
            console.log('Making POST request to homework endpoint...');
            const response = await seqtaFetch('/seqta/student/dashlet/summary/homework', {
                method: 'POST',
                body: {},
                params: {"majhvjju": ""}
            });
            homeworkData = JSON.parse(response);
        } catch (e: any) {
            console.error('Error details:', e);
            error = e.toString();
        } finally {
            loading = false;
        }
    }

    onMount(() => {
        console.log('Dashboard component mounted');
        fetchHomeworkData();
    });
</script>

<div class="space-y-8 max-w-3xl mx-auto py-10">
	<div class="flex justify-between items-center mb-6">
		<h1 class="text-2xl font-bold" style="color: var(--text);">Homework Dashboard</h1>
	</div>

	{#if loading}
		<div class="flex justify-center items-center py-12">
			<p style="color: var(--text-muted);">Loading homework data…</p>
		</div>
	{:else if error}
		<div class="bg-red-100 text-red-700 rounded-lg px-6 py-4 mb-4 border border-red-200">
			<p>Error: {error}</p>
		</div>
	{:else if homeworkData}
		<div class="flex flex-col gap-6">
			{#each homeworkData.payload as homework}
				<div class="bg-[var(--surface)] rounded-xl shadow-lg border-l-8" style="border-color: #1a73e8;">
					<div class="px-6 pt-5 pb-3">
						<h2 class="text-lg font-bold mb-2" style="color: var(--text);">{homework.title}</h2>
						<div class="flex flex-col gap-3">
							{#each homework.items as item}
								<div class="flex gap-2 items-start bg-[var(--surface-alt)] rounded-lg px-4 py-3 border border-[var(--surface-alt)]">
									<span class="text-xl text-blue-500 mt-1">•</span>
									<span class="text-[var(--text)]">{item}</span>
								</div>
							{/each}
						</div>
					</div>
				</div>
			{/each}
		</div>
	{:else}
		<p class="text-center" style="color: var(--text-muted);">No homework data available</p>
	{/if}
</div>

<style>
:global(body) {
	background: var(--background);
}
</style> 