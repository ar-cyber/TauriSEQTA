<script lang="ts">
  import { onMount } from 'svelte';
  import { Icon } from 'svelte-hero-icons';
  import { PaperAirplane, PaperClip, EllipsisHorizontal, UserCircle, Plus, XMark } from 'svelte-hero-icons';

  interface Recipient {
    id: number;
    name: string;
    role: string;
    avatar: string;
  }

  interface Conversation {
    id: number;
    name: string;
    role: string;
    avatar: string;
    lastMessage: string;
    time: string;
    unread: number;
    online: boolean;
  }

  interface Message {
    id: number;
    sender: string;
    content: string;
    time: string;
    isMe: boolean;
  }

  // Example data
  const conversations: Conversation[] = [
    {
      id: 1,
      name: "Mr. Smith",
      role: "Mathematics Teacher",
      avatar: "MS",
      lastMessage: "Don't forget to submit your assignment by Friday",
      time: "10:30 AM",
      unread: 2,
      online: true
    },
    {
      id: 2,
      name: "Mrs. Johnson",
      role: "Science Department",
      avatar: "SJ",
      lastMessage: "Great work on the lab report!",
      time: "Yesterday",
      unread: 0,
      online: false
    },
    {
      id: 3,
      name: "Student Support",
      role: "Administration",
      avatar: "SS",
      lastMessage: "Your request has been approved",
      time: "2 days ago",
      unread: 0,
      online: true
    }
  ];

  const messages: Message[] = [
    {
      id: 1,
      sender: "Mr. Smith",
      content: "Hi there! How's the assignment coming along?",
      time: "10:15 AM",
      isMe: false
    },
    {
      id: 2,
      sender: "You",
      content: "I'm working on it now. I have a question about question 3.",
      time: "10:20 AM",
      isMe: true
    },
    {
      id: 3,
      sender: "Mr. Smith",
      content: "Sure, what's your question?",
      time: "10:25 AM",
      isMe: false
    },
    {
      id: 4,
      sender: "You",
      content: "I'm not sure how to approach the quadratic equation part. Could you explain it?",
      time: "10:28 AM",
      isMe: true
    },
    {
      id: 5,
      sender: "Mr. Smith",
      content: "Don't forget to submit your assignment by Friday",
      time: "10:30 AM",
      isMe: false
    }
  ];

  // Example recipients data
  const allRecipients: Recipient[] = [
    { id: 1, name: "Mr. Smith", role: "Mathematics Teacher", avatar: "MS" },
    { id: 2, name: "Mrs. Johnson", role: "Science Department", avatar: "SJ" },
    { id: 3, name: "Student Support", role: "Administration", avatar: "SS" },
    { id: 4, name: "Mr. Brown", role: "English Teacher", avatar: "MB" },
    { id: 5, name: "Ms. Davis", role: "History Department", avatar: "MD" }
  ];

  let selectedConversation = $state<Conversation>(conversations[0]);
  let newMessage = $state('');
  let searchQuery = $state('');
  let showComposeModal = $state(false);
  let recipientSearch = $state('');
  let selectedRecipient = $state<Recipient | null>(null);
  let composeMessage = $state('');

  // Filter recipients based on search
  let filteredRecipients = $derived(
    allRecipients.filter(recipient => 
      recipient.name.toLowerCase().includes(recipientSearch.toLowerCase()) ||
      recipient.role.toLowerCase().includes(recipientSearch.toLowerCase())
    )
  );

  function sendMessage() {
    if (!newMessage.trim()) return;
    messages.push({
      id: messages.length + 1,
      sender: "You",
      content: newMessage,
      time: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
      isMe: true
    });
    newMessage = '';
  }

  function handleKeyPress(event: KeyboardEvent) {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      sendMessage();
    }
  }

  function startNewConversation() {
    if (!selectedRecipient || !composeMessage.trim()) return;
    
    // Add new conversation
    const newConversation = {
      id: conversations.length + 1,
      name: selectedRecipient.name,
      role: selectedRecipient.role,
      avatar: selectedRecipient.avatar,
      lastMessage: composeMessage,
      time: "Just now",
      unread: 0,
      online: true
    };
    
    conversations.push(newConversation);
    selectedConversation = newConversation;
    
    // Add initial message
    messages.push({
      id: messages.length + 1,
      sender: "You",
      content: composeMessage,
      time: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
      isMe: true
    });
    
    // Reset and close modal
    composeMessage = '';
    selectedRecipient = null;
    recipientSearch = '';
    showComposeModal = false;
  }
