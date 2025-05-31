<script lang="ts">
import { onMount } from 'svelte';
import { seqtaFetch } from '../../utils/seqtaFetch';
import SubjectSidebar from './components/SubjectSidebar.svelte';
import ScheduleSidebar from './components/ScheduleSidebar.svelte';
import CourseContent from './components/CourseContent.svelte';
import type { 
  Subject, 
  Folder, 
  CoursePayload, 
  ParsedDocument,
  Lesson,
  TermSchedule,
  WeeklyLessonContent 
} from './types';
import { page } from '$app/stores';

let folders: Folder[] = [];
let activeSubjects: Subject[] = [];
let otherFolders: Folder[] = [];
let loading = true;
let error: string | null = null;

let expandedFolders: Record<string, boolean> = {};
let selectedSubject: Subject | null = null;
let coursePayload: CoursePayload | null = null;
let parsedDocument: ParsedDocument | null = null;
let loadingCourse = false;
let courseError: string | null = null;
let search = '';

// Schedule navigation state
let selectedTerm: number | null = null;
let selectedWeek: number | null = null;
let selectedLesson: Lesson | null = null;
let selectedLessonContent: WeeklyLessonContent | null = null;

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

async function loadCourseContent(subject: Subject) {
  loadingCourse = true;
  courseError = null;
  coursePayload = null;
  parsedDocument = null;
  selectedTerm = null;
  selectedWeek = null;
  selectedLesson = null;
  selectedLessonContent = null;
  
  try {
    const res = await seqtaFetch('/seqta/student/load/courses', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json; charset=utf-8' },
      body: {
        programme: subject.programme.toString(),
        metaclass: subject.metaclass.toString()
      }
    });
    const data = JSON.parse(res);
    coursePayload = data.payload;
    
    // Parse the main document JSON string
    if (coursePayload?.document) {
      try {
        parsedDocument = JSON.parse(coursePayload.document);
      } catch (e) {
        console.error('Failed to parse document JSON:', e);
      }
    }
  } catch (e) {
    courseError = e instanceof Error ? e.message : String(e);
  } finally {
    loadingCourse = false;
  }
}

async function selectSubject(subject: Subject) {
  selectedSubject = subject;
  await loadCourseContent(subject);
}

function selectLesson(termSchedule: TermSchedule, lesson: Lesson, lessonIndex: number) {
  selectedTerm = termSchedule.t;
  selectedWeek = termSchedule.w;
  selectedLesson = lesson;
  
  // Find the corresponding weekly lesson content
  if (coursePayload?.w && coursePayload.w[termSchedule.n] && coursePayload.w[termSchedule.n][lessonIndex]) {
    selectedLessonContent = coursePayload.w[termSchedule.n][lessonIndex];
  } else {
    selectedLessonContent = null;
  }
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

// Event handlers with proper typing
function handleSelectSubject(event: CustomEvent<Subject>) {
  selectSubject(event.detail);
}

function handleToggleFolder(event: CustomEvent<string>) {
  expandedFolders[event.detail] = !expandedFolders[event.detail];
}

function handleSelectLesson(event: CustomEvent<{
  termSchedule: TermSchedule;
  lesson: Lesson;
  lessonIndex: number;
}>) {
  selectLesson(event.detail.termSchedule, event.detail.lesson, event.detail.lessonIndex);
}

onMount(async () => {
  await loadSubjects();
  // Auto-select subject and lesson if query params are present
  const url = new URL(window.location.href);
  const code = url.searchParams.get('code');
  const date = url.searchParams.get('date');
  if (code) {
    const subj = activeSubjects.find(s => s.code === code) || otherFolders.flatMap(f => f.subjects).find(s => s.code === code);
    if (subj) {
      await selectSubject(subj);
      if (date && coursePayload && coursePayload.d) {
        // Find the lesson closest to the date
        const targetDate = new Date(date);
        let closest: { termSchedule: TermSchedule; lesson: Lesson; lessonIndex: number; diff: number } | null = null;
        for (const termSchedule of coursePayload.d) {
          termSchedule.l.forEach((lesson: Lesson, lessonIndex: number) => {
            const lessonDate = new Date(lesson.d);
            const diff = Math.abs(lessonDate.getTime() - targetDate.getTime());
            if (!closest || diff < closest.diff) {
              closest = { termSchedule, lesson, lessonIndex, diff };
            }
          });
        }
        if (closest) {
          selectLesson(closest.termSchedule, closest.lesson, closest.lessonIndex);
        }
      }
    }
  }
});
</script>

<div class="flex w-full h-full bg-black">
  <!-- Subject Selection Sidebar -->
  <SubjectSidebar
    bind:search
    {loading}
    {error}
    {activeSubjects}
    {otherFolders}
    {selectedSubject}
    {expandedFolders}
    {subjectMatches}
    {folderMatches}
    on:selectSubject={handleSelectSubject}
    on:toggleFolder={handleToggleFolder}
  />

  <!-- Course Content Area -->
  <div class="flex flex-1 h-full">
    {#if selectedSubject}
      {#if loadingCourse}
        <div class="flex justify-center items-center w-full">
          <div class="text-slate-400 text-lg">Loading course content...</div>
        </div>
      {:else if courseError}
        <div class="flex justify-center items-center w-full">
          <div class="text-red-400 text-lg">Error loading course: {courseError}</div>
        </div>
      {:else if coursePayload}
        <!-- Schedule Navigation -->
        <ScheduleSidebar
          schedule={coursePayload.d}
          {selectedLesson}
          on:selectLesson={handleSelectLesson}
        />

        <!-- Main Content -->
        <CourseContent
          {coursePayload}
          {parsedDocument}
          {selectedLessonContent}
        />
      {:else}
        <div class="flex justify-center items-center w-full">
          <div class="text-slate-400 text-lg">No course content available</div>
        </div>
      {/if}
    {:else}
      <div class="flex justify-center items-center w-full">
        <div class="text-slate-400 text-lg">Select a subject to view course content</div>
      </div>
    {/if}
  </div>
</div>

<style>
  :global(.course-content) {
    @apply w-full h-full;
  }
  
  /* Style the course content to match the screenshot */
  :global(.course-content h1) {
    @apply text-3xl font-bold text-white bg-blue-500 p-6 rounded-t-xl m-4 mb-0;
  }
  
  :global(.course-content h2) {
    @apply text-xl font-bold text-white bg-blue-500 p-4 rounded-xl m-4 mb-2;
  }
  
  :global(.course-content p) {
    @apply text-slate-300 px-4 py-2;
  }
  
  :global(.course-content .section) {
    @apply m-4 bg-slate-900 rounded-xl overflow-hidden;
  }
  
  :global(.course-content a) {
    @apply text-blue-400 hover:text-blue-300 transition-colors;
  }
  
  :global(.course-content img) {
    @apply max-w-full h-auto rounded-lg;
  }
  
  /* File/document styling */
  :global(.course-content .file-item) {
    @apply bg-slate-800 rounded-lg p-4 m-2 hover:bg-slate-700 transition-colors cursor-pointer;
  }
  
  :global(.course-content .file-grid) {
    @apply grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 p-4;
  }
  
  /* Make embedded content responsive */
  :global(.course-content iframe) {
    @apply w-full rounded-lg;
  }
  
  :global(.course-content video) {
    @apply w-full rounded-lg;
  }
</style> 