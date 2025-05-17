<script lang="ts">
  import { Icon } from 'svelte-hero-icons';
  import { Plus, Inbox, PaperAirplane, Trash, Star } from 'svelte-hero-icons';
  import type { Folder } from '../types';

  let { selectedFolder, openFolder, openCompose } = $props<{
    selectedFolder: Folder;
    openFolder: (folder: Folder) => void;
    openCompose: () => void;
  }>();

  // Folder definitions
  const folders = [
    { name: 'Inbox' as Folder, icon: Inbox },
    { name: 'Sent' as Folder, icon: PaperAirplane },
    { name: 'Starred' as Folder, icon: Star },
    { name: 'Trash' as Folder, icon: Trash }
  ];
</script>

<aside class="flex flex-col border-r xl:w-64 border-slate-800">
  <div class="p-4 border-b border-slate-800">
    <button class="flex gap-2 items-center px-4 py-2 w-full text-base font-semibold text-white bg-blue-500 rounded-xl shadow-md transition-all duration-200 hover:bg-blue-600 focus:ring-2 focus:ring-blue-400 hover:shadow-lg" onclick={openCompose}>
      <Icon src={Plus} class="w-5 h-5" />
      <span>Compose</span>
    </button>
  </div>
  <nav class="flex flex-col flex-1 gap-1 px-2 py-4">
    {#each folders as folder}
      <button
        class="w-full flex items-center gap-3 px-6 py-2 text-left font-medium rounded-lg transition-all duration-200 relative group
          {selectedFolder === folder.name ? 'bg-slate-800 text-blue-500 border-l-4 border-blue-500 pl-[1.25rem]' : 'border-l-4 border-transparent'}
          hover:bg-slate-800 focus:outline-none focus:ring-2 focus:ring-blue-400"
        onclick={() => openFolder(folder.name)}
      >
        <Icon src={folder.icon} class="w-5 h-5" />
        <span>{folder.name}</span>
      </button>
    {/each}
  </nav>
</aside> 