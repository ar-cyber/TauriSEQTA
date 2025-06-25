<script lang="ts">
  import { Icon, ArrowDownTray, TableCells, DocumentText } from 'svelte-hero-icons';
  import { fly } from 'svelte/transition';

  export let showExportMenu: boolean;
  export let onExportCsv: () => void;
  export let onExportPdf: () => void;
</script>

<div class="inline-block relative text-left export-dropdown-container">
  <button
    class="flex gap-2 items-center px-4 py-2 rounded-xl border transition-all duration-200 bg-white/60 border-slate-200/70 hover:bg-white/80 dark:bg-slate-700/60 dark:border-slate-700/40 dark:hover:bg-slate-800/80 focus:outline-none focus:ring-2 focus:ring-slate-500/50"
    on:click={() => (showExportMenu = !showExportMenu)}
    aria-label="Export options">
    <Icon src={ArrowDownTray} class="w-4 h-4 text-slate-700 dark:text-slate-300" />
    <span class="font-medium text-slate-900 dark:text-white">Export</span>
  </button>
  {#if showExportMenu}
    <div
      class="absolute right-0 z-50 mt-3 w-56 rounded-2xl border shadow-2xl backdrop-blur-md bg-white/95 border-slate-200/60 dark:bg-slate-900/50 dark:border-slate-700/40"
      transition:fly={{ y: -8, duration: 200, opacity: 0 }}>
      <div class="p-2">
        <button
          class="flex gap-3 items-center px-4 py-3 w-full text-left rounded-xl transition-all duration-200 text-slate-700 hover:bg-slate-100 dark:text-slate-200 dark:hover:bg-slate-900/50 group"
          on:click={() => {
            showExportMenu = false;
            onExportCsv();
          }}>
          <div
            class="flex justify-center items-center w-8 h-8 rounded-lg transition-colors bg-slate-100 group-hover:bg-slate-200 dark:bg-slate-700/50 dark:group-hover:bg-slate-700">
            <Icon src={TableCells} class="w-4 h-4 text-slate-600 dark:text-slate-400" />
          </div>
          <div class="flex-1">
            <div class="font-medium">Export as CSV</div>
            <div class="text-xs text-slate-500 dark:text-slate-400">Spreadsheet format</div>
          </div>
        </button>

        <button
          class="flex gap-3 items-center px-4 py-3 w-full text-left rounded-xl transition-all duration-200 text-slate-700 hover:bg-slate-100 dark:text-slate-200 dark:hover:bg-slate-900/50 group"
          on:click={() => {
            showExportMenu = false;
            onExportPdf();
          }}>
          <div
            class="flex justify-center items-center w-8 h-8 rounded-lg transition-colors bg-slate-100 group-hover:bg-slate-200 dark:bg-slate-700/50 dark:group-hover:bg-slate-700">
            <Icon src={DocumentText} class="w-4 h-4 text-slate-600 dark:text-slate-400" />
          </div>
          <div class="flex-1">
            <div class="font-medium">Export as PDF</div>
            <div class="text-xs text-slate-500 dark:text-slate-400">Portable document</div>
          </div>
        </button>
      </div>
    </div>
  {/if}
</div> 