<script lang="ts">
  import { onMount } from 'svelte';
  import { themeService, type ThemeManifest } from '$lib/services/themeService';
  import { loadAndApplyTheme, currentTheme } from '$lib/stores/theme';
  import { get } from 'svelte/store';

  let availableThemes: ThemeManifest[] = [];
  let selectedTheme: ThemeManifest | null = null;
  let loading = true;
  let error: string | null = null;
  let currentThemeName = 'default';
  let imgErrors: boolean[] = [];

  // Load all themes from static/themes/*
  async function loadThemes() {
    loading = true;
    error = null;
    try {
      // List all theme directories manually (since getAvailableThemes is hardcoded)
      const themeNames = [
        'default', 'sunset', 'light', 'mint', 'grape', 'midnight', 'bubblegum', 'solarized', 'glass', 'aero'
      ];
      const themePromises = themeNames.map(async (name) => {
        try {
          return await themeService.getThemeManifest(name);
        } catch {
          return null;
        }
      });
      const themes = await Promise.all(themePromises);
      availableThemes = themes.filter((t): t is ThemeManifest => t !== null);
    } catch (e) {
      error = 'Failed to load themes.';
    } finally {
      loading = false;
    }
  }

  async function loadCurrentTheme() {
    try {
      currentThemeName = get(currentTheme);
    } catch {
      currentThemeName = 'default';
    }
  }

  onMount(async () => {
    await loadThemes();
    await loadCurrentTheme();
    currentTheme.subscribe((val) => {
      currentThemeName = val;
    });
  });

  async function handleApplyTheme(themeName: string) {
    await loadAndApplyTheme(themeName);
    selectedTheme = null;
  }

  function openThemeModal(theme: ThemeManifest) {
    selectedTheme = theme;
  }
  function closeThemeModal() {
    selectedTheme = null;
  }

  function getThemePreviewStyle(theme: ThemeManifest) {
    if (theme.features.glassmorphism) {
      return `backdrop-filter: blur(8px); background: ${theme.customProperties.backgroundColor}`;
    }
    if (theme.features.gradients && theme.customProperties.primaryColor && theme.customProperties.secondaryColor) {
      return `background: linear-gradient(135deg, ${theme.customProperties.primaryColor} 0%, ${theme.customProperties.secondaryColor} 100%)`;
    }
    return `background: ${theme.customProperties.backgroundColor}`;
  }
</script>

