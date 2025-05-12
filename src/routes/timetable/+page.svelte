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
      <button class="w-10 h-10 flex items-center justify-center rounded-full hover:bg-[var(--background)]" onclick={prevWeek}>&#60;</button>
      <span class="text-xl font-bold">{weekRangeLabel()}</span>
      <button class="w-10 h-10 flex items-center justify-center rounded-full hover:bg-[var(--background)]" onclick={nextWeek}>&#62;</button>
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
                <div class="relative flex flex-col min-w-[230px] max-w-[320px] bg-[var(--surface-alt)] rounded-xl shadow-lg border-l-8 p-0" style="border-color: {lesson.colour};">
                  <div class="px-6 pt-4 pb-2 flex flex-col gap-1 flex-1">
                    <div class="flex items-center justify-between">
                      <span class="font-bold text-base" style="color: var(--text);">{lesson.description}</span>
                    </div>
                    <div class="text-sm" style="color: var(--text-muted);">{lesson.staff}</div>
                    <div class="text-xs" style="color: var(--text-muted);">{lesson.room}</div>
                    <div class="mt-2 text-sm font-mono" style="color: var(--text);">{lesson.from} – {lesson.until}</div>
                    {#if lesson.attendanceTitle && lesson.attendanceTitle.trim()}
                      <div class="mt-1 text-xs italic" style="color: var(--text-muted);">{lesson.attendanceTitle}</div>
                    {/if}
                  </div>
                  {#if lesson.programmeID !== 0}
                    <div class="flex gap-2 px-4 pb-3 pt-1">
                      <button class="hover:scale-110 transition-transform" aria-label="View Assessment" onclick={() => (location.href = buildAssessmentURL(lesson.programmeID, lesson.metaID))}>
                        <svg viewBox="0 0 24 24" style="width:22px;height:22px;"><path d="M6 20H13V22H6C4.89 22 4 21.11 4 20V4C4 2.9 4.89 2 6 2H18C19.11 2 20 2.9 20 4V12.54L18.5 11.72L18 12V4H13V12L10.5 9.75L8 12V4H6V20M24 17L18.5 14L13 17L18.5 20L24 17M15 19.09V21.09L18.5 23L22 21.09V19.09L18.5 21L15 19.09Z" fill="currentColor"></path></svg>
                      </button>
                      <button class="hover:scale-110 transition-transform" aria-label="View Course" onclick={() => (location.href = `../#?page=/courses/${lesson.programmeID}:${lesson.metaID}`)}>
                        <svg viewBox="0 0 24 24" style="width:22px;height:22px;"><path d="M19 1L14 6V17L19 12.5V1M21 5V18.5C19.9 18.15 18.7 18 17.5 18C15.8 18 13.35 18.65 12 19.5V6C10.55 4.9 8.45 4.5 6.5 4.5C4.55 4.5 2.45 4.9 1 6V20.65C1 20.9 1.25 21.15 1.5 21.15C1.6 21.15 1.65 21.1 1.75 21.1C3.1 20.45 5.05 20 6.5 20C8.45 20 10.55 20.4 12 21.5C13.35 20.65 15.8 20 17.5 20C19.15 20 20.85 20.3 22.25 21.05C22.35 21.1 22.4 21.1 22.5 21.1C22.75 21.1 23 20.85 23 20.6V6C22.4 5.55 21.75 5.25 21 5M10 18.41C8.75 18.09 7.5 18 6.5 18C5.44 18 4.18 18.19 3 18.5V7.13C3.91 6.73 5.14 6.5 6.5 6.5C7.86 6.5 9.09 6.73 10 7.13V18.41Z" fill="currentColor"></path></svg>
                      </button>
                    </div>
                  {/if}
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