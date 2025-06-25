<script lang="ts">
  import Modal from './Modal.svelte';
  import { Icon, ArrowDownTray } from 'svelte-hero-icons';
  import { saveAs } from 'file-saver';

  const {
    showPdfViewer,
    pdfUrl,
    pdfLoading,
    onClose
  } = $props<{
    showPdfViewer: boolean;
    pdfUrl: string | null;
    pdfLoading: boolean;
    onClose: () => void;
  }>();

  let showPdfViewerState = $state(showPdfViewer);

  $effect(() => {
    showPdfViewerState = showPdfViewer;
  });

  $effect(() => {
    if (!showPdfViewerState) {
      onClose();
    }
  });
</script>

<Modal
  bind:open={showPdfViewerState}
  onclose={onClose}
  maxWidth="max-w-4xl"
  customClasses="min-h-[80vh]"
  title="Timetable PDF"
  ariaLabel="Timetable PDF Viewer">
  <div class="absolute top-6 right-6 z-10 pr-12 pointer-events-none">
    <button
      class="flex justify-center items-center w-10 h-10 rounded-xl transition-all duration-200 pointer-events-auto bg-slate-100 hover:bg-slate-200 dark:bg-slate-800 dark:hover:bg-slate-700"
      on:click={() => {
        if (pdfUrl) {
          saveAs(pdfUrl, 'timetable.pdf');
        }
      }}
      aria-label="Download">
      <Icon src={ArrowDownTray} class="w-5 h-5 text-slate-700 dark:text-slate-300" />
    </button>
  </div>

  <div class="overflow-hidden h-full">
    {#if pdfLoading}
      <div class="flex justify-center items-center h-full">
        <div
          class="w-16 h-16 rounded-full border-4 animate-spin border-indigo-500/30 border-t-indigo-500">
        </div>
      </div>
    {:else if pdfUrl}
      <iframe src={pdfUrl} class="w-full min-h-[80vh]" title="Timetable PDF"></iframe>
    {/if}
  </div>
</Modal> 