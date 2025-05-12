<script lang="ts">
import { onMount } from 'svelte';
import { seqtaFetch } from '../../utils/seqtaFetch';

const studentId = 69;

let weekStart: Date = getMonday(new Date());
let lessons = $state<any[]>([]);
let lessonColours = $state<any[]>([]);
let loadingLessons = $state<boolean>(true);

const days = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri'];
const dayLabels = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday'];
const timeSlots = [
  '08:50', '09:30', '10:10',
  '10:35', '11:15', '11:55',
  '12:00', '12:40', '13:20',
  '14:05', '14:45', '15:25'
];

function getMonday(d: Date) {
  d = new Date(d);
  const day = d.getDay(), diff = d.getDate() - day + (day === 0 ? -6 : 1);
  d.setDate(diff);
  d.setHours(0,0,0,0);
  return d;
}

function formatDate(date: Date): string {
  const y = date.getFullYear();
  const m = (date.getMonth() + 1).toString().padStart(2, '0');
  const d = date.getDate().toString().padStart(2, '0');
  return `${y}-${m}-${d}`;
}

async function loadLessonColours() {
  if (lessonColours.length) return lessonColours;
  const res = await seqtaFetch('/seqta/student/load/prefs?', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json; charset=utf-8' },
    body: { request: 'userPrefs', asArray: true, user: studentId }
  });
  lessonColours = JSON.parse(res).payload;
  return lessonColours;
}

async function loadLessons() {
  loadingLessons = true;
  const from = formatDate(weekStart);
  const until = formatDate(new Date(weekStart.getTime() + 4 * 86400000));
  const res = await seqtaFetch('/seqta/student/load/timetable?', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: { from, until, student: studentId }
  });
  const colours = await loadLessonColours();
  lessons = JSON.parse(res).payload.items.map((lesson: any) => {
    const colourPrefName = `timetable.subject.colour.${lesson.code}`;
    const subjectColour = colours.find((c: any) => c.name === colourPrefName);
    lesson.colour = subjectColour ? `${subjectColour.value}` : `#232428`;
    lesson.from = lesson.from.substring(0, 5);
    lesson.until = lesson.until.substring(0, 5);
    lesson.dayIdx = (new Date(lesson.date).getDay() + 6) % 7; // 0=Mon, 4=Fri
    return lesson;
  });
  loadingLessons = false;
}

function prevWeek() {
  weekStart = new Date(weekStart.valueOf() - 7 * 86400000);
  loadLessons();
}
function nextWeek() {
  weekStart = new Date(weekStart.valueOf() + 7 * 86400000);
  loadLessons();
}

function getLessonsFor(dayIdx: number, slot: string) {
  return lessons.filter(l => l.dayIdx === dayIdx && l.from === slot);
}

function weekRangeLabel() {
  const end = new Date(weekStart.valueOf() + 4 * 86400000);
  return `${weekStart.getDate()} ${weekStart.toLocaleString('default', { month: 'short' })} - ${end.getDate()} ${end.toLocaleString('default', { month: 'short' })} ${weekStart.getFullYear()}`;
}

onMount(loadLessons);
</script>

<div class="flex flex-col h-screen w-full bg-[var(--surface)] text-[var(--text)]">
  <div class="flex items-center justify-between px-8 py-4 bg-[var(--surface-alt)] shadow sticky top-0 z-10">
    <div class="flex items-center gap-2">
      <button class="w-10 h-10 flex items-center justify-center rounded-full hover:bg-[var(--background)]" on:click={prevWeek}>&#60;</button>
      <span class="text-xl font-bold">{weekRangeLabel()}</span>
      <button class="w-10 h-10 flex items-center justify-center rounded-full hover:bg-[var(--background)]" on:click={nextWeek}>&#62;</button>
    </div>
    <span class="text-lg font-semibold">Cardijn College Timetable</span>
  </div>

  <div class="flex-1 flex items-stretch overflow-auto h-0">
    <div class="flex-1 w-full h-full flex flex-col justify-stretch">
      <div class="grid grid-cols-[80px_repeat(5,1fr)] w-full" style="border-bottom: 2px solid var(--surface-alt);">
        <div class="bg-[var(--surface-alt)] w-20"></div>
        {#each dayLabels as day}
          <div class="py-2 px-4 text-center font-bold bg-[var(--surface-alt)] border-l border-[var(--surface)]">{day.toUpperCase()}</div>
        {/each}
      </div>
      <div class="grid grid-cols-[80px_repeat(5,1fr)] w-full flex-1 h-full grid-rows-[repeat(12,minmax(0,1fr))]" style="height:100%">
        {#each timeSlots as slot, slotIdx}
          <div class="contents">
            <div class="flex items-center justify-center py-4 px-2 text-center font-mono border-t border-[var(--surface-alt)] bg-[var(--surface)] h-full w-20 min-w-0">{slot}</div>
            {#each Array(5) as _, dayIdx}
              <div class="relative border-t border-l border-[var(--surface-alt)] bg-[var(--surface)] h-full flex items-center justify-center">
                {#each getLessonsFor(dayIdx, slot) as lesson}
                  <div class="rounded-lg shadow p-2 text-xs flex flex-col gap-1 w-full h-full items-center justify-center" style="background:{lesson.colour}; color:#fff;">
                    <div class="font-bold text-sm text-center">{lesson.description}</div>
                    <div class="text-center">{lesson.staff}</div>
                    <div class="opacity-80 text-center">{lesson.room}</div>
                    <div class="opacity-70 text-[10px] text-center">{lesson.from}–{lesson.until}</div>
                  </div>
                {/each}
              </div>
            {/each}
          </div>
        {/each}
      </div>
    </div>
    {#if loadingLessons}
      <div class="absolute inset-0 flex items-center justify-center"><span class="text-lg text-[var(--text-muted)]">Loading…</span></div>
    {/if}
  </div>
</div> 