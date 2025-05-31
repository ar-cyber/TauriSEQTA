<script lang="ts">
import { createEventDispatcher } from 'svelte';
import { formatDate, formatTime } from '../utils';
import type { TermSchedule, Lesson } from '../types';

export let schedule: TermSchedule[] = [];
export let selectedLesson: Lesson | null = null;

const dispatch = createEventDispatcher<{
  selectLesson: {
    termSchedule: TermSchedule;
    lesson: Lesson;
    lessonIndex: number;
  };
}>();
</script>

<aside class="flex flex-col w-64 h-full border-r bg-slate-950 border-slate-800">
  <div class="px-4 py-3 border-b border-slate-800">
    <h3 class="text-lg font-bold text-white">Schedule</h3>
  </div>
  <div class="overflow-y-auto flex-1">
    {#each schedule as termSchedule}
      <div class="mb-4">
        <div class="px-4 py-2 bg-gradient-to-r from-indigo-600 to-purple-700 text-sm font-semibold text-white">
          Term {termSchedule.t} - Week {termSchedule.w}
        </div>
        {#each termSchedule.l as lesson, lessonIndex}
          <button 
            class="w-full px-4 py-2 text-left text-sm hover:bg-slate-800 border-l-2 border-transparent hover:border-indigo-500 transition-all {selectedLesson === lesson ? 'bg-slate-800 border-indigo-500' : ''}"
            on:click={() => dispatch('selectLesson', { termSchedule, lesson, lessonIndex })}>
            <div class="font-medium text-slate-200">{lesson.p}</div>
            <div class="text-xs text-slate-400">
              {formatDate(lesson.d)} • {formatTime(lesson.s)}-{formatTime(lesson.e)}
            </div>
          </button>
        {/each}
      </div>
    {/each}
  </div>
</aside> 