<div class="p-8 max-w-5xl mx-auto">
  <div class="flex justify-between items-center mb-8">
    <h1 class="text-3xl font-bold">Theme Store</h1>
    <div class="text-sm text-slate-600 dark:text-slate-400">Current: {currentThemeName}</div>
  </div>

  {#if loading}
    <div class="flex justify-center items-center py-16">
      <div class="flex flex-col gap-4 items-center">
        <div class="w-8 h-8 rounded-full border-4 animate-spin border-accent/30 border-t-accent"></div>
        <p class="text-sm text-slate-600 dark:text-slate-400">Loading themes...</p>
      </div>
    </div>
  {:else if error}
    <div class="text-red-500 text-center py-8">{error}</div>
  {:else}
    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
      {#each availableThemes as theme, i (theme.name)}
        <div
          class="rounded-xl shadow-lg p-6 flex flex-col items-center bg-white/10 dark:bg-slate-900/80 border border-slate-200 dark:border-slate-700 transition-all duration-200 hover:scale-105 hover:shadow-2xl cursor-pointer {currentThemeName === theme.name.toLowerCase() ? 'ring-2 ring-accent' : ''}"
          on:click={() => openThemeModal(theme)}
        >
          {#if !imgErrors[i]}
            <img
              src={theme.preview.thumbnail}
              alt={theme.name + ' preview'}
              class="w-16 h-16 rounded-full mb-4 border-2 object-cover"
              style="border-color: {theme.customProperties.primaryColor}; {getThemePreviewStyle(theme)}"
              on:error={() => imgErrors[i] = true}
            />
          {:else}
            <div class="w-16 h-16 rounded-full mb-4 border-2 flex items-center justify-center" style="border-color: {theme.customProperties.primaryColor}; {getThemePreviewStyle(theme)}">
              <span class="text-xs text-slate-400">No Image</span>
            </div>
          {/if}
          <div class="font-semibold text-lg mb-2">{theme.name}</div>
          <div class="text-sm text-slate-500 dark:text-slate-400 mb-4 text-center">{theme.description}</div>
          <div class="flex gap-2 items-center text-xs text-slate-400">
            <span>v{theme.version}</span>
            <span>•</span>
            <span>by {theme.author}</span>
          </div>
          {#if currentThemeName === theme.name.toLowerCase()}
            <div class="mt-2 px-2 py-1 text-xs bg-accent text-white rounded-full">Active</div>
          {/if}
        </div>
      {/each}
    </div>

    {#if selectedTheme}
      <div class="fixed inset-0 z-50 bg-black/50 flex items-center justify-center">
        <div
          class="bg-white dark:bg-slate-900 rounded-2xl shadow-2xl w-full max-w-2xl max-h-[90vh] overflow-hidden animate-fade-in-up relative"
          style="{getThemePreviewStyle(selectedTheme)}"
        >
          <button
            class="absolute top-4 right-4 text-xl text-white bg-black/30 rounded-full w-8 h-8 flex items-center justify-center hover:bg-black/60 transition"
            on:click={closeThemeModal}
          >
            ×
          </button>
          <div class="flex flex-col h-full">
            <div class="flex h-64">
              <div
                class="w-20 flex flex-col items-center py-4"
                style="background: {selectedTheme.customProperties.primaryColor}"
              >
                <div class="w-8 h-8 rounded-lg bg-white/80 mb-4"></div>
                <div class="w-8 h-8 rounded-lg bg-white/60 mb-4"></div>
                <div class="w-8 h-8 rounded-lg bg-white/40 mb-4"></div>
                <div class="w-8 h-8 rounded-lg bg-white/20"></div>
              </div>
              <div class="flex-1 flex flex-col">
                <div
                  class="h-12 flex items-center px-6 text-white"
                  style="background: {selectedTheme.customProperties.primaryColor}"
                >
                  <span class="font-bold text-lg">DesQTA</span>
                  <span class="ml-auto text-sm opacity-80">User</span>
                </div>
                <div class="flex-1 bg-white/80 dark:bg-slate-900/80 p-6 flex flex-col gap-4">
                  <div class="h-6 w-1/3 rounded bg-slate-300 dark:bg-slate-700 mb-2"></div>
                  <div class="h-4 w-2/3 rounded bg-slate-200 dark:bg-slate-800 mb-2"></div>
                  <div class="h-4 w-1/2 rounded bg-slate-200 dark:bg-slate-800 mb-2"></div>
                  <div class="h-32 rounded-lg bg-slate-100 dark:bg-slate-800"></div>
                </div>
              </div>
            </div>
            <div class="p-6 flex flex-col items-center">
              <div class="font-semibold text-lg mb-2">{selectedTheme.name} Theme</div>
              <div class="text-sm text-slate-700 dark:text-slate-300 mb-4 text-center">{selectedTheme.description}</div>
              <div class="flex gap-4">
                <button
                  class="px-4 py-2 rounded-lg font-semibold text-white transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-accent accent-bg hover:accent-bg-hover active:scale-95"
                  on:click={() => handleApplyTheme(selectedTheme.name.toLowerCase())}
                >
                  Apply Theme
                </button>
                {#if currentThemeName !== 'default'}
                  <button
                    class="px-4 py-2 rounded-lg font-semibold text-slate-700 dark:text-slate-300 bg-slate-200 dark:bg-slate-700 transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-slate-400 hover:bg-slate-300 dark:hover:bg-slate-600 active:scale-95"
                    on:click={() => handleApplyTheme('default')}
                  >
                    Reset to Default
                  </button>
                {/if}
              </div>
            </div>
          </div>
        </div>
      </div>
    {/if}
  {/if}
</div>

<style>
  @keyframes fade-in-up {
    0% { opacity: 0; transform: translateY(32px); }
    100% { opacity: 1; transform: translateY(0); }
  }
  .animate-fade-in-up {
    animation: fade-in-up 0.7s cubic-bezier(0.22, 1, 0.36, 1);
  }
</style> 