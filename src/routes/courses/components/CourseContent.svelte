<script lang="ts">
import { 
  renderDraftJSText, 
  sanitizeHtml, 
  getFileIcon, 
  getFileColor, 
  formatFileSize,
  fetchLinkPreview,
  isValidUrl
} from '../utils';
import LinkPreview from './LinkPreview.svelte';
import type { 
  CoursePayload, 
  ParsedDocument, 
  WeeklyLessonContent,
  Module,
  TitleModule,
  TextBlockModule,
  ResourceModule,
  LinkModule,
  ResourceLink,
  LinkPreview as LinkPreviewType
} from '../types';

let { 
  coursePayload, 
  parsedDocument = null, 
  selectedLessonContent = null, 
  showingOverview = true 
}: {
  coursePayload: CoursePayload;
  parsedDocument?: ParsedDocument | null;
  selectedLessonContent?: WeeklyLessonContent | null;
  showingOverview?: boolean;
} = $props();

let linkPreviews: Map<string, LinkPreviewType | null> = $state(new Map());

function isModule<T extends Module>(module: Module, contentCheck: (content: any) => boolean): module is T {
  return 'content' in module && contentCheck(module.content);
}

function isTitleModule(module: Module): module is TitleModule {
  return isModule(module, (content) => content && typeof content.value === 'string');
}

function isTextBlockModule(module: Module): module is TextBlockModule {
  return isModule(module, (content) => content && content.content && content.content.blocks);
}

function isResourceModule(module: Module): module is ResourceModule {
  return isModule(module, (content) => content && content.value && content.value.resources);
}

function isLinkModule(module: Module): module is LinkModule {
  return isModule(module, (content) => content && content.url);
}

async function loadLinkPreview(url: string) {
  if (!linkPreviews.has(url)) {
    linkPreviews.set(url, null);
    const preview = await fetchLinkPreview(url);
    linkPreviews.set(url, preview);
    linkPreviews = linkPreviews;
  }
}

type RenderedModule = 
  | { type: 'title'; content: string }
  | { type: 'text'; content: string }
  | { type: 'resources'; content: ResourceLink[] }
  | { type: 'link'; content: string };

function renderModule(module: Module): RenderedModule | null {
  if (isTitleModule(module)) {
    return { type: 'title', content: module.content.value };
  } else if (isTextBlockModule(module)) {
    return { type: 'text', content: renderDraftJSText(module.content.content) };
  } else if (isResourceModule(module)) {
    return { 
      type: 'resources', 
      content: module.content.value.resources.filter(r => r.selected)
    };
  } else if (isLinkModule(module)) {
    const url = module.content.url;
    if (isValidUrl(url)) {
      loadLinkPreview(url);
    }
    return { type: 'link', content: url };
  }
  return null;
}

function parseLessonDocument(lessonContent: WeeklyLessonContent) {
  if (!lessonContent.document) return null;
  try {
    return JSON.parse(lessonContent.document.contents);
  } catch (error) {
    console.error('Failed to parse lesson document:', error);
    return null;
  }
}

function sortModules(modules: Module[]): Module[] {
  if (!modules || modules.length === 0) return [];
  
  const moduleMap = new Map<string, Module>();
  modules.forEach(module => {
    moduleMap.set(module.uuid, module);
  });
  
  const firstModule = modules.find(module => 
    !module.prevModule || !moduleMap.has(module.prevModule)
  );
  
  if (!firstModule) {
    return modules;
  }
  
  const orderedModules: Module[] = [];
  let currentModule: Module | undefined = firstModule;
  
  while (currentModule && orderedModules.length < modules.length) {
    orderedModules.push(currentModule);
    currentModule = currentModule.nextModule ? moduleMap.get(currentModule.nextModule) : undefined;
  }
  
  modules.forEach(module => {
    if (!orderedModules.includes(module)) {
      orderedModules.push(module);
    }
  });
  
  return orderedModules;
}
</script>

