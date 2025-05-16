<script lang="ts">
import { onMount } from 'svelte';
import { seqtaFetch } from '../../utils/seqtaFetch';

interface Subject {
  code: string;
  classunit: number;
  description: string;
  metaclass: number;
  title: string;
  programme: number;
  marksbook_type: string;
}
interface Folder {
  code: string;
  subjects: Subject[];
  description: string;
  active?: number;
  id: number;
}

let folders: Folder[] = [];
let activeSubjects: Subject[] = [];
let otherFolders: Folder[] = [];
let loading = true;
let error: string | null = null;

let expandedFolders: Record<string, boolean> = {};
let selectedSubject: Subject | null = null;
let search = '';

async function loadSubjects() {
  loading = true;
  error = null;
  try {
    const res = await seqtaFetch('/seqta/student/load/subjects?', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json; charset=utf-8' },
      body: {}
    });
    const data = JSON.parse(res);
    folders = data.payload;
    const activeFolder = folders.find((f: Folder) => f.active === 1);
    activeSubjects = activeFolder ? activeFolder.subjects : [];
    otherFolders = folders.filter((f: Folder) => f.active !== 1);
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
  } finally {
    loading = false;
  }
}

function toggleFolder(code: string) {
  expandedFolders[code] = !expandedFolders[code];
}

function selectSubject(subject: Subject) {
  selectedSubject = subject;
}

function subjectMatches(subj: Subject) {
  const q = search.trim().toLowerCase();
  return (
    subj.title.toLowerCase().includes(q) ||
    subj.code.toLowerCase().includes(q) ||
    subj.description.toLowerCase().includes(q)
  );
}

function folderMatches(folder: Folder) {
  const q = search.trim().toLowerCase();
  return (
    folder.code.toLowerCase().includes(q) ||
    folder.subjects.some(subjectMatches)
  );
}

onMount(loadSubjects);
</script>

<div class="flex w-full h-full bg-black">
  <aside class="flex flex-col w-80 h-full border-r bg-slate-950 border-slate-800">
    <div class="px-4 py-3 border-b border-slate-800">
      <input
        type="text"
        placeholder="Search subjects..."
        bind:value={search}
        class="px-3 py-2 w-full rounded-lg border bg-slate-800 text-slate-50 border-slate-800 focus:outline-none focus:ring focus:ring-blue-500"
      />
    </div>
    <div class="overflow-y-auto flex-1">
      {#if loading}
        <div class="px-6 py-6 text-slate-400">Loading…</div>
      {:else if error}
        <div class="px-6 py-6 text-red-400">{error}</div>
      {:else}
        {#each activeSubjects.filter(subjectMatches) as subj}
          <button class="px-6 py-3 w-full text-left font-bold text-base hover:bg-slate-800 cursor-pointer border-l-2 border-transparent hover:border-blue-500 transition-all {selectedSubject && selectedSubject.classunit === subj.classunit ? 'bg-slate-800 border-blue-500' : ''}"
            onclick={() => selectSubject(subj)}>
            {subj.title}
          </button>
        {/each}
        <div class="my-2 border-t border-slate-800"></div>
        {#each otherFolders.filter(folderMatches) as folder}
          <div>
            <button class="flex justify-between items-center px-6 py-3 w-full border-l-2 border-transparent transition-all cursor-pointer hover:bg-slate-800 hover:border-blue-500"
              onclick={() => toggleFolder(folder.code)}>
              <span class="text-base font-bold">{folder.code}</span>
              <svg class="ml-2 w-4 h-4 transition-transform duration-200 text-slate-400" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24" style="transform: rotate({expandedFolders[folder.code] ? 90 : 0}deg)">
                <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
              </svg>
          </button>
          {#if expandedFolders[folder.code]}
            {#each folder.subjects.filter(subjectMatches) as subj}
              <button class="pl-10 pr-6 py-2 font-medium text-sm hover:bg-slate-800 cursor-pointer border-l-2 border-transparent hover:border-blue-500 transition-all {selectedSubject && selectedSubject.classunit === subj.classunit ? 'bg-slate-800 border-blue-500' : ''}"
                onclick={() => selectSubject(subj)}>
                {subj.title}
              </button>
            {/each}
          {/if}
          </div>
        {/each}
      {/if}
    </div>
  </aside>
  <div class="flex-1 p-8 bg-black">
    {#if selectedSubject}
      <div class="p-8 mx-auto max-w-xl rounded-xl shadow-lg bg-slate-900">
        <h2 class="mb-2 text-2xl font-bold">{selectedSubject.title}</h2>
        <div class="mb-4 text-slate-400">{selectedSubject.description}</div>
        <div class="grid grid-cols-2 gap-4 text-sm">
          <div><span class="font-semibold">Code:</span> {selectedSubject.code}</div>
          <div><span class="font-semibold">Class Unit:</span> {selectedSubject.classunit}</div>
          <div><span class="font-semibold">Programme:</span> {selectedSubject.programme}</div>
          <div><span class="font-semibold">Metaclass:</span> {selectedSubject.metaclass}</div>
          <div><span class="font-semibold">Marksbook Type:</span> {selectedSubject.marksbook_type}</div>
        </div>
      </div>
    {/if}
  </div>
</div> 