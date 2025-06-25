<script lang="ts">
  import { Icon, ArrowDownTray } from 'svelte-hero-icons';
  import TimetableExport from './TimetableExport.svelte';
  import { onMount } from 'svelte';

  const {
    weekStart,
    loadingLessons,
    onPrevWeek,
    onNextWeek,
    onExportCsv,
    onExportPdf
  } = $props<{
    weekStart: Date;
    loadingLessons: boolean;
    onPrevWeek: () => void;
    onNextWeek: () => void;
    onExportCsv: () => void;
    onExportPdf: () => void;
  }>();

  let showExportMenu = $state(false);

  function weekRangeLabel() {
    const end = new Date(weekStart.valueOf() + 4 * 86400000);
    return `${weekStart.getDate()} ${weekStart.toLocaleString('default', { month: 'short' })} - ${end.getDate()} ${end.toLocaleString('default', { month: 'short' })} ${weekStart.getFullYear()}`;
  }

  function handleExportClickOutside(event: MouseEvent) {
    const target = event.target as Element;
    if (!target.closest('.export-dropdown-container')) {
      showExportMenu = false;
    }
  }

  onMount(() => {
    document.addEventListener('click', handleExportClickOutside);
    return () => {
      document.removeEventListener('click', handleExportClickOutside);
    };
  });
</script>

<div class="flex justify-between items-center px-4 py-2 shadow bg-slate-100 dark:bg-slate-800">
  <div class="flex gap-2 items-center">
    <button
      class="flex justify-center items-center w-8 h-8 rounded-full transition-transform duration-300 hover:bg-slate-200 dark:hover:bg-slate-950/40 hover:scale-110 disabled:opacity-50 disabled:cursor-not-allowed"
      on:click={onPrevWeek}
      disabled={loadingLessons}>&#60;</button>
    <span class="text-lg font-bold">{weekRangeLabel()}</span>
    <button
      class="flex justify-center items-center w-8 h-8 rounded-full transition-transform duration-300 hover:bg-slate-200 dark:hover:bg-slate-950/40 hover:scale-110 disabled:opacity-50 disabled:cursor-not-allowed"
      on:click={onNextWeek}
      disabled={loadingLessons}>&#62;</button>
  </div>
  
  <TimetableExport 
    bind:showExportMenu
    {onExportCsv}
    {onExportPdf}
  />
</div> 