<div class="overflow-y-auto relative flex-1">
  {#if !showingOverview && selectedLessonContent}
    <div class="p-6">
      <h1 class="p-6 mb-6 text-3xl font-bold text-white accent-bg rounded-xl backdrop-blur-sm border border-gray-300/50 dark:border-slate-700/50">
        {selectedLessonContent.t}
      </h1>
      
      {#if selectedLessonContent.h}
        <div class="p-4 mb-4 rounded-xl border backdrop-blur-sm bg-white/80 dark:bg-slate-900/50 border-gray-300/50 dark:border-slate-800/50">
          <h3 class="mb-2 text-lg font-semibold text-gray-900 dark:text-white">Homework/Notes</h3>
          <div class="max-w-none prose prose-gray dark:prose-invert prose-indigo">
            <p class="text-gray-700 dark:text-slate-300">{selectedLessonContent.h}</p>
          </div>
        </div>
      {/if}

      {#if selectedLessonContent.r && selectedLessonContent.r.length > 0}
        <div class="mb-6">
          <h3 class="p-4 mb-4 text-xl font-bold text-white accent-bg rounded-xl backdrop-blur-sm border border-gray-300/50 dark:border-slate-700/50">
            Lesson Resources
          </h3>
          <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
            {#each selectedLessonContent.r as resource}
              <div class="p-4 rounded-xl border backdrop-blur-sm bg-white/80 dark:bg-slate-900/50 border-gray-300/50 dark:border-slate-800/50 transition-all hover:scale-[1.02] hover:shadow-lg">
                <div class="flex items-center mb-2">
                  <span class="mr-3 text-2xl">{getFileIcon(resource.mimetype)}</span>
                  <div class="flex-1 min-w-0">
                    <div class="font-semibold text-gray-900 truncate dark:text-white">{resource.t}</div>
                    <div class="text-xs text-gray-600 dark:text-slate-400">{formatFileSize(resource.size)}</div>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      {#if selectedLessonContent.document}
        {@const lessonDoc = parseLessonDocument(selectedLessonContent)}
        {#if lessonDoc?.document?.modules}
          {@const sortedModules = sortModules(lessonDoc.document.modules)}
          <div class="max-w-none prose prose-gray dark:prose-invert prose-indigo">
            {#each sortedModules as module}
              {@const renderedModule = renderModule(module)}
              {#if renderedModule}
                {#if renderedModule.type === 'title'}
                  <h2 class="p-4 my-4 text-xl font-bold text-white accent-bg rounded-xl border border-gray-300/50 dark:border-slate-700/50">
                    {renderedModule.content}
                  </h2>
                {:else if renderedModule.type === 'text'}
                  <div class="p-4 my-4 rounded-xl border backdrop-blur-sm bg-white/80 dark:bg-slate-800/50 border-gray-300/50 dark:border-slate-700/50">
                    {@html sanitizeHtml(renderedModule.content)}
                  </div>
                {:else if renderedModule.type === 'link'}
                  <LinkPreview url={renderedModule.content} preview={linkPreviews.get(renderedModule.content)} />
                {/if}
              {/if}
            {/each}
          </div>
        {/if}
      {/if}
    </div>
  {:else}
    <div class="p-6">
      <h1 class="p-6 mb-6 text-3xl font-bold text-white accent-bg rounded-xl backdrop-blur-sm border border-gray-300/50 dark:border-slate-700/50">
        {coursePayload.t}
      </h1>

      {#if parsedDocument?.document?.modules}
        {@const sortedModules = sortModules(parsedDocument.document.modules)}
        <div class="max-w-none prose prose-gray dark:prose-invert prose-indigo">
          {#each sortedModules as module}
            {@const renderedModule = renderModule(module)}
            {#if renderedModule}
              {#if renderedModule.type === 'title'}
                <h2 class="p-4 mb-4 text-xl font-bold text-white accent-bg rounded-xl backdrop-blur-sm border border-gray-300/50 dark:border-slate-700/50">
                  {renderedModule.content}
                </h2>
              {:else if renderedModule.type === 'text'}
                <div class="p-4 mb-6 rounded-xl border backdrop-blur-sm bg-white/80 dark:bg-slate-800/50 border-gray-300/50 dark:border-slate-700/50">
                  {@html sanitizeHtml(renderedModule.content)}
                </div>
              {:else if renderedModule.type === 'resources'}
                <div class="mb-6">
                  <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
                    {#each renderedModule.content as resource}
                      {@const fileDetails = coursePayload.cf.find(f => f.uuid === resource.uuid)}
                      {#if fileDetails}
                        <div class="p-4 rounded-xl border backdrop-blur-sm bg-white/80 dark:bg-slate-900/50 border-gray-300/50 dark:border-slate-800/50 transition-all hover:scale-[1.02] hover:shadow-lg">
                          <div class="flex items-center mb-2">
                            <span class="mr-3 text-2xl">{getFileIcon(fileDetails.mimetype)}</span>
                            <div class="flex-1 min-w-0">
                              <div class="font-semibold text-gray-900 truncate dark:text-white">{fileDetails.filename}</div>
                              <div class="text-xs text-gray-600 dark:text-slate-400">{formatFileSize(fileDetails.size)}</div>
                            </div>
                          </div>
                        </div>
                      {/if}
                    {/each}
                  </div>
                </div>
              {:else if renderedModule.type === 'link'}
                <LinkPreview url={renderedModule.content} preview={linkPreviews.get(renderedModule.content)} />
              {/if}
            {/if}
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div> 