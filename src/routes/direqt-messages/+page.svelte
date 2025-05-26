<script lang="ts">
  import { onMount } from 'svelte';
  import { seqtaFetch } from '../../utils/seqtaFetch';
  import { type Message, type Folder } from './types';
  import { cache } from '../../utils/cache';

  // Components
  import Sidebar from './components/Sidebar.svelte';
  import MessageList from './components/MessageList.svelte';
  import MessageDetail from './components/Message.svelte';
  import ComposeModal from './components/ComposeModal.svelte';

  let messages = $state<Message[]>([]);
  let loading = $state(true);
  let error = $state<string | null>(null);

  let selectedFolder = $state<Folder>('Inbox');
  let selectedMessage = $state<Message | null>(null);
  let showComposeModal = $state(false);
  let composeTo = $state('');
  let composeSubject = $state('');
  let composeBody = $state('');

  let detailLoading = $state(false);
  let detailError = $state<string | null>(null);

  let starring = $state(false);
  let deleting = $state(false);
  let restoring = $state(false);

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

  $effect(() => {
    // This effect will replace onMount
    fetchMessages('inbox');
  });

  async function openMessage(msg: Message) {
    selectedMessage = msg;
    msg.unread = false;
    
    // Check cache first
    const cacheKey = `message_${msg.id}`;
    const cachedContent = cache.get<string>(cacheKey);
    
    if (cachedContent) {
      msg.body = cachedContent;
      return;
    }
    
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
        // Cache the message content for 24 hours
        cache.set(cacheKey, msg.body, 1440); // 24 hours TTL
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

  function openFolder(folder: Folder) {
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

  function closeModal() {
    showComposeModal = false;
  }

  function sendMessage() {
    if (!composeTo.trim() || !composeSubject.trim() || !composeBody.trim()) return;
    messages = [...messages, {
      id: Math.max(0, ...messages.map(m => m.id)) + 1,
      folder: 'Sent',
      sender: 'You',
      to: composeTo,
      subject: composeSubject,
      preview: composeBody.slice(0, 60),
      body: composeBody,
      date: new Date().toISOString().slice(0, 16).replace('T', ' '),
      unread: false
    }];
    showComposeModal = false;
  }

  async function starMessage(msg: Message) {
    if (starring) return;
    starring = true;
    try {
      let newStarred = true;
      // If in Starred folder and already starred, unstar
      if (selectedFolder === 'Starred' && msg.starred) {
        newStarred = false;
      }
      const response = await seqtaFetch(
        '/seqta/student/save/message?',
        {
          method: 'POST',
          body: {
            mode: 'x-star',
            starred: newStarred,
            items: [msg.id]
          }
        }
      );
      const data = typeof response === 'string' ? JSON.parse(response) : response;
      if (typeof data?.payload?.starred !== 'undefined') {
        msg.starred = !!data.payload.starred;
        // If unstarred in Starred folder, remove from list
        if (!msg.starred && selectedFolder === 'Starred') {
          messages = messages.filter(m => m.id !== msg.id);
          if (selectedMessage && selectedMessage.id === msg.id) {
            selectedMessage = null;
          }
        }
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

  async function restoreMessage(msg: Message) {
    if (restoring) return;
    restoring = true;
    try {
      const response = await seqtaFetch(
        '/seqta/student/save/message?',
        {
          method: 'POST',
          body: {
            mode: 'x-label',
            label: 'inbox',
            items: [msg.id]
          }
        }
      );
      const data = typeof response === 'string' ? JSON.parse(response) : response;
      if (data?.payload?.label === 'inbox') {
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
      restoring = false;
    }
  }
</script>

<div class="flex h-full">
  <div class="flex h-full w-full max-xl:flex-col">
    <Sidebar 
      selectedFolder={selectedFolder}
      openFolder={openFolder}
      openCompose={openCompose}
    />
    
    <MessageList
      selectedFolder={selectedFolder}
      messages={messages}
      loading={loading}
      error={error}
      selectedMessage={selectedMessage}
      openMessage={openMessage}
    />
  </div>
  
  <!-- Message detail view - full screen on mobile -->
  <div class="hidden xl:block flex-1">
    <MessageDetail
      selectedMessage={selectedMessage}
      selectedFolder={selectedFolder}
      detailLoading={detailLoading}
      detailError={detailError}
      openCompose={openCompose}
      starMessage={starMessage}
      deleteMessage={deleteMessage}
      restoreMessage={restoreMessage}
      starring={starring}
      deleting={deleting}
      restoring={restoring}
    />
  </div>

  <!-- Mobile message detail view -->
  {#if selectedMessage}
    <div class="xl:hidden fixed inset-0 z-50 bg-slate-900/95 backdrop-blur-sm">
      <div class="h-full flex flex-col">
        <div class="flex items-center justify-between p-4 border-b border-slate-800/50">
          <button
            class="flex items-center gap-2 text-slate-300 hover:text-white transition-colors"
            onclick={() => selectedMessage = null}
          >
            <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M9.707 16.707a1 1 0 01-1.414 0l-6-6a1 1 0 010-1.414l6-6a1 1 0 011.414 1.414L5.414 9H17a1 1 0 110 2H5.414l4.293 4.293a1 1 0 010 1.414z" clip-rule="evenodd" />
            </svg>
            <span class="text-sm font-medium">Back</span>
          </button>
          <span class="text-sm font-medium text-slate-300">Message</span>
          <div class="w-8"></div> <!-- Spacer for alignment -->
        </div>
        
        <div class="flex-1 overflow-y-auto">
          <MessageDetail
            selectedMessage={selectedMessage}
            selectedFolder={selectedFolder}
            detailLoading={detailLoading}
            detailError={detailError}
            openCompose={openCompose}
            starMessage={starMessage}
            deleteMessage={deleteMessage}
            restoreMessage={restoreMessage}
            starring={starring}
            deleting={deleting}
            restoring={restoring}
          />
        </div>
      </div>
    </div>
  {/if}
</div>

<ComposeModal
  showComposeModal={showComposeModal}
  composeTo={composeTo}
  composeSubject={composeSubject}
  composeBody={composeBody}
  sendMessage={sendMessage}
  closeModal={closeModal}
/>

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

  @keyframes slide-in {
    from {
      transform: translateX(20px);
      opacity: 0;
    }
    to {
      transform: translateX(0);
      opacity: 1;
    }
  }

  @keyframes scale-in {
    from {
      transform: scale(0.95);
      opacity: 0;
    }
    to {
      transform: scale(1);
      opacity: 1;
    }
  }

  .animate-fadeIn {
    animation: fade-in 0.3s ease-out;
  }

  .animate-slideIn {
    animation: slide-in 0.3s ease-out;
  }

  .animate-scaleIn {
    animation: scale-in 0.3s ease-out;
  }
</style> 