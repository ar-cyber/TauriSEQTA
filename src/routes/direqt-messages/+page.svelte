<script lang="ts">
  import { onMount } from 'svelte';
  import { seqtaFetch } from '../../utils/seqtaFetch';
  import { Icon } from 'svelte-hero-icons';
  import { Plus, Inbox, PaperAirplane, PencilSquare, Trash, DocumentDuplicate, XMark, Star } from 'svelte-hero-icons';

  // Example folders
  const folders = [
    { name: 'Inbox', icon: Inbox },
    { name: 'Sent', icon: PaperAirplane },
    { name: 'Starred', icon: Star },
    { name: 'Trash', icon: Trash }
  ];

  // Example messages
  interface Message {
    id: number;
    folder: string;
    sender: string;
    to: string;
    subject: string;
    preview: string;
    body: string;
    date: string;
    unread: boolean;
    starred?: boolean;
  }

  let messages: Message[] = $state([]);
  let loading = $state(true);
  let error = $state<string | null>(null);

  let selectedFolder = $state('Inbox');
  let selectedMessage: Message | null = $state(null);
  let showComposeModal = $state(false);
  let composeTo = $state('');
  let composeSubject = $state('');
  let composeBody = $state('');

  let detailLoading = $state(false);
  let detailError = $state<string | null>(null);

  let starring = $state(false);
  let deleting = $state(false);

  async function fetchMessages(folderLabel: string = 'inbox') {
    loading = true;
    error = null;
    try {
      if (folderLabel === 'sent') {
        // Fetch both sent and outbox, then combine
        const [sentRes, outboxRes] = await Promise.all([
          seqtaFetch('/seqta/student/load/message?', {
            method: 'POST',
            body: {
              searchValue: "",
              sortBy: "date",
              sortOrder: "desc",
              action: "list",
              label: 'sent',
              offset: 0,
              limit: 100,
              datetimeUntil: null
            }
          }),
          seqtaFetch('/seqta/student/load/message?', {
            method: 'POST',
            body: {
              searchValue: "",
              sortBy: "date",
              sortOrder: "desc",
              action: "list",
              label: 'outbox',
              offset: 0,
              limit: 100,
              datetimeUntil: null
            }
          })
        ]);
        const sentData = typeof sentRes === 'string' ? JSON.parse(sentRes) : sentRes;
        const outboxData = typeof outboxRes === 'string' ? JSON.parse(outboxRes) : outboxRes;
        const sentMsgs = (sentData?.payload?.messages || []).map((msg: any) => ({
          id: msg.id,
          folder: 'Sent',
          sender: msg.sender,
          to: msg.participants?.[0]?.name || '',
          subject: msg.subject,
          preview: msg.subject + (msg.attachments ? ' (Attachment)' : ''),
          body: '',
          date: msg.date?.replace('T', ' ').slice(0, 16) || '',
          unread: !msg.read
        }));
        const outboxMsgs = (outboxData?.payload?.messages || []).map((msg: any) => ({
          id: msg.id,
          folder: 'Sent',
          sender: msg.sender,
          to: msg.participants?.[0]?.name || '',
          subject: msg.subject,
          preview: msg.subject + (msg.attachments ? ' (Attachment)' : ''),
          body: '',
          date: msg.date?.replace('T', ' ').slice(0, 16) || '',
          unread: !msg.read
        }));
        messages = [...sentMsgs, ...outboxMsgs].sort((a, b) => b.date.localeCompare(a.date));
      } else {
        const response = await seqtaFetch(
          '/seqta/student/load/message?',
          {
            method: 'POST',
            body: {
              searchValue: "",
              sortBy: "date",
              sortOrder: "desc",
              action: "list",
              label: folderLabel,
              offset: 0,
              limit: 100,
              datetimeUntil: null
            }
          }
        );
        const data = typeof response === 'string' ? JSON.parse(response) : response;
        if (data?.payload?.messages) {
          messages = data.payload.messages.map((msg: any) => ({
            id: msg.id,
            folder: folderLabel.charAt(0).toUpperCase() + folderLabel.slice(1),
            sender: msg.sender,
            to: msg.participants?.[0]?.name || '',
            subject: msg.subject,
            preview: msg.subject + (msg.attachments ? ' (Attachment)' : ''),
            body: '',
            date: msg.date?.replace('T', ' ').slice(0, 16) || '',
            unread: !msg.read,
            starred: !!msg.starred
          }));
        } else {
          messages = [];
        }
      }
    } catch (e) {
      error = 'Failed to load messages.';
      messages = [];
    } finally {
      loading = false;
    }
  }

  onMount(() => fetchMessages('inbox'));

  async function openMessage(msg: Message) {
    selectedMessage = msg;
    msg.unread = false;
    // If already loaded, don't fetch again
    if (msg.body) return;
    detailLoading = true;
    detailError = null;
    try {
      const response = await seqtaFetch(
        '/seqta/student/load/message?',
        {
          method: 'POST',
          body: {
            action: 'message',
            id: msg.id
          }
        }
      );
      const data = typeof response === 'string' ? JSON.parse(response) : response;
      if (data?.payload?.contents) {
        msg.body = data.payload.contents;
      } else {
        msg.body = '<em>No content.</em>';
      }
      // If the API returns starred in the detail, update it
      if (typeof data?.payload?.starred !== 'undefined') {
        msg.starred = !!data.payload.starred;
      }
    } catch (e) {
      detailError = 'Failed to load message.';
      msg.body = '';
    } finally {
      detailLoading = false;
    }
  }

  function openFolder(folder: string) {
    selectedFolder = folder;
    selectedMessage = null;
    if (folder === 'Inbox') fetchMessages('inbox');
    else if (folder === 'Sent') fetchMessages('sent');
    else if (folder === 'Starred') fetchMessages('starred');
    else if (folder === 'Trash') fetchMessages('trash');
  }

  function openCompose() {
    showComposeModal = true;
    composeTo = '';
    composeSubject = '';
    composeBody = '';
  }

  function sendMessage() {
    if (!composeTo.trim() || !composeSubject.trim() || !composeBody.trim()) return;
    messages.push({
      id: messages.length + 1,
      folder: 'Sent',
      sender: 'You',
      to: composeTo,
      subject: composeSubject,
      preview: composeBody.slice(0, 60),
      body: composeBody,
      date: new Date().toISOString().slice(0, 16).replace('T', ' '),
      unread: false
    });
    showComposeModal = false;
  }

  async function starMessage(msg: Message) {
    if (starring) return;
    starring = true;
    try {
      const response = await seqtaFetch(
        '/seqta/student/save/message?',
        {
          method: 'POST',
          body: {
            mode: 'x-star',
            starred: true,
            items: [msg.id]
          }
        }
      );
      const data = typeof response === 'string' ? JSON.parse(response) : response;
      if (data?.payload?.starred) {
        msg.starred = true;
      }
    } catch (e) {
      // Optionally show error
    } finally {
      starring = false;
    }
  }

  async function deleteMessage(msg: Message) {
    if (deleting) return;
    deleting = true;
    try {
      const response = await seqtaFetch(
        '/seqta/student/save/message?',
        {
          method: 'POST',
          body: {
            mode: 'x-label',
            label: 'trash',
            items: [msg.id]
          }
        }
      );
      const data = typeof response === 'string' ? JSON.parse(response) : response;
      if (data?.payload?.label === 'trash') {
        // Remove from messages list
        messages = messages.filter(m => m.id !== msg.id);
        // If this was the open message, clear detail
        if (selectedMessage && selectedMessage.id === msg.id) {
          selectedMessage = null;
        }
      }
    } catch (e) {
      // Optionally show error
    } finally {
      deleting = false;
    }
  }
