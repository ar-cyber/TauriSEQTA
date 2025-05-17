<script lang="ts">
  import type { Message } from '../types';

  let { 
    selectedFolder,
    messages,
    loading,
    error,
    selectedMessage,
    openMessage
  } = $props<{
    selectedFolder: string;
    messages: Message[];
    loading: boolean;
    error: string | null;
    selectedMessage: Message | null;
    openMessage: (msg: Message) => void;
  }>();
</script>

<section class="w-[28rem] border-r border-slate-800 flex flex-col bg-slate-900/10">
  <div class="p-4 text-lg font-semibold border-b border-slate-800">{selectedFolder}</div>
  <div class="overflow-y-auto flex-1">
    {#if loading}
      <div class="p-8 text-center text-slate-300">Loading messages...</div>
    {:else if error}
      <div class="p-8 text-center text-red-500">{error}</div>
    {:else}
      {#each messages.filter((m: Message) => m.folder === selectedFolder) as msg (msg.id)}
        <button
          class="w-full text-left px-6 py-4 border-b border-slate-800 transition-all duration-200 flex flex-col gap-1
            rounded-lg group relative
            {selectedMessage?.id === msg.id ? 'bg-blue-950/60 shadow-lg border-blue-500 border-l-4' : ''}
            {msg.unread ? 'border-l-4 border-blue-500' : 'border-l-4 border-transparent'}
            hover:bg-slate-800 focus:outline-none focus:ring-2 focus:ring-blue-400"
          onclick={() => openMessage(msg)}
        >
          <div class="flex justify-between items-center">
            <div class="flex gap-2 items-center">
              <span class="font-bold {msg.unread ? 'text-blue-400' : ''}">{msg.sender}</span>
              <span class="text-xs text-slate-300">to {msg.to}</span>
            </div>
            <span class="text-xs text-slate-300">{msg.date}</span>
          </div>
          <div class="flex gap-2 items-center">
            <span class="font-semibold {msg.unread ? 'text-blue-400' : ''}">{msg.subject}</span>
            <span class="text-xs truncate text-slate-300">{msg.preview}</span>
          </div>
        </button>
      {:else}
        <div class="p-8 text-center text-slate-300">No messages in this folder.</div>
      {/each}
    {/if}
  </div>
</section> 