<script lang="ts">
  const { lesson } = $props<{
    lesson: {
      description: string;
      staff: string;
      room: string;
      attendanceTitle?: string;
      from: string;
      until: string;
      colour: string;
    };
  }>();
</script>

<div
  class="flex absolute right-1 left-1 flex-col justify-center p-2 bg-white rounded-lg border-l-4 shadow-sm dark:bg-slate-800"
  style="border-color: {lesson.colour};">
  <span class="text-sm font-bold truncate">{lesson.description}</span>
  <span class="text-xs truncate text-slate-600 dark:text-slate-400">{lesson.staff}</span>
  <span class="text-xs truncate text-slate-600 dark:text-slate-400">{lesson.room}</span>
  {#if lesson.attendanceTitle && lesson.attendanceTitle.trim()}
    <span class="text-xs italic truncate text-slate-600 dark:text-slate-400">{lesson.attendanceTitle}</span>
  {/if}
  <span class="mt-1 font-mono text-xs">{lesson.from} - {lesson.until}</span>
</div> 