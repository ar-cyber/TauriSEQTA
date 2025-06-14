<script lang="ts">
  import { Window } from '@tauri-apps/api/window';
  import { Icon, Minus, Square2Stack, XMark } from 'svelte-hero-icons';
  import { theme } from '../stores/theme';

  const appWindow = Window.getCurrent();
  let isMaximized = $state(false);

  async function minimize() {
    await appWindow.minimize();
  }

  async function maximize() {
    await appWindow.toggleMaximize();
    isMaximized = !isMaximized;
  }

  async function close() {
    await appWindow.close();
  }

  // Listen for window state changes
  appWindow.onResized(() => {
    appWindow.isMaximized().then((maximized: boolean) => {
      isMaximized = maximized;
    });
  });
</script>

<div
  class="flex justify-between items-center h-12 px-4 bg-white/90 dark:bg-slate-900/90 backdrop-blur-sm border-b border-slate-200 dark:border-slate-800"
  data-tauri-drag-region>
  <div class="flex items-center space-x-2" data-tauri-drag-region>
    <img src="/betterseqta-dark-icon.png" alt="DesQTA" class="w-6 h-6 invert dark:invert-0" />
    <span class="text-sm font-medium text-slate-900 dark:text-white">DesQTA</span>
  </div>

  <div class="flex items-center space-x-2" data-tauri-drag-region>
    <button
      class="flex justify-center items-center w-8 h-8 rounded-lg transition-all duration-200 hover:bg-slate-100 dark:hover:bg-slate-800 focus:outline-none focus:ring-2 accent-ring"
      onclick={minimize}
      aria-label="Minimize">
      <Icon src={Minus} class="w-4 h-4 text-slate-600 dark:text-slate-400" />
    </button>
    <button
      class="flex justify-center items-center w-8 h-8 rounded-lg transition-all duration-200 hover:bg-slate-100 dark:hover:bg-slate-800 focus:outline-none focus:ring-2 accent-ring"
      onclick={maximize}
      aria-label="Maximize">
      <Icon src={Square2Stack} class="w-4 h-4 text-slate-600 dark:text-slate-400" />
    </button>
    <button
      class="flex justify-center items-center w-8 h-8 rounded-lg transition-all duration-200 hover:bg-red-500 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2"
      onclick={close}
      aria-label="Close">
      <Icon src={XMark} class="w-4 h-4 text-slate-600 dark:text-slate-400 group-hover:text-white" />
    </button>
  </div>
</div> 