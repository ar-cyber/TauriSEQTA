<script lang="ts">
import { createEventDispatcher } from 'svelte';
import type { Subject, Folder } from '../types';

export let search: string = '';
export let loading: boolean = false;
export let error: string | null = null;
export let activeSubjects: Subject[] = [];
export let otherFolders: Folder[] = [];
export let selectedSubject: Subject | null = null;
export let expandedFolders: Record<string, boolean> = {};
export let subjectMatches: (subj: Subject) => boolean;
export let folderMatches: (folder: Folder) => boolean;

const dispatch = createEventDispatcher<{
  selectSubject: Subject;
  toggleFolder: string;
}>();
</script>

<aside class="flex flex-col w-80 h-full border-r bg-white/80 dark:bg-slate-800/50 backdrop-blur-sm border-gray-300/50 dark:border-slate-700/50">
  <div class="px-4 py-3 border-b border-gray-300/50 dark:border-slate-700/50">
    <div class="relative mb-4 animate-pulse-subtle">
      <input
        type="text"
        placeholder="Search subjects..."
        class="w-full px-4 py-2 bg-white dark:bg-slate-800 border border-gray-300 dark:border-slate-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 dark:focus:ring-indigo-400 transition-all duration-200"
        bind:value={search}
      />
      <svg class="w-5 h-5 absolute right-3 top-1/2 transform -translate-y-1/2 text-gray-400 dark:text-slate-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
      </svg>
    </div>

    {#if search}
      <div class="mb-4 animate-fade-in">
        <h3 class="text-sm font-medium text-gray-500 dark:text-slate-400 mb-2">Search Results</h3>
        {#if activeSubjects.filter(subjectMatches).length === 0}
          <p class="text-sm text-gray-500 dark:text-slate-400 animate-fade-in">No subjects found</p>
        {:else}
          <div class="space-y-2">
            {#each activeSubjects.filter(subjectMatches) as subject, i}
              <button
                class="w-full p-3 text-left bg-white dark:bg-slate-800 rounded-lg border border-gray-200 dark:border-slate-700 hover:border-indigo-500 dark:hover:border-indigo-400 transition-all duration-200 animate-slide-in"
                style="animation-delay: {i * 50}ms"
                on:click={() => dispatch('selectSubject', subject)}
              >
                <div class="flex items-center">
                  <span class="text-2xl mr-3">{subject.icon}</span>
                  <div>
                    <h4 class="font-medium text-gray-900 dark:text-white">{subject.title}</h4>
                    <p class="text-sm text-gray-500 dark:text-slate-400">{subject.description}</p>
                  </div>
                </div>
              </button>
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  </div>
  <div class="overflow-y-auto flex-1">
    {#if loading}
      <div class="px-6 py-6 text-gray-600 dark:text-slate-400 flex items-center justify-center">
        <div class="w-6 h-6 border-2 border-accent/30 border-t-accent rounded-full animate-spin mr-3"></div>
        Loading…
      </div>
    {:else if error}
      <div class="px-6 py-6 text-red-400 animate-fade-in">⚠️ {error}</div>
    {:else}
      {#each activeSubjects.filter(subjectMatches) as subj, i}
        <button 
          class="px-6 py-3 w-full text-left font-bold text-base text-gray-900 dark:text-white hover:bg-white/50 dark:hover:bg-slate-800/50 cursor-pointer border-l-2 border-transparent hover:accent-border transition-all duration-200 hover:translate-x-1 {selectedSubject && selectedSubject.classunit === subj.classunit ? 'bg-white/50 dark:bg-slate-800/50 accent-border' : ''}"
          style="animation: fadeIn 0.3s ease-out {i * 0.05}s both;"
          on:click={() => dispatch('selectSubject', subj)}>
          {subj.title}
        </button>
      {/each}
      <div class="my-2 border-t border-gray-300/50 dark:border-slate-700/50"></div>
      {#each otherFolders.filter(folderMatches) as folder, i}
        <div style="animation: fadeIn 0.3s ease-out {i * 0.05}s both;">
          <button 
            class="flex justify-between items-center px-6 py-3 w-full border-l-2 border-transparent transition-all duration-200 cursor-pointer text-gray-900 dark:text-white hover:bg-white/50 dark:hover:bg-slate-800/50 hover:accent-border hover:translate-x-1"
            on:click={() => dispatch('toggleFolder', folder.code)}>
            <span class="text-base font-bold">{folder.code}</span>
            <svg 
              class="ml-2 w-4 h-4 transition-transform duration-300 text-gray-600 dark:text-slate-400" 
              fill="none" 
              stroke="currentColor" 
              stroke-width="2" 
              viewBox="0 0 24 24" 
              style="transform: rotate({expandedFolders[folder.code] ? 90 : 0}deg)">
              <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
            </svg>
          </button>
          {#if expandedFolders[folder.code]}
            <div class="animate-expand">
              {#each folder.subjects.filter(subjectMatches) as subj, j}
                <button 
                  class="pl-10 pr-6 py-2 font-medium text-sm text-gray-900 dark:text-white hover:bg-white/50 dark:hover:bg-slate-800/50 cursor-pointer border-l-2 border-transparent hover:accent-border transition-all duration-200 hover:translate-x-1 {selectedSubject && selectedSubject.classunit === subj.classunit ? 'bg-white/50 dark:bg-slate-800/50 accent-border' : ''}"
                  style="animation: fadeIn 0.2s ease-out {j * 0.05}s both;"
                  on:click={() => dispatch('selectSubject', subj)}>
                  {subj.title}
                </button>
              {/each}
            </div>
          {/if}
        </div>
      {/each}
    {/if}
  </div>
</aside>

<style>
  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @keyframes pulseSubtle {
    0%, 100% {
      transform: scale(1);
    }
    50% {
      transform: scale(1.01);
    }
  }

  .animate-fade-in {
    opacity: 0;
    animation: fadeIn 0.3s ease-out forwards;
  }

  .animate-slide-in {
    opacity: 0;
    animation: slideIn 0.3s ease-out forwards;
  }

  .animate-bounce {
    animation: bounce 0.8s infinite;
  }

  .animate-pulse-subtle {
    animation: pulseSubtle 2s ease-in-out infinite;
  }

  @keyframes bounce {
    0%, 100% {
      transform: translateY(0);
    }
    50% {
      transform: translateY(-5px);
    }
  }

  .animate-expand {
    animation: expand 0.3s ease-out;
  }

  @keyframes expand {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style> 