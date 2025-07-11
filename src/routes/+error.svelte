<script lang="ts">
  import { page } from '$app/stores';
  import { Icon } from 'svelte-hero-icons';
  import { ExclamationTriangle, Home, ArrowLeft, ArrowPath } from 'svelte-hero-icons';
  import { goto } from '$app/navigation';
  import { accentColor } from '../lib/stores/theme';

  let { error, status } = $props<{
    error: Error & { status?: number; message?: string };
    status: number;
  }>();

  // Get error details
  let errorMessage = $derived(error?.message || 'An unexpected error occurred');
  let errorStatus = $derived(status || error?.status || 500);
  let isNetworkError = $derived(errorMessage.includes('fetch') || errorMessage.includes('network') || errorMessage.includes('Failed to fetch'));
  let isAuthError = $derived(errorStatus === 401 || errorStatus === 403);
  let isNotFoundError = $derived(errorStatus === 404);
  let isServerError = $derived(errorStatus >= 500);

  // Error type classification
  let errorType = $derived(isAuthError ? 'Authentication Error' :
                 isNotFoundError ? 'Page Not Found' :
                 isNetworkError ? 'Network Error' :
                 isServerError ? 'Server Error' :
                 'Application Error');

  // Error description based on type
  let errorDescription = $derived(isAuthError ? 'You need to log in to access this page.' :
                       isNotFoundError ? 'The page you\'re looking for doesn\'t exist.' :
                       isNetworkError ? 'Unable to connect to the server. Please check your internet connection.' :
                       isServerError ? 'Something went wrong on our end. Please try again later.' :
                       'Something unexpected happened. Please try again.');

  function goHome() {
    goto('/');
  }

  function goBack() {
    if (window.history.length > 1) {
      window.history.back();
    } else {
      goHome();
    }
  }

  function refreshPage() {
    window.location.reload();
  }

  function reportError() {
    // You can implement error reporting here
    console.error('Error details:', {
      status: errorStatus,
      message: errorMessage,
      stack: error?.stack,
      url: $page.url.href
    });
  }
</script>

<svelte:head>
  <title>Error {errorStatus} - DesQTA</title>
</svelte:head>

<div class="min-h-screen bg-gray-900 flex items-center justify-center p-4">
  <div class="max-w-md w-full text-center">
    <!-- Error Icon -->
    <div class="mb-8 flex justify-center">
      <div class="w-24 h-24 rounded-full bg-red-500/20 flex items-center justify-center">
        <Icon 
          src={ExclamationTriangle} 
          size="48" 
          class="text-red-400"
        />
      </div>
    </div>

    <!-- Error Status -->
    <div class="mb-4">
      <h1 class="text-6xl font-bold text-red-400 mb-2">{errorStatus}</h1>
      <h2 class="text-xl font-semibold text-white mb-2">{errorType}</h2>
    </div>

    <!-- Error Description -->
    <div class="mb-8">
      <p class="text-gray-300 leading-relaxed">{errorDescription}</p>
      {#if !isAuthError && !isNotFoundError}
        <p class="text-sm text-gray-400 mt-2">Error: {errorMessage}</p>
      {/if}
    </div>

    <!-- Action Buttons -->
    <div class="space-y-3">
      {#if isAuthError}
        <button
          onclick={goHome}
          class="w-full px-4 py-3 bg-accent-bg text-white rounded-lg font-medium transition-all duration-200 transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 focus:ring-accent-ring focus:ring-offset-2 focus:ring-offset-gray-900"
        >
          <Icon src={Home} size="20" class="inline mr-2" />
          Go to Login
        </button>
      {:else}
        <button
          onclick={goBack}
          class="w-full px-4 py-3 bg-accent-bg text-white rounded-lg font-medium transition-all duration-200 transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 focus:ring-accent-ring focus:ring-offset-2 focus:ring-offset-gray-900"
        >
          <Icon src={ArrowLeft} size="20" class="inline mr-2" />
          Go Back
        </button>
      {/if}

      <button
        onclick={refreshPage}
        class="w-full px-4 py-3 bg-gray-700 text-white rounded-lg font-medium transition-all duration-200 transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2 focus:ring-offset-gray-900"
      >
        <Icon src={ArrowPath} size="20" class="inline mr-2" />
        Try Again
      </button>

      <button
        onclick={goHome}
        class="w-full px-4 py-3 bg-gray-800 text-gray-300 rounded-lg font-medium transition-all duration-200 transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 focus:ring-gray-600 focus:ring-offset-2 focus:ring-offset-gray-900"
      >
        <Icon src={Home} size="20" class="inline mr-2" />
        Go Home
      </button>
    </div>

    <!-- Debug Info (only in development) -->
    {#if import.meta.env.DEV}
      <div class="mt-8 p-4 bg-gray-800 rounded-lg text-left">
        <h3 class="text-sm font-semibold text-gray-300 mb-2">Debug Information:</h3>
        <pre class="text-xs text-gray-400 overflow-auto">{JSON.stringify({
          status: errorStatus,
          message: errorMessage,
          url: $page.url.href,
          timestamp: new Date().toISOString()
        }, null, 2)}</pre>
      </div>
    {/if}
  </div>
</div> 