<script lang="ts">
import type { LinkPreview } from '../types';

export let url: string;
export let preview: LinkPreview | null = null;
</script>

<div class="mb-4 p-4 bg-slate-900 rounded-xl border border-slate-700 hover:border-indigo-500 transition-colors">
  {#if preview}
    <div class="flex flex-col">
      {#if preview.image}
        <img src={preview.image} alt={preview.title} class="w-full h-48 object-cover rounded-lg mb-4" />
      {/if}
      <div class="flex-1">
        <h4 class="text-lg font-semibold text-white mb-2">{preview.title}</h4>
        {#if preview.description}
          <p class="text-slate-300 text-sm mb-3 line-clamp-3">{preview.description}</p>
        {/if}
        <a 
          href={url} 
          target="_blank" 
          rel="noopener noreferrer"
          class="inline-flex items-center text-indigo-400 hover:text-purple-300 font-medium text-sm transition-colors">
          <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"/>
          </svg>
          Visit Link
        </a>
      </div>
    </div>
  {:else}
    <!-- Fallback when no preview is available -->
    <div class="flex items-center">
      <div class="flex-shrink-0 w-12 h-12 bg-gradient-to-r from-indigo-600 to-purple-700 rounded-lg flex items-center justify-center mr-4">
        <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"/>
        </svg>
      </div>
      <div class="flex-1 min-w-0">
        <a 
          href={url} 
          target="_blank" 
          rel="noopener noreferrer"
          class="text-indigo-400 hover:text-purple-300 font-medium transition-colors break-all">
          {url}
        </a>
        <p class="text-slate-400 text-sm mt-1">External Link</p>
      </div>
    </div>
  {/if}
</div> 