</script>

<div class="flex h-screen bg-[var(--surface)] text-[var(--text)]">
  <!-- Sidebar -->
  <aside class="w-64 border-r border-[var(--surface-alt)] flex flex-col bg-[var(--surface)]">
    <div class="p-4 border-b border-[var(--surface-alt)]">
      <button class="w-full px-4 py-2 rounded-xl bg-blue-500 text-white hover:bg-blue-600 focus:ring-2 focus:ring-blue-400 transition-all duration-200 flex items-center gap-2 shadow-md hover:shadow-lg font-semibold text-base" on:click={openCompose}>
        <Icon src={Plus} class="w-5 h-5" />
        <span>Compose</span>
      </button>
    </div>
    <nav class="flex-1 py-4">
      {#each folders as folder}
        <button
          class="w-full flex items-center gap-3 px-6 py-2 text-left font-medium rounded-lg transition-all duration-200 relative group
            {selectedFolder === folder.name ? 'bg-[var(--surface-alt)] text-blue-500 border-l-4 border-blue-500 pl-[1.25rem]' : 'border-l-4 border-transparent'}
            hover:bg-[var(--surface-alt)] focus:outline-none focus:ring-2 focus:ring-blue-400"
          on:click={() => openFolder(folder.name)}
        >
          <Icon src={folder.icon} class="w-5 h-5" />
          <span>{folder.name}</span>
        </button>
      {/each}
    </nav>
  </aside>

  <!-- Message List -->
  <section class="w-[28rem] border-r border-[var(--surface-alt)] flex flex-col bg-[var(--surface)]">
    <div class="p-4 border-b border-[var(--surface-alt)] font-semibold text-lg">{selectedFolder}</div>
    <div class="flex-1 overflow-y-auto bg-[var(--surface)]">
      {#if loading}
        <div class="p-8 text-center text-[var(--text-muted)]">Loading messages...</div>
      {:else if error}
        <div class="p-8 text-center text-red-500">{error}</div>
      {:else}
        {#each messages.filter(m => m.folder === selectedFolder) as msg (msg.id)}
          <button
            class="w-full text-left px-6 py-4 border-b border-[var(--surface-alt)] transition-all duration-200 flex flex-col gap-1
              rounded-lg group relative
              {selectedMessage?.id === msg.id ? 'bg-blue-950/60 shadow-lg border-blue-500 border-l-4' : ''}
              {msg.unread ? 'border-l-4 border-blue-500' : 'border-l-4 border-transparent'}
              hover:bg-[var(--surface-alt)] focus:outline-none focus:ring-2 focus:ring-blue-400"
            on:click={() => openMessage(msg)}
          >
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <span class="font-bold {msg.unread ? 'text-blue-400' : ''}">{msg.sender}</span>
                <span class="text-xs text-[var(--text-muted)]">to {msg.to}</span>
              </div>
              <span class="text-xs text-[var(--text-muted)]">{msg.date}</span>
            </div>
            <div class="flex items-center gap-2">
              <span class="font-semibold {msg.unread ? 'text-blue-400' : ''}">{msg.subject}</span>
              <span class="text-xs text-[var(--text-muted)] truncate">{msg.preview}</span>
            </div>
          </button>
        {:else}
          <div class="p-8 text-center text-[var(--text-muted)]">No messages in this folder.</div>
        {/each}
      {/if}
    </div>
  </section>

  <!-- Message Detail -->
  <main class="flex-1 flex flex-col bg-[var(--surface)] p-6">
    {#if selectedMessage}
      <div class="rounded-xl shadow-lg bg-[var(--surface-alt)] border border-[var(--surface-alt)] flex flex-col h-full">
        <div class="p-6 border-b border-[var(--surface)] flex items-center justify-between rounded-t-xl bg-[var(--surface-alt)]">
          <div>
            <div class="font-bold text-2xl mb-1 text-blue-400">{selectedMessage.subject}</div>
            <div class="text-sm text-[var(--text-muted)] mt-1"><span class="font-semibold text-[var(--text)]">{selectedMessage.sender}</span></div>
            <div class="text-sm text-[var(--text-muted)]">To: <span class="font-semibold text-[var(--text)]">{selectedMessage.to}</span></div>
          </div>
          <div class="flex gap-2 bg-[var(--surface)] rounded-lg p-2 shadow-sm items-center">
            {#if selectedMessage}
              <button class="p-2 rounded-lg hover:bg-yellow-400/20 focus:bg-yellow-400/30 transition-all duration-200" title="Star" on:click={() => selectedMessage && starMessage(selectedMessage)} disabled={starring || selectedMessage.starred}>
                {#if starring}
                  <svg class="w-5 h-5 animate-spin text-yellow-400" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z"></path></svg>
                {:else}
                  <Icon src={Star} class="w-5 h-5 text-yellow-400" solid={selectedMessage.starred} />
                {/if}
              </button>
            {/if}
            <button class="p-2 rounded-lg hover:bg-blue-500/20 focus:bg-blue-500/30 transition-all duration-200" on:click={openCompose} title="Reply">
              <Icon src={PencilSquare} class="w-5 h-5 text-blue-400" />
            </button>
            <button class="p-2 rounded-lg hover:bg-red-500/20 focus:bg-red-500/30 transition-all duration-200" title="Delete" on:click={() => selectedMessage && deleteMessage(selectedMessage)} disabled={deleting}>
              {#if deleting}
                <svg class="w-5 h-5 animate-spin text-red-400" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8v8z"></path></svg>
              {:else}
                <Icon src={Trash} class="w-5 h-5 text-red-400" />
              {/if}
            </button>
          </div>
        </div>
        <div class="flex-1 overflow-y-auto p-8 text-base bg-[var(--surface-alt)] rounded-b-xl">
          {#if detailLoading}
            <div class="text-center text-[var(--text-muted)]">Loading message...</div>
          {:else if detailError}
            <div class="text-center text-red-500">{detailError}</div>
          {:else}
            <div>{@html selectedMessage.body}</div>
          {/if}
        </div>
      </div>
    {:else}
      <div class="flex-1 flex items-center justify-center text-[var(--text-muted)] text-lg">
        Select a message to view its details.
      </div>
    {/if}
  </main>
</div>

<!-- Compose Modal -->
{#if showComposeModal}
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 animate-fade-in">
    <div class="bg-[var(--surface)] rounded-xl w-[32rem] max-w-full shadow-2xl flex flex-col border border-[var(--surface-alt)]">
      <div class="p-4 border-b border-[var(--surface-alt)] flex items-center justify-between rounded-t-xl">
        <h2 class="text-lg font-semibold">Compose Message</h2>
        <button class="p-2 rounded-lg hover:bg-[var(--surface-alt)] transition-all duration-200" on:click={() => showComposeModal = false}>
          <Icon src={XMark} class="w-6 h-6" />
        </button>
      </div>
      <div class="p-4 flex flex-col gap-4">
        <input
          type="text"
          placeholder="To"
          bind:value={composeTo}
          class="px-4 py-2 rounded-lg bg-[var(--surface-alt)] text-[var(--text)] placeholder-[var(--text-muted)] focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <input
          type="text"
          placeholder="Subject"
          bind:value={composeSubject}
          class="px-4 py-2 rounded-lg bg-[var(--surface-alt)] text-[var(--text)] placeholder-[var(--text-muted)] focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        <textarea
          placeholder="Message..."
          bind:value={composeBody}
          rows="8"
          class="px-4 py-2 rounded-lg bg-[var(--surface-alt)] text-[var(--text)] placeholder-[var(--text-muted)] focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
        ></textarea>
        <button
          class="self-end px-6 py-2 rounded-lg bg-blue-500 text-white hover:bg-blue-600 focus:ring-2 focus:ring-blue-400 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
          disabled={!composeTo.trim() || !composeSubject.trim() || !composeBody.trim()}
          on:click={sendMessage}
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