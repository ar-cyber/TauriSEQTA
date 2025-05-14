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

<div class="flex h-screen w-full bg-black">
  <aside class="w-80 bg-[var(--surface)] text-[var(--text)] flex flex-col h-full border-r border-[var(--surface-alt)]">
    <div class="flex items-center h-12 px-4 border-b border-[var(--surface-alt)] text-xs uppercase tracking-widest text-[var(--text-muted)] sticky top-0 bg-[var(--surface)] z-10">
      <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7" /></svg>
      Courses
    </div>
    <div class="px-4 py-3 border-b border-[var(--surface-alt)]">
      <input
        type="text"
        placeholder="Search subjects..."
        bind:value={search}
        class="w-full px-3 py-2 rounded-lg bg-[var(--surface-alt)] text-[var(--text)] border border-[var(--surface-alt)] focus:outline-none focus:ring focus:ring-blue-500"
      />
    </div>
    <div class="flex-1 overflow-y-auto">
      {#if loading}
        <div class="px-6 py-6 text-[var(--text-muted)]">Loading…</div>
      {:else if error}
        <div class="px-6 py-6 text-red-400">{error}</div>
      {:else}
        {#each activeSubjects.filter(subjectMatches) as subj}
          <div class="px-6 py-3 font-bold text-base hover:bg-[var(--surface-alt)] cursor-pointer border-l-2 border-transparent hover:border-blue-500 transition-all {selectedSubject && selectedSubject.classunit === subj.classunit ? 'bg-[var(--surface-alt)] border-blue-500' : ''}"
            onclick={() => selectSubject(subj)}>
            {subj.title}
          </div>
        {/each}
        <div class="my-2 border-t border-[var(--surface-alt)]"></div>
        {#each otherFolders.filter(folderMatches) as folder}
          <div>
            <div class="px-6 py-3 flex items-center justify-between hover:bg-[var(--surface-alt)] cursor-pointer border-l-2 border-transparent hover:border-blue-500 transition-all"
              onclick={() => toggleFolder(folder.code)}>
              <span class="font-bold text-base">{folder.code}</span>
              <svg class="w-4 h-4 ml-2 text-[var(--text-muted)] transition-transform duration-200" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24" style="transform: rotate({expandedFolders[folder.code] ? 90 : 0}deg)"><path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" /></svg>
            </div>
            {#if expandedFolders[folder.code]}
              {#each folder.subjects.filter(subjectMatches) as subj}
                <div class="pl-10 pr-6 py-2 font-medium text-sm hover:bg-[var(--surface-alt)] cursor-pointer border-l-2 border-transparent hover:border-blue-500 transition-all {selectedSubject && selectedSubject.classunit === subj.classunit ? 'bg-[var(--surface-alt)] border-blue-500' : ''}"
                  onclick={() => selectSubject(subj)}>
                  {subj.title}
                </div>
              {/each}
            {/if}
          </div>
        {/each}
      {/if}
    </div>
  </aside>
  <div class="flex-1 bg-black p-8">
    {#if selectedSubject}
      <div class="bg-[var(--surface)] rounded-xl shadow-lg p-8 max-w-xl mx-auto">
        <h2 class="text-2xl font-bold mb-2">{selectedSubject.title}</h2>
        <div class="text-[var(--text-muted)] mb-4">{selectedSubject.description}</div>
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