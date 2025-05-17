<script lang="ts">
  import { Icon } from 'svelte-hero-icons';
  import { XMark } from 'svelte-hero-icons';

  let {
    showComposeModal,
    composeTo,
    composeSubject,
    composeBody,
    sendMessage,
    closeModal
  } = $props<{
    showComposeModal: boolean;
    composeTo: string;
    composeSubject: string;
    composeBody: string;
    sendMessage: () => void;
    closeModal: () => void;
  }>();
</script>

{#if showComposeModal}
  <div class="flex fixed inset-0 z-50 justify-center items-center bg-black bg-opacity-50 animate-fade-in">
    <div class="bg-slate-900 rounded-xl w-[32rem] max-w-full shadow-2xl flex flex-col border border-slate-800">
      <div class="p-4 border-b border-slate-800 flex items-center justify-between rounded-t-xl">
        <h2 class="text-lg font-semibold">Compose Message</h2>
        <button class="p-2 rounded-lg hover:bg-slate-800 transition-all duration-200" onclick={closeModal}>
          <Icon src={XMark} class="w-6 h-6" />
        </button>
      </div>
      <div class="flex flex-col gap-4 p-4">
        <input
          type="text"
          placeholder="To"
          bind:value={composeTo}
          class="px-4 py-2 rounded-lg bg-slate-800 text-slate-100 placeholder-slate-300 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <input
          type="text"
          placeholder="Subject"
          bind:value={composeSubject}
          class="px-4 py-2 rounded-lg bg-slate-800 text-slate-100 placeholder-slate-300 focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <textarea
          placeholder="Message..."
          bind:value={composeBody}
          rows="8"
          class="px-4 py-2 rounded-lg bg-slate-800 text-slate-100 placeholder-slate-300 focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
        ></textarea>
        <button
          class="self-end px-6 py-2 text-white bg-blue-500 rounded-lg transition-all duration-200 hover:bg-blue-600 focus:ring-2 focus:ring-blue-400 disabled:opacity-50 disabled:cursor-not-allowed"
          disabled={!composeTo.trim() || !composeSubject.trim() || !composeBody.trim()}
          onclick={sendMessage}
        >
          Send
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  @keyframes fade-in {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
  .animate-fade-in {
    animation: fade-in 0.3s ease-out;
  }
</style> 