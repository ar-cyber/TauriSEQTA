<script lang="ts">
import { onMount } from 'svelte';
import { invoke } from '@tauri-apps/api/core';
import { notify } from '../../utils/notify';

interface Shortcut {
  name: string;
  icon: string;
  url: string;
}

let shortcuts: Shortcut[] = [];
let loading = true;
let saving = false;
let saveSuccess = false;
let saveError = '';
let weatherEnabled = false;
let weatherLocation = '';
let remindersEnabled = true;

async function loadSettings() {
  loading = true;
  try {
    const settings = await invoke<{ shortcuts: Shortcut[], weather_enabled: boolean, weather_location: string, reminders_enabled: boolean }>('get_settings');
    shortcuts = settings.shortcuts || [];
    weatherEnabled = settings.weather_enabled ?? false;
    weatherLocation = settings.weather_location ?? '';
    remindersEnabled = settings.reminders_enabled ?? true;
  } catch (e) {
    shortcuts = [];
    weatherEnabled = false;
    weatherLocation = '';
    remindersEnabled = true;
  }
  loading = false;
}

async function saveSettings() {
  saving = true;
  saveSuccess = false;
  saveError = '';
  try {
    await invoke('save_settings', { newSettings: { shortcuts, weather_enabled: weatherEnabled, weather_location: weatherLocation, reminders_enabled: remindersEnabled } });
    saveSuccess = true;
  } catch (e) {
    saveError = 'Failed to save settings.';
  }
  saving = false;
}

function addShortcut() {
  shortcuts = [...shortcuts, { name: '', icon: '', url: '' }];
}

function removeShortcut(idx: number) {
  shortcuts = shortcuts.filter((_, i) => i !== idx);
}

async function sendTestNotification() {
  if (!remindersEnabled) {
    alert('Reminders are currently disabled. Enable them to receive notifications.');
    return;
  }
  await notify({
    title: 'Test Notification',
    body: 'This is a test notification from DesQTA settings.',
  });
}

onMount(loadSettings);
</script>

