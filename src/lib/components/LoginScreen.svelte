<script lang="ts">
  import { Window } from '@tauri-apps/api/window';
  import { Icon } from 'svelte-hero-icons';
  import { Minus, Square2Stack, XMark } from 'svelte-hero-icons';
  import jsQR from 'jsqr';
  import { authService } from '$lib/services/authService';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { theme } from '$lib/stores/theme';

  interface Props {
    seqtaUrl: string;
    onStartLogin: () => void;
    onUrlChange: (url: string) => void;
  }

  let { seqtaUrl, onStartLogin, onUrlChange }: Props = $props();

  const appWindow = Window.getCurrent();
  
  let qrProcessing = $state(false);
  let qrError = $state('');
  let qrSuccess = $state('');
</script>

<div class="flex flex-col h-full">
  <!-- Window Controls Bar -->
  <div 
    class="flex justify-between items-center px-4 py-2 bg-white/80 dark:bg-slate-900/80 backdrop-blur-sm border-b border-slate-200/60 dark:border-slate-700/60"
    data-tauri-drag-region>
    <!-- Draggable area with branding -->
    <div class="flex items-center space-x-3" data-tauri-drag-region>
      <img src="/betterseqta-dark-icon.png" alt="DesQTA" class="w-6 h-6 invert dark:invert-0" />
      <h1 class="text-lg font-bold text-transparent bg-clip-text bg-gradient-to-r from-slate-900 to-slate-600 dark:from-white dark:to-slate-300">
        DesQTA
      </h1>
    </div>

    <!-- Window Controls -->
    <div class="flex items-center space-x-2" data-tauri-drag-region>
      <button
        class="flex justify-center items-center w-8 h-8 rounded-lg transition-all duration-200 hover:bg-slate-100 dark:hover:bg-slate-800 focus:outline-none focus:ring-2 focus:ring-slate-500 focus:ring-offset-2"
        onclick={() => appWindow.minimize()}
        aria-label="Minimize">
        <Icon src={Minus} class="w-4 h-4 text-slate-600 dark:text-slate-400" />
      </button>
      <button
        class="flex justify-center items-center w-8 h-8 rounded-lg transition-all duration-200 hover:bg-slate-100 dark:hover:bg-slate-800 focus:outline-none focus:ring-2 focus:ring-slate-500 focus:ring-offset-2"
        onclick={() => appWindow.toggleMaximize()}
        aria-label="Maximize">
        <Icon src={Square2Stack} class="w-4 h-4 text-slate-600 dark:text-slate-400" />
      </button>
      <button
        class="flex justify-center items-center w-8 h-8 rounded-lg transition-all duration-200 group hover:bg-red-500 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2"
        onclick={() => appWindow.close()}
        aria-label="Close">
        <Icon src={XMark} class="w-4 h-4 transition duration-200 text-slate-600 dark:text-slate-400 group-hover:text-white" />
      </button>
    </div>
  </div>

  <!-- Login Content -->
  <div class="flex justify-center items-center p-6 flex-1">
    <div
      class="flex overflow-hidden flex-col w-full max-w-6xl rounded-3xl border shadow-2xl backdrop-blur-xl bg-white/80 border-slate-200/60 dark:bg-slate-900/80 dark:border-slate-700/60 md:flex-row animate-fade-in-up"
      style="box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);">
      <!-- Left side - Image and branding -->
      <div class="hidden relative md:block md:w-2/3">
        <div
          class="absolute inset-0 bg-gradient-to-br from-indigo-500/20 to-purple-500/20 dark:from-indigo-500/10 dark:to-purple-500/10">
        </div>
        <img
          src="/images/signin.jpg"
          alt="Sign in"
          class="object-cover w-full h-full min-h-[600px]" />
        <div
          class="flex absolute inset-0 flex-col justify-end p-8 bg-gradient-to-t to-transparent from-black/50">
          <h1 class="mb-4 text-4xl font-bold text-white">Welcome to DesQTA</h1>
          <p class="text-lg text-slate-200">
            Experience SEQTA Learn like never before with our powerful desktop application
          </p>
        </div>
      </div>

      <!-- Right side - Login form -->
      <div class="flex flex-col justify-center p-8 w-full md:w-1/3 md:p-12">
        <div class="mb-8 text-center md:hidden">
          <h1 class="mb-2 text-3xl font-bold text-slate-900 dark:text-white">
            Welcome to DesQTA
          </h1>
          <p class="text-slate-600 dark:text-slate-300">
            Experience SEQTA Learn like never before with our powerful desktop application
          </p>
        </div>

        <div class="space-y-6">
          <div>
            <label
              for="seqta-url"
              class="block mb-2 text-sm font-medium text-slate-700 dark:text-slate-300">
              SEQTA URL
            </label>
            <div class="relative">
              <div
                class="flex absolute inset-y-0 left-0 items-center pl-3 pointer-events-none">
                <svg
                  class="w-5 h-5 text-slate-400"
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 20 20"
                  fill="currentColor">
                  <path
                    fill-rule="evenodd"
                    d="M12.586 4.586a2 2 0 112.828 2.828l-3 3a2 2 0 01-2.828 0 1 1 0 00-1.414 1.414 4 4 0 005.656 0l3-3a4 4 0 00-5.656-5.656l-1.5 1.5a1 1 0 101.414 1.414l1.5-1.5zm-5 5a2 2 0 012.828 0 1 1 0 101.414-1.414 4 4 0 00-5.656 0l-3 3a4 4 0 105.656 5.656l1.5-1.5a1 1 0 10-1.414-1.414l-1.5 1.5a2 2 0 11-2.828-2.828l3-3z"
                    clip-rule="evenodd" />
                </svg>
              </div>
              <input
                id="seqta-url"
                type="text"
                bind:value={seqtaUrl}
                oninput={(e) => onUrlChange((e.target as HTMLInputElement).value)}
                placeholder="your-school.seqta.com.au"
                class="py-3 pr-4 pl-10 w-full rounded-lg border transition-colors text-slate-900 bg-slate-50 border-slate-300 dark:bg-slate-800 dark:text-white dark:border-slate-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent" />
            </div>
            <p class="mt-2 text-sm text-slate-500 dark:text-slate-400">
              Enter your school's SEQTA URL to get started
            </p>
          </div>

          <div>
            <label
              for="seqta-qrcode"
              class="block mb-2 text-sm font-medium text-slate-700 dark:text-slate-300">
              SEQTA QR Code
            </label>
            <div class="relative">
              <div
                class="flex absolute inset-y-0 left-0 items-center pl-3 pointer-events-none">
                <svg
                  class="w-5 h-5 text-slate-400"
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 20 20"
                  fill="currentColor">
                  <path
                    fill-rule="evenodd"
                    d="M12.586 4.586a2 2 0 112.828 2.828l-3 3a2 2 0 01-2.828 0 1 1 0 00-1.414 1.414 4 4 0 005.656 0l3-3a4 4 0 00-5.656-5.656l-1.5 1.5a1 1 0 101.414 1.414l1.5-1.5zm-5 5a2 2 0 012.828 0 1 1 0 101.414-1.414 4 4 0 00-5.656 0l-3 3a4 4 0 105.656 5.656l1.5-1.5a1 1 0 10-1.414-1.414l-1.5 1.5a2 2 0 11-2.828-2.828l3-3z"
                    clip-rule="evenodd" />
                </svg>
              </div>
              <input
                id="seqta-qrcode"
                type="file"
                accept="image/*"
                oninput={async (e) => {
                  const fileInput = e.target as HTMLInputElement;
                  const signinButton = document.getElementById('signin-button') as HTMLButtonElement;
                  
                  console.log('[QR_FRONTEND] File input changed');
                  
                  if (fileInput.files && fileInput.files.length > 0) {
                    const file = fileInput.files[0];
                    console.log('[QR_FRONTEND] File selected:', file.name, 'Size:', file.size, 'Type:', file.type);
                    
                    qrProcessing = true;
                    qrError = '';
                    qrSuccess = '';
                    
                    const reader = new FileReader();
                    
                    reader.onload = (e) => {
                      console.log('[QR_FRONTEND] File read completed');
                      try {
                        const imageData = e.target?.result as ArrayBuffer;
                        console.log('[QR_FRONTEND] Image data size:', imageData.byteLength);
                        
                        // Create a blob URL from the array buffer
                        const blob = new Blob([imageData], { type: file.type });
                        const imageUrl = URL.createObjectURL(blob);
                        
                        // Create an image element to load the image
                        const img = new Image();
                        img.onload = () => {
                          console.log('[QR_FRONTEND] Image loaded, dimensions:', img.width, 'x', img.height);
                          
                          // Create a canvas to get pixel data
                          const canvas = document.createElement('canvas');
                          const ctx = canvas.getContext('2d');
                          
                          if (!ctx) {
                            console.error('[QR_FRONTEND] Could not get canvas context');
                            qrError = 'Could not process image';
                            qrProcessing = false;
                            signinButton.disabled = true;
                            return;
                          }
                          
                          // Set canvas size to image size
                          canvas.width = img.width;
                          canvas.height = img.height;
                          
                          // Draw image to canvas
                          ctx.drawImage(img, 0, 0);
                          
                          // Get pixel data
                          const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
                          console.log('[QR_FRONTEND] Canvas pixel data size:', imageData.data.length);
                          
                          // Decode QR code
                          const code = jsQR(imageData.data, imageData.width, imageData.height);
                          console.log('[QR_FRONTEND] jsQR result:', code ? 'Success' : 'Failed to decode');
                          
                          if (code) {
                            console.log('[QR_FRONTEND] QR code data:', code.data);
                            console.log('[QR_FRONTEND] QR code data length:', code.data.length);
                            
                            if (code.data.startsWith('seqtalearn://')) {
                              console.log('[QR_FRONTEND] Valid SEQTA deeplink detected');
                              seqtaUrl = code.data;
                              qrSuccess = 'QR code processed successfully!';
                              signinButton.disabled = false;
                              console.log('[QR_FRONTEND] Setting seqtaUrl to:', seqtaUrl);
                            } else {
                              console.log('[QR_FRONTEND] Invalid QR code - not a SEQTA deeplink');
                              qrError = 'QR code does not contain a valid SEQTA deeplink';
                              signinButton.disabled = true;
                            }
                          } else {
                            console.log('[QR_FRONTEND] Failed to decode QR code from image');
                            qrError = 'Could not read QR code from image';
                            signinButton.disabled = true;
                          }
                          
                          // Clean up
                          URL.revokeObjectURL(imageUrl);
                          qrProcessing = false;
                          console.log('[QR_FRONTEND] QR processing completed');
                        };
                        
                        img.onerror = () => {
                          console.error('[QR_FRONTEND] Failed to load image');
                          qrError = 'Failed to load image file';
                          qrProcessing = false;
                          signinButton.disabled = true;
                          URL.revokeObjectURL(imageUrl);
                        };
                        
                        // Load the image
                        img.src = imageUrl;
                        
                      } catch (error) {
                        console.error('[QR_FRONTEND] Error processing QR code:', error);
                        qrError = 'Error processing QR code: ' + (error instanceof Error ? error.message : 'Unknown error');
                        qrProcessing = false;
                        signinButton.disabled = true;
                      }
                    };
                    
                    reader.onerror = () => {
                      console.error('[QR_FRONTEND] File read error');
                      qrError = 'Error reading file';
                      qrProcessing = false;
                      signinButton.disabled = true;
                    };
                    
                    console.log('[QR_FRONTEND] Starting file read as ArrayBuffer');
                    reader.readAsArrayBuffer(file);
                  } else {
                    console.log('[QR_FRONTEND] No file selected');
                    signinButton.disabled = true;
                    qrError = '';
                    qrSuccess = '';
                  }
                }}
                class="py-3 pr-4 pl-10 w-full rounded-lg border transition-colors text-slate-900 bg-slate-50 border-slate-300 dark:bg-slate-800 dark:text-white dark:border-slate-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent" />
            </div>
            <p class="mt-2 text-sm text-slate-500 dark:text-slate-400">
              Upload your SEQTA Login QR Code to get started
            </p>
            
            {#if qrProcessing}
              <div class="mt-2 text-sm text-blue-600 dark:text-blue-400">
                Processing QR code...
              </div>
            {/if}
            
            {#if qrSuccess}
              <div class="mt-2 text-sm text-green-600 dark:text-green-400">
                {qrSuccess}
              </div>
            {/if}
            
            {#if qrError}
              <div class="mt-2 text-sm text-red-600 dark:text-red-400">
                {qrError}
              </div>
            {/if}
          </div>

          <button
            id="signin-button"
            class="py-3 w-full text-lg font-semibold text-white bg-gradient-to-r from-indigo-600 to-purple-600 rounded-lg shadow-lg transition-all duration-200 hover:from-indigo-700 hover:to-purple-700 disabled:opacity-60 disabled:cursor-not-allowed"
            onclick={onStartLogin}
            disabled={true}>
            Sign In
          </button>

          <div class="text-center">
            <p class="text-sm text-slate-600 dark:text-slate-400">
              Need help? <a
                href="https://github.com/betterseqta/desqta"
                target="_blank"
                rel="noopener noreferrer"
                class="text-indigo-600 dark:text-indigo-400 hover:underline">Visit GitHub</a>
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>