<script lang="ts">
  import { Icon } from "svelte-hero-icons";
  import { PencilSquare, Trash, Star, ArrowUturnLeft } from "svelte-hero-icons";
  import type { Message } from "../types";
  import DOMPurify from "dompurify";
  import { onMount } from "svelte";

  let {
    selectedMessage,
    selectedFolder,
    detailLoading,
    detailError,
    openCompose,
    starMessage,
    deleteMessage,
    restoreMessage,
    starring,
    deleting,
    restoring,
  } = $props<{
    selectedMessage: Message | null;
    selectedFolder: string;
    detailLoading: boolean;
    detailError: string | null;
    openCompose: () => void;
    starMessage: (msg: Message) => Promise<void>;
    deleteMessage: (msg: Message) => Promise<void>;
    restoreMessage: (msg: Message) => Promise<void>;
    starring: boolean;
    deleting: boolean;
    restoring: boolean;
  }>();

  let iframe: HTMLIFrameElement | null = $state(null);

  function updateIframeContent() {
    if (!selectedMessage || !iframe || !iframe.contentWindow) return;

    const sanitizedContent = DOMPurify.sanitize(selectedMessage.body);

    const html = `
      <!DOCTYPE html>
      <html>
        <head>
          <meta charset="utf-8">
          <style>
            body {
              font-family: system-ui, -apple-system, sans-serif;
              color: #f1f5f9;
              margin: 0;
              padding: 0;
              background-color: transparent;
            }

            .forward {
              border: 1px solid rgba(255, 255, 255, 0.1);
              background-color: rgba(0, 0, 0, 0.05);
              padding: 8px;
              border-radius: 8px;
              margin: 0;

              .preamble {
                font-size: 12px;
                color: rgba(255, 255, 255, 0.8);
                margin-bottom: 6px;
                padding-bottom: 6px;
                border-bottom: 1px solid rgba(255, 255, 255, 0.1);
              }
            }
          </style>
        </head>
        <body>${sanitizedContent}</body>
      </html>
    `;

    const doc = iframe.contentWindow.document;
    doc.open();
    doc.write(html);
    doc.close();
  }

  $effect(() => {
    if (selectedMessage && iframe) {
      updateIframeContent();
    }
  });

  onMount(() => {
    if (selectedMessage && iframe) {
      updateIframeContent();
    }
  });
</script>

<main class="flex flex-col flex-1 p-6">
  {#if selectedMessage}
    <div>
      <div
        class="flex justify-between items-center p-3 rounded-t-xl border border-slate-900 bg-slate-800/10"
      >
        <div>
          <div class="mb-1 text-2xl font-bold text-blue-400">
            {selectedMessage.subject}
          </div>
          <div class="mt-1 text-sm text-slate-300">
            <span class="font-semibold text-slate-100"
              >{selectedMessage.sender}</span
            >
          </div>
          <div class="text-sm text-slate-300">
            To: <span class="font-semibold text-slate-100"
              >{selectedMessage.to}</span
            >
          </div>
        </div>
        <div
          class="flex gap-2 items-center p-2 rounded-lg shadow-sm bg-slate-900"
        >
          {#if selectedFolder === "Trash"}
            <button
              class="flex flex-col justify-center items-center p-1.5 rounded-lg transition-all duration-200 hover:bg-green-400/20 focus:bg-green-400/30"
              title="Restore"
              onclick={() => selectedMessage && restoreMessage(selectedMessage)}
              disabled={restoring}
            >
              {#if restoring}
                <svg
                  class="w-4 h-4 text-green-400 animate-spin"
                  fill="none"
                  viewBox="0 0 24 24"
                  ><circle
                    class="opacity-25"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    stroke-width="4"
                  ></circle><path
                    class="opacity-75"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8v8z"
                  ></path></svg
                >
              {:else}
                <Icon
                  src={ArrowUturnLeft}
                  class="mb-0.5 w-4 h-4 text-green-400"
                />
              {/if}
              <span class="text-sm font-medium text-green-400">Restore</span>
            </button>
          {:else if selectedFolder === "Starred"}
            <button
              class="flex flex-col justify-center items-center p-2 rounded-lg transition-all duration-200 hover:bg-yellow-400/20 focus:bg-yellow-400/30"
              title="Unstar"
              onclick={() => selectedMessage && starMessage(selectedMessage)}
              disabled={starring || !selectedMessage.starred}
            >
              {#if starring}
                <svg
                  class="w-5 h-5 text-yellow-400 animate-spin"
                  fill="none"
                  viewBox="0 0 24 24"
                  ><circle
                    class="opacity-25"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    stroke-width="4"
                  ></circle><path
                    class="opacity-75"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8v8z"
                  ></path></svg
                >
              {:else}
                <Icon
                  src={Star}
                  class="mb-1 w-5 h-5 text-yellow-400"
                  solid={true}
                />
              {/if}
              <span class="font-semibold text-yellow-400">Unstar</span>
            </button>
          {:else}
            <button
              class="p-2 rounded-lg transition-all duration-200 hover:bg-yellow-400/20 focus:bg-yellow-400/30"
              title="Star"
              onclick={() => selectedMessage && starMessage(selectedMessage)}
              disabled={starring || selectedMessage.starred}
            >
              {#if starring}
                <svg
                  class="w-5 h-5 text-yellow-400 animate-spin"
                  fill="none"
                  viewBox="0 0 24 24"
                  ><circle
                    class="opacity-25"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    stroke-width="4"
                  ></circle><path
                    class="opacity-75"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8v8z"
                  ></path></svg
                >
              {:else}
                <Icon
                  src={Star}
                  class="w-5 h-5 text-yellow-400"
                  solid={selectedMessage.starred}
                />
              {/if}
            </button>
          {/if}
          <button
            class="p-2 rounded-lg transition-all duration-200 hover:bg-blue-500/20 focus:bg-blue-500/30"
            onclick={openCompose}
            title="Reply"
          >
            <Icon src={PencilSquare} class="w-5 h-5 text-blue-400" />
          </button>
          <button
            class="p-2 rounded-lg transition-all duration-200 hover:bg-red-500/20 focus:bg-red-500/30"
            title="Delete"
            onclick={() => selectedMessage && deleteMessage(selectedMessage)}
            disabled={deleting}
          >
            {#if deleting}
              <svg
                class="w-5 h-5 text-red-400 animate-spin"
                fill="none"
                viewBox="0 0 24 24"
                ><circle
                  class="opacity-25"
                  cx="12"
                  cy="12"
                  r="10"
                  stroke="currentColor"
                  stroke-width="4"
                ></circle><path
                  class="opacity-75"
                  fill="currentColor"
                  d="M4 12a8 8 0 018-8v8z"
                ></path></svg
              >
            {:else}
              <Icon src={Trash} class="w-5 h-5 text-red-400" />
            {/if}
          </button>
        </div>
      </div>
      <div
        class="flex-1 flex-shrink p-4 text-base rounded-b-xl border border-t-0 border-slate-900 bg-slate-800/10"
      >
        {#if detailLoading}
          <div class="text-center text-slate-300">Loading message...</div>
        {:else if detailError}
          <div class="text-center text-red-500">{detailError}</div>
        {:else}
          <iframe
            bind:this={iframe}
            title="Message Content"
            sandbox="allow-same-origin"
            class="w-full border-0 min-h-[300px]"
          ></iframe>
        {/if}
      </div>
    </div>
  {:else}
    <div class="flex flex-1 justify-center items-center text-lg text-slate-300">
      Select a message to view its details.
    </div>
  {/if}
</main>