<div class="max-w-4xl mx-auto p-8">
  <div class="flex justify-between items-center mb-8 animate-fade-in-up">
    <h1 class="text-2xl font-bold">Settings</h1>
    <div class="flex gap-2 items-center">
      <button 
        class="px-6 py-2 rounded bg-green-600 text-white hover:bg-green-700 focus:ring-2 focus:ring-green-400 transition-transform duration-200 active:scale-95 hover:scale-105 shadow-lg"
        on:click={saveSettings} 
        disabled={saving}
      >
        Save Changes
      </button>
      {#if saveSuccess}
        <span class="text-green-400 animate-fade-in">Settings saved successfully!</span>
      {/if}
      {#if saveError}
        <span class="text-red-400 animate-fade-in">{saveError}</span>
      {/if}
    </div>
  </div>

  {#if loading}
    <div class="flex justify-center items-center py-12 animate-fade-in">
      <p class="text-slate-400">Loading settings...</p>
    </div>
  {:else}
    <div class="space-y-8">
      <!-- Homepage Settings -->
      <section class="bg-slate-900 rounded-2xl shadow-xl border border-slate-800 overflow-hidden transition-all duration-300 hover:shadow-2xl hover:border-blue-700 animate-fade-in-up">
        <div class="px-6 py-4 border-b border-slate-800">
          <h2 class="text-lg font-semibold">Homepage</h2>
          <p class="text-sm text-slate-400">Customize your homepage experience</p>
        </div>
        <div class="p-6 space-y-6">
          <!-- Shortcuts -->
          <div>
            <h3 class="text-base font-semibold mb-4">Quick Access Shortcuts</h3>
            <p class="mb-4 text-sm text-slate-400">Add shortcuts to frequently used websites that will appear at the top of your homepage.</p>
            <div class="space-y-4">
              {#each shortcuts as shortcut, idx}
                <div class="flex gap-2 items-center bg-slate-800 rounded-lg p-3 transition-all duration-200 hover:shadow-lg hover:bg-slate-700 animate-fade-in">
                  <input class="w-24 px-2 py-1 rounded bg-slate-900 focus:ring-2 focus:ring-blue-500 transition" placeholder="Name" bind:value={shortcut.name} />
                  <input class="w-16 px-2 py-1 rounded bg-slate-900 focus:ring-2 focus:ring-blue-500 transition" placeholder="Icon (emoji)" bind:value={shortcut.icon} />
                  <input class="flex-1 px-2 py-1 rounded bg-slate-900 focus:ring-2 focus:ring-blue-500 transition" placeholder="URL" bind:value={shortcut.url} />
                  <button class="text-red-400 hover:text-red-600 px-2 transition-transform duration-200 active:scale-110" on:click={() => removeShortcut(idx)} title="Remove">✕</button>
                </div>
              {/each}
              <button class="px-4 py-2 rounded bg-blue-600 text-white hover:bg-blue-700 focus:ring-2 focus:ring-blue-400 transition-transform duration-200 active:scale-95 hover:scale-105 shadow" on:click={addShortcut}>Add Shortcut</button>
            </div>
          </div>
          <!-- Widget Settings (Placeholder) -->
          <div>
            <h3 class="text-base font-semibold mb-4">Widget Settings</h3>
            <p class="mb-4 text-sm text-slate-400">Configure which widgets appear on your in DesQTA.</p>
            <div class="p-4 bg-slate-800 rounded-lg animate-fade-in space-y-4">
              <div class="flex items-center gap-4">
                <input id="weather-enabled" type="checkbox" class="accent-blue-600 w-5 h-5" bind:checked={weatherEnabled} />
                <label for="weather-enabled" class="text-slate-200 font-medium cursor-pointer">Show Weather Widget</label>
              </div>
              <div class="flex items-center gap-4 pl-1" style="opacity: {weatherEnabled ? 1 : 0.5}; pointer-events: {weatherEnabled ? 'auto' : 'none'};">
                <label for="weather-location" class="text-slate-400">Location (city or postcode):</label>
                <input id="weather-location" class="px-3 py-2 rounded bg-slate-900 text-white border border-slate-700 focus:outline-none focus:ring-2 focus:ring-blue-500 transition w-64" placeholder="e.g. Perth, 6000" bind:value={weatherLocation} />
              </div>
            </div>
          </div>
        </div>
      </section>
      <!-- Appearance Settings -->
      <section class="bg-slate-900 rounded-2xl shadow-xl border border-slate-800 overflow-hidden transition-all duration-300 hover:shadow-2xl hover:border-blue-700 animate-fade-in-up delay-100">
        <div class="px-6 py-4 border-b border-slate-800">
          <h2 class="text-lg font-semibold">Appearance</h2>
          <p class="text-sm text-slate-400">Customize how DesQTA looks</p>
        </div>
        <div class="p-6 space-y-6">
          <!-- Theme Settings (Placeholder) -->
          <div>
            <h3 class="text-base font-semibold mb-4">Theme</h3>
            <p class="mb-4 text-sm text-slate-400">Choose your preferred color scheme and theme settings.</p>
            <div class="p-4 bg-slate-800 rounded-lg animate-fade-in">
              <p class="text-slate-400 text-sm">Theme settings coming soon...</p>
            </div>
          </div>
          <!-- Layout Settings (Placeholder) -->
          <div>
            <h3 class="text-base font-semibold mb-4">Layout</h3>
            <p class="mb-4 text-sm text-slate-400">Adjust the layout and sizing of various elements.</p>
            <div class="p-4 bg-slate-800 rounded-lg animate-fade-in">
              <p class="text-slate-400 text-sm">Layout settings coming soon...</p>
            </div>
          </div>
        </div>
      </section>
      <!-- Notification Settings -->
      <section class="bg-slate-900 rounded-2xl shadow-xl border border-slate-800 overflow-hidden transition-all duration-300 hover:shadow-2xl hover:border-blue-700 animate-fade-in-up delay-200">
        <div class="px-6 py-4 border-b border-slate-800">
          <h2 class="text-lg font-semibold">Notifications</h2>
          <p class="text-sm text-slate-400">Manage your notification preferences</p>
        </div>
        <div class="p-6">
          <div class="p-4 bg-slate-800 rounded-lg animate-fade-in flex flex-col gap-4">
            <div class="flex items-center gap-3 mb-4">
              <input id="reminders-enabled" type="checkbox" class="accent-blue-600 w-5 h-5" bind:checked={remindersEnabled} />
              <label for="reminders-enabled" class="text-slate-200 font-medium cursor-pointer">Enable assessment reminder notifications</label>
            </div>
            <button class="px-4 py-2 rounded bg-blue-600 text-white hover:bg-blue-700 focus:ring-2 focus:ring-blue-400 transition-transform duration-200 active:scale-95 hover:scale-105 shadow w-fit" on:click={sendTestNotification}>
              Send Test Notification
            </button>
          </div>
        </div>
      </section>
    </div>
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
.animate-fade-in {
  animation: fade-in 0.5s cubic-bezier(0.22, 1, 0.36, 1);
}
@keyframes fade-in {
  from { opacity: 0; }
  to { opacity: 1; }
}
</style> 