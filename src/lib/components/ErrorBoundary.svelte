<script lang="ts">
  import { errorService } from '../services/errorService';
  import { Icon } from 'svelte-hero-icons';
  import { ExclamationTriangle, ArrowPath } from 'svelte-hero-icons';

  let { children, fallback } = $props<{
    children: any;
    fallback?: (error: Error, retry: () => void) => any;
  }>();
  let hasError = $state(false);
  let errorMessage = $state('');
  let error: Error | null = $state(null);

  // This is a simple error boundary - in a real implementation,
  // you might want to use a more sophisticated approach
  function handleError(err: Error) {
    hasError = true;
    errorMessage = err?.message || 'An error occurred in this component';
    error = err;
    
    // Log the error
    console.error('Error caught by ErrorBoundary:', err);
    
    // Report to error service
    errorService.handleManualError(err);
  }

  function retry() {
    hasError = false;
    errorMessage = '';
    error = null;
  }

  // Expose error handling to parent components
  $effect(() => {
    if (children && typeof children === 'function') {
      try {
        children();
      } catch (err) {
        handleError(err as Error);
      }
    }
  });
</script>

{#if hasError}
  {#if fallback}
    {@render fallback(error!, retry)}
  {:else}
    <div class="p-4 bg-red-500/10 border border-red-500/20 rounded-lg">
      <div class="flex items-center gap-3 mb-3">
        <Icon src={ExclamationTriangle} size="20" class="text-red-400" />
        <h3 class="text-sm font-semibold text-red-400">Component Error</h3>
      </div>
      
      <p class="text-sm text-gray-300 mb-3">{errorMessage}</p>
      
          <button
      onclick={retry}
      class="px-3 py-1 bg-red-500/20 text-red-400 rounded text-sm transition-all duration-200 transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 focus:ring-offset-gray-900"
    >
        <Icon src={ArrowPath} size="14" class="inline mr-1" />
        Retry
      </button>
    </div>
  {/if}
{:else}
  {@render children()}
{/if} 