<script lang="ts">
  import TimetableLesson from './TimetableLesson.svelte';

  const {
    lessons,
    selectedDay,
    loadingLessons,
    error,
    onRetry
  } = $props<{
    lessons: any[];
    selectedDay: number;
    loadingLessons: boolean;
    error: string | null;
    onRetry: () => void;
  }>();

  let selectedDayState = $state(selectedDay);

  const dayLabels = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday'];
  const GRID_HEIGHT = 800;

  function getLessonsFor(dayIdx: number) {
    return lessons.filter((l: any) => l.dayIdx === dayIdx).sort((a: any, b: any) => a.from.localeCompare(b.from));
  }

  function getUniqueTimes() {
    const times = Array.from(new Set(lessons.map((l: any) => l.from)));
    return times.sort((a: any, b: any) => (a as string).localeCompare(b as string));
  }

  function timeToMinutes(time: string): number {
    const [h, m] = time.split(':').map(Number);
    return h * 60 + m;
  }

  function getTimeBounds() {
    if (!lessons.length) return { min: 8 * 60, max: 16 * 60 };
    const allTimes = lessons.flatMap((l: any) => [timeToMinutes(l.from), timeToMinutes(l.until)]);
    return {
      min: Math.min(...allTimes),
      max: Math.max(...allTimes),
    };
  }

  function timeToY(time: number, min: number, max: number): number {
    return ((time - min) / (max - min)) * GRID_HEIGHT;
  }

  const timeBounds = $derived(getTimeBounds());

  function prevDay() {
    selectedDayState = selectedDayState === 1 ? 5 : selectedDayState - 1;
  }

  function nextDay() {
    selectedDayState = selectedDayState === 5 ? 1 : selectedDayState + 1;
  }
</script>

<div class="flex overflow-hidden flex-1 items-stretch">
  <div class="flex flex-col flex-1 w-full min-h-0 justify-stretch">
    <!-- Header Row -->
    <div
      class="grid grid-cols-[60px_repeat(5,1fr)] w-full border-b-2 border-slate-300 dark:border-slate-800">
      <div class="w-14 bg-slate-100 dark:bg-slate-800"></div>
      {#each dayLabels as day, index}
        <div
          class="py-3 px-2 text-center font-semibold bg-slate-100 dark:bg-slate-800 border-l border-slate-300 dark:border-slate-900 text-base text-slate-900 dark:text-white/80 {new Date().getDay() ===
          index + (1 % 7)
            ? 'bg-blue-500 text-slate-800 dark:text-white font-bold'
            : ''} hidden sm:block">
          {day}
        </div>
      {/each}
    </div>

    <!-- Mobile Day Navigation -->
    <div
      class="flex justify-between items-center px-4 py-2 border-b border-slate-300 sm:hidden bg-slate-100/50 dark:bg-slate-800/50 dark:border-slate-800">
      <button
        class="flex justify-center items-center w-8 h-8 rounded-full transition-transform duration-300 hover:bg-slate-200 dark:hover:bg-slate-950/40 hover:scale-110 disabled:opacity-50 disabled:cursor-not-allowed"
        on:click={prevDay}
        disabled={loadingLessons}>&#60;</button>
      <span class="text-base font-bold">{dayLabels[selectedDayState - 1].toUpperCase()}</span>
      <button
        class="flex justify-center items-center w-8 h-8 rounded-full transition-transform duration-300 hover:bg-slate-200 dark:hover:bg-slate-950/40 hover:scale-110 disabled:opacity-50 disabled:cursor-not-allowed"
        on:click={nextDay}
        disabled={loadingLessons}>&#62;</button>
    </div>

    <!-- Time grid and lessons -->
    {#if error}
      <div class="flex flex-col justify-center items-center py-16">
        <div
          class="w-16 h-16 rounded-full border-4 animate-spin border-red-500/30 border-t-red-500">
        </div>
        <p class="mt-4 text-red-400">{error}</p>
        <button
          class="px-4 py-2 mt-4 text-sm font-medium bg-red-600 rounded-lg transition-colors hover:bg-red-500"
          on:click={onRetry}>
          Retry
        </button>
      </div>
    {:else if loadingLessons}
      <div class="flex flex-col justify-center items-center py-16">
        <div
          class="w-16 h-16 rounded-full border-4 animate-spin border-indigo-500/30 border-t-indigo-500">
        </div>
        <p class="mt-4 text-slate-600 dark:text-slate-400">Loading timetable...</p>
      </div>
    {:else if lessons.length}
      <div class="overflow-y-auto relative flex-1 w-full min-h-0">
        <div class="relative w-full" style="height: {GRID_HEIGHT}px;">
          <!-- Time labels -->
          <div class="absolute top-0 left-0 z-10 w-14 h-full pointer-events-none">
            {#each getUniqueTimes() as time}
              <div
                class="absolute left-0 w-full border-t border-slate-300 dark:border-slate-800"
                style="top: {timeToY(
                  timeToMinutes(time as string),
                  timeBounds.min,
                  timeBounds.max,
                )}px;">
                <span class="text-xs text-slate-600 dark:text-slate-400">{time}</span>
              </div>
            {/each}
          </div>
          <!-- Day columns -->
          <div class="grid absolute top-0 right-0 left-14 grid-cols-1 h-full sm:grid-cols-5">
            {#each Array(5) as _, dayIdx}
              <div
                class="relative h-full border-l border-slate-300 dark:border-slate-800 {dayIdx +
                  1 !==
                selectedDayState
                  ? 'hidden sm:block'
                  : ''}">
                {#each getLessonsFor(dayIdx) as lesson}
                  <div
                    class="absolute right-1 left-1"
                    style="
                    top: {timeToY(
                      timeToMinutes(lesson.from),
                      timeBounds.min,
                      timeBounds.max,
                    )}px;
                    height: {timeToY(
                      timeToMinutes(lesson.until),
                      timeBounds.min,
                      timeBounds.max,
                    ) -
                      timeToY(timeToMinutes(lesson.from), timeBounds.min, timeBounds.max)}px;
                  ">
                    <TimetableLesson lesson={lesson} />
                  </div>
                {/each}
              </div>
            {/each}
          </div>
        </div>
      </div>
    {:else}
      <div class="flex flex-col justify-center items-center py-16">
        <div
          class="w-20 h-20 flex items-center justify-center rounded-full bg-gradient-to-br from-indigo-500 to-purple-600 text-3xl shadow-[0_0_20px_rgba(99,102,241,0.3)] animate-gradient">
          📚
        </div>
        <p class="mt-4 text-xl text-slate-700 dark:text-slate-300">
          No lessons available for this week.
        </p>
      </div>
    {/if}
  </div>
</div> 