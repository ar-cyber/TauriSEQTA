<script lang="ts">
import { createEventDispatcher } from 'svelte';
import { formatDate, formatTime } from '../utils';
import type { TermSchedule, Lesson } from '../types';

type ClosestType = { termSchedule: TermSchedule; lesson: Lesson; lessonIndex: number; diff: number };

export let schedule: TermSchedule[] = [];
export let selectedLesson: Lesson | null = null;

const dispatch = createEventDispatcher<{
  selectLesson: {
    termSchedule: TermSchedule;
    lesson: Lesson;
    lessonIndex: number;
  };
}>();

function goToToday() {
  const now = new Date();
  let closest: ClosestType | null = null;
  schedule.forEach((termSchedule: TermSchedule) => {
    termSchedule.l.forEach((lesson: Lesson, lessonIndex: number) => {
      const lessonDate = new Date(lesson.d);
      const diff = lessonDate.getTime() - now.getTime();
      if (!closest || (diff >= 0 && (closest.diff < 0 || diff < closest.diff)) || (diff < 0 && closest.diff < 0 && diff > closest.diff)) {
        closest = { termSchedule, lesson, lessonIndex, diff } as ClosestType;
      }
    });
  });
  if (closest !== null) {
    const payload = {
      termSchedule: (closest as ClosestType).termSchedule,
      lesson: (closest as ClosestType).lesson,
      lessonIndex: (closest as ClosestType).lessonIndex
    };
    dispatch('selectLesson', payload);
  }
}
</script>

<aside class="flex flex-col w-64 h-full border-r bg-slate-950/50 backdrop-blur-xl border-slate-800/50">
  <div class="px-4 py-3 border-b border-slate-800/50">
    <h3 class="text-lg font-bold text-white">Schedule</h3>
    <button class="mt-2 w-full py-2 rounded-lg bg-gradient-to-r from-indigo-500/80 to-pink-500/80 text-white font-semibold shadow hover:opacity-90 transition-all" on:click={goToToday}>
      Today
    </button>
  </div>
  <div class="overflow-y-auto flex-1">
    {#each schedule as termSchedule}
      <div class="mb-4">
        <div class="px-4 py-2 bg-gradient-to-r from-indigo-600/80 to-purple-700/80 backdrop-blur-sm text-sm font-semibold text-white">
          Term {termSchedule.t} - Week {termSchedule.w}
        </div>
        {#each termSchedule.l as lesson, lessonIndex}
          <button 
            class="w-full px-4 py-2 text-left text-sm hover:bg-slate-800/50 border-l-2 border-transparent hover:border-indigo-500/50 transition-all {selectedLesson === lesson ? 'bg-slate-800/50 border-indigo-500/50' : ''}"
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