<script lang="ts">
  import Placeholder from '@tiptap/extension-placeholder';
  import Commands from './Plugins/Commands/command';
  import { Dropcursor } from "@tiptap/extension-dropcursor";
  import BubbleMenu from '@tiptap/extension-bubble-menu';
  import Typography from '@tiptap/extension-typography';
  import TaskList from '@tiptap/extension-task-list';
  import TaskItem from '@tiptap/extension-task-item';
  import StarterKit from '@tiptap/starter-kit';
  import Link from '@tiptap/extension-link';
  
  import { browser } from '$app/environment';
  import { Editor } from '@tiptap/core';
  
  import CommandList from './Plugins/Commands/CommandList.svelte';
  import suggestion from './Plugins/Commands/suggestion';
  import { slashVisible } from './Plugins/Commands/stores';
  import { get } from 'svelte/store';

  import { onMount, onDestroy } from 'svelte';
  import './EditorStyles.css';

  // Make htmlContent bindable from parent components
  let { htmlContent = $bindable('') } = $props();
  
  let output = $state(null);
  let outputType = $state('');
  let commandListInstance = $state<any>(null);

  let element = $state<HTMLElement | null>(null);
  let editor = $state<Editor | null>(null);
  let w = $state(0);

  onMount(() => {
    if (browser) {
      editor = new Editor({
        element: element!,
        editorProps: {
          attributes: {
            class: 'focus:outline-none flex flex-col items-center px-3 md:px-0'
          },
          handleKeyDown: (view, event) => {
            // Handle keyboard events when slash menu is visible
            if (get(slashVisible) && commandListInstance) {
              if (event.key === 'Enter') {
                const handled = commandListInstance.handleKeydown(event, editor);
                if (handled) {
                  return true; // Prevent TipTap from handling this event
                }
              }
            }
            return false; // Let TipTap handle other events
          }
        },
        extensions: [
          StarterKit,
          Placeholder.configure({
            placeholder: ({ node }: { node: any }) => {
              if (node.type.name === 'heading') {
                return 'Heading'
              } else if (node.type.name === 'paragraph') {
                return "Type '/' for commands"
              }
                
              return 'Type something...';
            },
          }),
          TaskList,
          TaskItem,
          Link,
          Typography,
          Commands.configure({
            suggestion
          }),
          BubbleMenu.configure({
            element: document.querySelector('.menu') as HTMLElement,
          }),
          Dropcursor.configure({ width: 5, color: "#ddeeff" }),
        ],
        onTransaction: () => {
          // force re-render so `editor.isActive` works as expected
          editor = editor;
        },
        onUpdate: ({ editor }: { editor: Editor }) => {
          // Update the htmlContent with the editor's HTML
          htmlContent = editor.getHTML();
        }
      });
    }
  });

  onDestroy(() => {
    if (editor) {
      editor.destroy();
    }
  });
  
  function handleKeydownCapture(event: KeyboardEvent) {
    if (commandListInstance && editor && get(slashVisible)) {
      if (commandListInstance.handleKeydown(event, editor)) {
        event.preventDefault();
        event.stopPropagation();
      }
    }
  }
  
  function showJsonOutput() {
    if (editor) {
      output = editor.getJSON() as any;
      outputType = 'json';
    }
  }
  
  function clearOutput() {
    output = null;
  }
  
  function copyOutput() {
    if (output) {
      navigator.clipboard.writeText(JSON.stringify(output));
    }
  }
</script>

<div class="prose prose-slate prose-invert sm:prose-xl lg:prose-3xl" bind:clientWidth={w}>
  <div 
    class="w-full" 
    bind:this={element} 
    onkeydown={handleKeydownCapture}
    role="textbox"
    tabindex="-1"
  ></div>
</div>

<CommandList bind:this={commandListInstance} />

<div class="my-4 sm:flex">
  <button
    onclick={showJsonOutput}
    class="m-2 border rounded-full px-4 py-2 border-slate-500 {outputType == 'json'
      ? 'bg-blue-200'
      : ''}">See JSON Output</button
  >
</div>

{#if output}
  <div class="flex-row-reverse sm:flex">
    <button
      class="p-2 font-semibold underline text-slate-700 hover:text-slate-800 cursor"
      onclick={clearOutput}
    >
      Clear output
    </button>
    <button
      class="p-2 font-semibold underline text-slate-700 hover:text-slate-800 cursor"
      onclick={copyOutput}
    >
      Copy output
    </button>
  </div>
  {JSON.stringify(output)}
{/if}
<hr />
