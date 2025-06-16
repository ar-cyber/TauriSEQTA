<script lang="ts">
  import Modal from '$lib/components/Modal.svelte';
  import {
    Icon,
    BookOpen,
    ClipboardDocumentList,
    CalendarDays,
    ChatBubbleLeftRight,
    Bell,
    DocumentText,
    GlobeAlt,
  } from 'svelte-hero-icons';

  interface Props {
    open: boolean;
    onclose?: () => void;
  }

  let { open = $bindable(false), onclose }: Props = $props();

  function closeModal() {
    onclose?.();
  }
</script>

<Modal bind:open onclose={closeModal} ariaLabel="About Modal">
  <!-- Easter Egg Trigger Area -->
  <div
    class="absolute top-0 right-0 w-16 h-16 opacity-0 cursor-pointer"
    onclick={(e) => {
      e.stopPropagation();
      const audio = new Audio('/sounds/konami.mp3');
      audio.play();
      document.body.classList.add('konami-mode');
      setTimeout(() => document.body.classList.remove('konami-mode'), 5000);
    }}
    onkeydown={(e) => {
      e.stopPropagation();
      if (e.key === 'Enter') {
        const audio = new Audio('/sounds/konami.mp3');
        audio.play();
      }
    }}
    role="button"
    tabindex="0">
  </div>

  <div class="p-4 sm:p-6 md:p-8 max-h-[90vh] overflow-y-auto">
    <div class="flex flex-col sm:flex-row sm:justify-between sm:items-start gap-4 mb-6 sm:mb-8">
      <div class="flex items-center space-x-3 sm:space-x-4">
        <img src="/betterseqta-dark-icon.png" alt="DesQTA" class="invert size-10 sm:size-14 dark:invert-0" />
        <div>
          <h2
            class="text-2xl sm:text-3xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-slate-900 to-slate-600 dark:from-white dark:to-slate-300">
            About DesQTA
          </h2>
          <p class="text-sm sm:text-base text-slate-600 dark:text-slate-400">Your SEQTA Learn Desktop Experience</p>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-1 gap-4 sm:gap-6 items-stretch md:grid-cols-2">
      <div class="flex flex-col gap-4 sm:gap-6 h-full">
        <div
          class="bg-white/60 dark:bg-slate-800/60 backdrop-blur-md rounded-2xl border border-slate-200/40 dark:border-slate-700/40 p-4 sm:p-6 flex-1 flex flex-col justify-between min-h-[140px] sm:min-h-[160px]">
          <h3 class="mb-3 sm:mb-4 text-base sm:text-lg font-semibold text-slate-900 dark:text-white">
            Version Information
          </h3>
          <div class="space-y-2 sm:space-y-3">
            <div class="flex justify-between items-center">
              <span class="text-sm sm:text-base text-slate-600 dark:text-slate-400">Version</span>
              <span class="text-sm sm:text-base font-medium text-slate-900 dark:text-white">1.0.0</span>
            </div>
            <div class="flex justify-between items-center">
              <span class="text-sm sm:text-base text-slate-600 dark:text-slate-400">Build Date</span>
              <span class="text-sm sm:text-base font-medium text-slate-900 dark:text-white"
                >{new Date().toLocaleDateString()}</span>
            </div>
            <div class="flex justify-between items-center">
              <span class="text-sm sm:text-base text-slate-600 dark:text-slate-400">Platform</span>
              <span class="text-sm sm:text-base font-medium text-slate-900 dark:text-white">Desktop</span>
            </div>
          </div>
        </div>
        <div
          class="bg-white/60 dark:bg-slate-800/60 backdrop-blur-md rounded-2xl border border-slate-200/40 dark:border-slate-700/40 p-4 sm:p-6 flex-1 flex flex-col justify-between min-h-[140px] sm:min-h-[160px]">
          <h3 class="mb-3 sm:mb-4 text-base sm:text-lg font-semibold text-slate-900 dark:text-white">Built With</h3>
          <div class="grid grid-cols-2 gap-3 sm:gap-4">
            <div class="flex items-center space-x-2 sm:space-x-3">
              <div
                class="w-8 h-8 sm:w-10 sm:h-10 rounded-xl bg-[#FF3E00] flex items-center justify-center text-white font-bold text-xs sm:text-sm">
                S
              </div>
              <span class="text-sm sm:text-base text-slate-900 dark:text-white">SvelteKit</span>
            </div>
            <div class="flex items-center space-x-2 sm:space-x-3">
              <div
                class="w-8 h-8 sm:w-10 sm:h-10 rounded-xl bg-[#FFC131] flex items-center justify-center text-white font-bold text-xs sm:text-sm">
                T
              </div>
              <span class="text-sm sm:text-base text-slate-900 dark:text-white">Tauri</span>
            </div>
            <div class="flex items-center space-x-2 sm:space-x-3">
              <div
                class="w-8 h-8 sm:w-10 sm:h-10 rounded-xl bg-[#06B6D4] flex items-center justify-center text-white font-bold text-xs sm:text-sm">
                T
              </div>
              <span class="text-sm sm:text-base text-slate-900 dark:text-white">TailwindCSS</span>
            </div>
            <div class="flex items-center space-x-2 sm:space-x-3">
              <div
                class="w-8 h-8 sm:w-10 sm:h-10 rounded-xl bg-[#764ABC] flex items-center justify-center text-white font-bold text-xs sm:text-sm">
                R
              </div>
              <span class="text-sm sm:text-base text-slate-900 dark:text-white">Rust</span>
            </div>
          </div>
        </div>
      </div>
      <div class="flex flex-col gap-4 sm:gap-6 h-full">
        <div
          class="bg-white/60 dark:bg-slate-800/60 backdrop-blur-md rounded-2xl border border-slate-200/40 dark:border-slate-700/40 p-4 sm:p-6 flex-1 flex flex-col justify-between min-h-[140px] sm:min-h-[160px]">
          <h3 class="mb-3 sm:mb-4 text-base sm:text-lg font-semibold text-slate-900 dark:text-white">Features</h3>
          <ul class="overflow-auto pr-2 space-y-2 sm:space-y-3 max-h-32 sm:max-h-40">
            <li class="flex items-center space-x-2 sm:space-x-3">
              <div
                class="flex justify-center items-center w-5 h-5 sm:w-6 sm:h-6 bg-blue-100 rounded-lg dark:bg-blue-900/50">
                <Icon src={BookOpen} class="w-3 h-3 sm:w-4 sm:h-4 text-blue-600 dark:text-blue-400" />
              </div>
              <span class="text-sm sm:text-base text-slate-900 dark:text-white">Course Management</span>
            </li>
            <li class="flex items-center space-x-2 sm:space-x-3">
              <div
                class="flex justify-center items-center w-5 h-5 sm:w-6 sm:h-6 bg-purple-100 rounded-lg dark:bg-purple-900/50">
                <Icon
                  src={ClipboardDocumentList}
                  class="w-3 h-3 sm:w-4 sm:h-4 text-purple-600 dark:text-purple-400" />
              </div>
              <span class="text-sm sm:text-base text-slate-900 dark:text-white">Assessment Tracking</span>
            </li>
            <li class="flex items-center space-x-2 sm:space-x-3">
              <div
                class="flex justify-center items-center w-5 h-5 sm:w-6 sm:h-6 bg-orange-100 rounded-lg dark:bg-orange-900/50">
                <Icon src={CalendarDays} class="w-3 h-3 sm:w-4 sm:h-4 text-orange-600 dark:text-orange-400" />
              </div>
              <span class="text-sm sm:text-base text-slate-900 dark:text-white">Timetable Integration</span>
            </li>
            <li class="flex items-center space-x-2 sm:space-x-3">
              <div
                class="flex justify-center items-center w-5 h-5 sm:w-6 sm:h-6 bg-pink-100 rounded-lg dark:bg-pink-900/50">
                <Icon src={ChatBubbleLeftRight} class="w-3 h-3 sm:w-4 sm:h-4 text-pink-600 dark:text-pink-400" />
              </div>
              <span class="text-sm sm:text-base text-slate-900 dark:text-white">Direqt Messaging</span>
            </li>
            <li class="flex items-center space-x-2 sm:space-x-3">
              <div
                class="flex justify-center items-center w-5 h-5 sm:w-6 sm:h-6 bg-red-100 rounded-lg dark:bg-red-900/50">
                <Icon src={Bell} class="w-3 h-3 sm:w-4 sm:h-4 text-red-600 dark:text-red-400" />
              </div>
              <span class="text-sm sm:text-base text-slate-900 dark:text-white">Real-time Notifications</span>
            </li>
            <li class="flex items-center space-x-2 sm:space-x-3">
              <div
                class="flex justify-center items-center w-5 h-5 sm:w-6 sm:h-6 bg-indigo-100 rounded-lg dark:bg-indigo-900/50">
                <Icon src={DocumentText} class="w-3 h-3 sm:w-4 sm:h-4 text-indigo-600 dark:text-indigo-400" />
              </div>
              <span class="text-sm sm:text-base text-slate-900 dark:text-white">Notice Board</span>
            </li>
          </ul>
        </div>
        <div
          class="bg-white/60 dark:bg-slate-800/60 backdrop-blur-md rounded-2xl border border-slate-200/40 dark:border-slate-700/40 p-4 sm:p-6 flex-1 flex flex-col justify-between min-h-[140px] sm:min-h-[160px]">
          <h3 class="mb-3 sm:mb-4 text-base sm:text-lg font-semibold text-slate-900 dark:text-white">Links</h3>
          <div class="space-y-3 sm:space-y-4">
            <a
              href="https://github.com/betterseqta/desqta"
              target="_blank"
              rel="noopener noreferrer"
              class="flex items-center p-2 sm:p-3 space-x-2 sm:space-x-3 bg-slate-100 rounded-xl transition-all duration-200 hover:bg-slate-200 dark:bg-slate-700/50 dark:hover:bg-slate-700 group">
              <div
                class="flex justify-center items-center w-7 h-7 sm:w-8 sm:h-8 bg-slate-800 rounded-lg dark:bg-slate-200">
                <svg xmlns="http://www.w3.org/2000/svg"  viewBox="0 0 72 72" width="128px" height="128px"><path d="M36,12c13.255,0,24,10.745,24,24c0,10.656-6.948,19.685-16.559,22.818c0.003-0.009,0.007-0.022,0.007-0.022	s-1.62-0.759-1.586-2.114c0.038-1.491,0-4.971,0-6.248c0-2.193-1.388-3.747-1.388-3.747s10.884,0.122,10.884-11.491	c0-4.481-2.342-6.812-2.342-6.812s1.23-4.784-0.426-6.812c-1.856-0.2-5.18,1.774-6.6,2.697c0,0-2.25-0.922-5.991-0.922	c-3.742,0-5.991,0.922-5.991,0.922c-1.419-0.922-4.744-2.897-6.6-2.697c-1.656,2.029-0.426,6.812-0.426,6.812	s-2.342,2.332-2.342,6.812c0,11.613,10.884,11.491,10.884,11.491s-1.097,1.239-1.336,3.061c-0.76,0.258-1.877,0.576-2.78,0.576	c-2.362,0-4.159-2.296-4.817-3.358c-0.649-1.048-1.98-1.927-3.221-1.927c-0.817,0-1.216,0.409-1.216,0.876s1.146,0.793,1.902,1.659	c1.594,1.826,1.565,5.933,7.245,5.933c0.617,0,1.876-0.152,2.823-0.279c-0.006,1.293-0.007,2.657,0.013,3.454	c0.034,1.355-1.586,2.114-1.586,2.114s0.004,0.013,0.007,0.022C18.948,55.685,12,46.656,12,36C12,22.745,22.745,12,36,12z"/></svg>
              </div>
              <div>
                <div class="text-sm sm:text-base font-medium text-slate-900 dark:text-white">GitHub Repository</div>
                <div class="text-xs text-slate-500 dark:text-slate-400">View source code</div>
              </div>
            </a>
            <a
              href="https://desqta.betterseqta.org"
              target="_blank"
              rel="noopener noreferrer"
              class="flex items-center p-2 sm:p-3 space-x-2 sm:space-x-3 bg-slate-100 rounded-xl transition-all duration-200 hover:bg-slate-200 dark:bg-slate-700/50 dark:hover:bg-slate-700 group">
              <div 
                class="flex justify-center items-center w-7 h-7 sm:w-8 sm:h-8 bg-slate-800 rounded-lg dark:bg-slate-200">
                <Icon src={GlobeAlt} class="w-3 h-3 sm:w-4 sm:h-4 text-slate-600 dark:text-slate-400" />
              </div>
              <div>
                <div class="text-sm sm:text-base font-medium text-slate-900 dark:text-white">Website</div>
                <div class="text-xs text-slate-500 dark:text-slate-400">Official website</div>
              </div>
            </a>
          </div>
        </div>
      </div>
    </div>

    <div class="pt-4 sm:pt-6 mt-6 sm:mt-8 border-t border-slate-200/60 dark:border-slate-700/60">
      <p class="text-center text-sm sm:text-base text-slate-600 dark:text-slate-400">
        © 2025 DesQTA. Licensed under MIT License.
      </p>
    </div>
  </div>
</Modal>
