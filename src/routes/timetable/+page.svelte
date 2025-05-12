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

function getLessonsFor(dayIdx: number) {
  return lessons.filter(l => l.dayIdx === dayIdx).sort((a, b) => a.from.localeCompare(b.from));
}

function weekRangeLabel() {
  const end = new Date(weekStart.valueOf() + 4 * 86400000);
  return `${weekStart.getDate()} ${weekStart.toLocaleString('default', { month: 'short' })} - ${end.getDate()} ${end.toLocaleString('default', { month: 'short' })} ${weekStart.getFullYear()}`;
}

function hexToRgb(hex: string) {
  // Remove '#' if present
  hex = hex.replace(/^#/, '');
  if (hex.length === 3) {
    hex = hex.split('').map(x => x + x).join('');
  }
  const num = parseInt(hex, 16);
  return [
    (num >> 16) & 255,
    (num >> 8) & 255,
    num & 255
  ];
}

function isColorLight(hex: string) {
  const [r, g, b] = hexToRgb(hex);
  // Perceived brightness formula
  return (r * 299 + g * 587 + b * 114) / 1000 > 150;
}

function getTextColor(bg: string) {
  // Remove 'var(...)' and fallback to white if not a hex
  if (!bg.startsWith('#')) return '#fff';
  return isColorLight(bg) ? '#111' : '#fff';
}

function getUniqueTimes() {
  // Get all unique lesson start times for the week, sorted
  const times = Array.from(new Set(lessons.map(l => l.from)));
  return times.sort((a, b) => a.localeCompare(b));
}

function getLessonsAt(dayIdx: number, time: string) {
  return lessons.filter(l => l.dayIdx === dayIdx && l.from === time);
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
  </div>

  <div class="flex-1 flex items-stretch overflow-auto h-0">
    <div class="flex-1 w-full h-full flex flex-col justify-stretch">
      <div class="grid grid-cols-[80px_repeat(5,1fr)] w-full" style="border-bottom: 2px solid var(--surface-alt);">
        <div class="bg-[var(--surface-alt)] w-20"></div>
        {#each dayLabels as day}
          <div class="py-2 px-4 text-center font-bold bg-[var(--surface-alt)] border-l border-[var(--surface)]">{day.toUpperCase()}</div>
        {/each}
      </div>
      <div class="grid w-full flex-1 h-full" style="grid-template-columns: 80px repeat(5, 1fr); height:100%">
        {#each getUniqueTimes() as time}
          <div class="flex items-center justify-center py-4 px-2 text-center font-mono border-t border-[var(--surface-alt)] bg-[var(--surface)] h-full w-20 min-w-0">{time}</div>
          {#each Array(5) as _, dayIdx}
            <div class="m-2 flex flex-col gap-2 h-full">
              {#each getLessonsAt(dayIdx, time) as lesson}
                <div class="rounded-lg shadow p-2 text-xs flex flex-col gap-1 items-center justify-center min-w-0 h-full" style="background:{lesson.colour}; color:{getTextColor(lesson.colour)};">
                  <div class="font-bold text-sm text-center">{lesson.description}</div>
                  <div class="text-center">{lesson.staff}</div>
                  <div class="opacity-80 text-center">{lesson.room}</div>
                  <div class="opacity-70 text-[10px] text-center">{lesson.from}–{lesson.until}</div>
                </div>
              {/each}
              {#if getLessonsAt(dayIdx, time).length === 0}
                <div></div>
              {/if}
            </div>
          {/each}
        {/each}
      </div>
    </div>
    {#if loadingLessons}
      <div class="absolute inset-0 flex items-center justify-center"><span class="text-lg text-[var(--text-muted)]">Loading…</span></div>
    {/if}
  </div>
</div> 