</script>

<div class="flex h-screen bg-[var(--surface)] text-[var(--text)]">
  <!-- Sidebar -->
  <div class="w-80 border-r border-[var(--surface-alt)] flex flex-col">
    <!-- New Message Button -->
    <div class="p-4 border-b border-[var(--surface-alt)]">
      <button
        class="w-full px-4 py-2 rounded-lg bg-blue-500 text-white hover:bg-blue-600 transition-all duration-200 flex items-center justify-center gap-2 transform hover:scale-[1.02] active:scale-[0.98] shadow-md hover:shadow-lg"
        on:click={() => showComposeModal = true}
      >
        <Icon src={Plus} class="w-5 h-5" />
        <span>New Message</span>
      </button>
    </div>

    <!-- Search -->
    <div class="p-4 border-b border-[var(--surface-alt)]">
      <div class="relative group">
        <input
          type="text"
          bind:value={searchQuery}
          placeholder="Search messages..."
          class="w-full px-4 py-2 rounded-lg bg-[var(--surface-alt)] text-[var(--text)] placeholder-[var(--text-muted)] focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all duration-200 group-hover:bg-[var(--surface-hover)]"
        />
      </div>
    </div>

    <!-- Conversations -->
    <div class="flex-1 overflow-y-auto">
      {#each conversations as conversation}
        <button
          class="w-full p-4 flex items-center gap-3 hover:bg-[var(--surface-alt)] transition-all duration-200 {selectedConversation.id === conversation.id ? 'bg-[var(--surface-alt)]' : ''} group"
          on:click={() => selectedConversation = conversation}
        >
          <div class="relative">
            <div class="w-12 h-12 rounded-full bg-blue-600 text-white flex items-center justify-center font-bold text-lg transition-transform duration-200 group-hover:scale-110">
              {conversation.avatar}
            </div>
            {#if conversation.online}
              <div class="absolute bottom-0 right-0 w-3 h-3 rounded-full bg-green-500 border-2 border-[var(--surface)] animate-pulse"></div>
            {/if}
          </div>
          <div class="flex-1 min-w-0">
            <div class="flex items-center justify-between">
              <span class="font-semibold truncate group-hover:text-blue-500 transition-colors duration-200">{conversation.name}</span>
              <span class="text-xs text-[var(--text-muted)]">{conversation.time}</span>
            </div>
            <div class="flex items-center justify-between">
              <span class="text-sm text-[var(--text-muted)] truncate group-hover:text-[var(--text)] transition-colors duration-200">{conversation.lastMessage}</span>
              {#if conversation.unread}
                <span class="ml-2 px-2 py-0.5 rounded-full bg-blue-500 text-white text-xs animate-bounce">{conversation.unread}</span>
              {/if}
            </div>
          </div>
        </button>
      {/each}
    </div>
  </div>

  <!-- Chat Area -->
  <div class="flex-1 flex flex-col">
    <!-- Chat Header -->
    <div class="h-16 border-b border-[var(--surface-alt)] flex items-center justify-between px-6">
      <div class="flex items-center gap-3">
        <div class="relative group">
          <div class="w-10 h-10 rounded-full bg-blue-600 text-white flex items-center justify-center font-bold transition-transform duration-200 group-hover:scale-110">
            {selectedConversation.avatar}
          </div>
          {#if selectedConversation.online}
            <div class="absolute bottom-0 right-0 w-2.5 h-2.5 rounded-full bg-green-500 border-2 border-[var(--surface)] animate-pulse"></div>
          {/if}
        </div>
        <div>
          <h2 class="font-semibold group-hover:text-blue-500 transition-colors duration-200">{selectedConversation.name}</h2>
          <p class="text-sm text-[var(--text-muted)]">{selectedConversation.role}</p>
        </div>
      </div>
      <button class="p-2 rounded-lg hover:bg-[var(--surface-alt)] transition-all duration-200 hover:rotate-90" on:click={() => {}}>
        <Icon src={EllipsisHorizontal} class="w-6 h-6" />
      </button>
    </div>

    <!-- Messages -->
    <div class="flex-1 overflow-y-auto p-6 space-y-4">
      {#each messages as message}
        <div class="flex {message.isMe ? 'justify-end' : 'justify-start'} animate-fade-in">
          <div class="max-w-[70%] {message.isMe ? 'bg-blue-500 text-white' : 'bg-[var(--surface-alt)]'} rounded-2xl px-4 py-2 shadow-sm hover:shadow-md transition-all duration-200 transform hover:scale-[1.02]">
            {#if !message.isMe}
              <div class="text-xs font-semibold mb-1">{message.sender}</div>
            {/if}
            <div class="text-sm">{message.content}</div>
            <div class="text-xs mt-1 opacity-70 text-right">{message.time}</div>
          </div>
        </div>
      {/each}
    </div>

    <!-- Message Input -->
    <div class="p-4 border-t border-[var(--surface-alt)]">
      <div class="flex items-center gap-2">
        <button class="p-2 rounded-lg hover:bg-[var(--surface-alt)] transition-all duration-200 hover:rotate-12">
          <Icon src={PaperClip} class="w-6 h-6" />
        </button>
        <div class="flex-1 relative">
          <textarea
            bind:value={newMessage}
            on:keypress={handleKeyPress}
            placeholder="Type a message..."
            class="w-full px-4 py-2 rounded-lg bg-[var(--surface-alt)] text-[var(--text)] placeholder-[var(--text-muted)] resize-none focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all duration-200 hover:bg-[var(--surface-hover)]"
            rows="1"
          ></textarea>
        </div>
        <button
          on:click={() => sendMessage()}
          class="p-2 rounded-lg bg-blue-500 text-white hover:bg-blue-600 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed transform hover:scale-110 active:scale-95 disabled:transform-none"
          disabled={!newMessage.trim()}
        >
          <Icon src={PaperAirplane} class="w-6 h-6" />
        </button>
      </div>
    </div>
  </div>
</div>

<!-- Compose Message Modal -->
{#if showComposeModal}
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 animate-fade-in">
    <div class="bg-[var(--surface)] rounded-lg w-[500px] max-h-[80vh] flex flex-col shadow-2xl transform transition-all duration-300 scale-100">
      <!-- Modal Header -->
      <div class="p-4 border-b border-[var(--surface-alt)] flex items-center justify-between">
        <h2 class="text-lg font-semibold">New Message</h2>
        <button
          class="p-2 rounded-lg hover:bg-[var(--surface-alt)] transition-all duration-200 hover:rotate-90"
          on:click={() => showComposeModal = false}
        >
          <Icon src={XMark} class="w-6 h-6" />
        </button>
      </div>

      <!-- Recipient Search -->
      <div class="p-4 border-b border-[var(--surface-alt)]">
        <input
          type="text"
          bind:value={recipientSearch}
          placeholder="Search for a recipient..."
          class="w-full px-4 py-2 rounded-lg bg-[var(--surface-alt)] text-[var(--text)] placeholder-[var(--text-muted)] focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all duration-200 hover:bg-[var(--surface-hover)]"
        />
      </div>

      <!-- Recipient List -->
      <div class="flex-1 overflow-y-auto p-4">
        {#each filteredRecipients as recipient}
          <button
            class="w-full p-3 flex items-center gap-3 hover:bg-[var(--surface-alt)] transition-all duration-200 rounded-lg {selectedRecipient?.id === recipient.id ? 'bg-[var(--surface-alt)]' : ''} group"
            on:click={() => selectedRecipient = recipient}
          >
            <div class="w-10 h-10 rounded-full bg-blue-600 text-white flex items-center justify-center font-bold transition-transform duration-200 group-hover:scale-110">
              {recipient.avatar}
            </div>
            <div class="text-left">
              <div class="font-semibold group-hover:text-blue-500 transition-colors duration-200">{recipient.name}</div>
              <div class="text-sm text-[var(--text-muted)] group-hover:text-[var(--text)] transition-colors duration-200">{recipient.role}</div>
            </div>
          </button>
        {/each}
      </div>

      <!-- Message Input -->
      <div class="p-4 border-t border-[var(--surface-alt)]">
        <textarea
          bind:value={composeMessage}
          placeholder="Type your message..."
          class="w-full px-4 py-2 rounded-lg bg-[var(--surface-alt)] text-[var(--text)] placeholder-[var(--text-muted)] resize-none focus:outline-none focus:ring-2 focus:ring-blue-500 mb-4 transition-all duration-200 hover:bg-[var(--surface-hover)]"
          rows="3"
        ></textarea>
        <button
          class="w-full px-4 py-2 rounded-lg bg-blue-500 text-white hover:bg-blue-600 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed transform hover:scale-[1.02] active:scale-[0.98] disabled:transform-none shadow-md hover:shadow-lg"
          disabled={!selectedRecipient || !composeMessage.trim()}
          on:click={startNewConversation}
        >
          Send Message
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  textarea {
    min-height: 40px;
    max-height: 120px;
  }

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