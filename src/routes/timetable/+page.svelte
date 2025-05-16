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

function buildAssessmentURL(programmeID: number, metaID: number, itemID?: number) {
  const base = `../#?page=/assessments/${programmeID}:${metaID}`;
  return itemID ? `${base}&item=${itemID}` : base;
}

// Utility: Convert "HH:MM" to minutes since midnight
function timeToMinutes(time: string): number {
  const [h, m] = time.split(":").map(Number);
  return h * 60 + m;
}

// Get min and max lesson times for the week
function getTimeBounds() {
  if (!lessons.length) return { min: 8 * 60, max: 16 * 60 }; // fallback 8am-4pm
  const allTimes = lessons.flatMap(l => [timeToMinutes(l.from), timeToMinutes(l.until)]);
  return {
    min: Math.min(...allTimes),
    max: Math.max(...allTimes)
  };
}

// Map a time (in minutes) to a Y position (px) in the grid
const GRID_HEIGHT = 800; // px, can adjust for your UI
function timeToY(time: number, min: number, max: number): number {
  return ((time - min) / (max - min)) * GRID_HEIGHT;
}

const timeBounds = $derived(getTimeBounds);

onMount(loadLessons);
</script>

<div class="flex flex-col h-full w-full text-slate-50">
  <div class="flex items-center justify-between px-4 py-2 bg-slate-800 shadow">
    <div class="flex gap-2 items-center">
      <button class="w-8 h-8 flex items-center justify-center rounded-full hover:bg-[var(--background)] transition-transform duration-300 hover:scale-110" onclick={prevWeek}>&#60;</button>
      <span class="text-lg font-bold">{weekRangeLabel()}</span>
      <button class="w-8 h-8 flex items-center justify-center rounded-full hover:bg-[var(--background)] transition-transform duration-300 hover:scale-110" onclick={nextWeek}>&#62;</button>
    </div>
  </div>

  {#key lessons}
  <div class="flex overflow-hidden flex-1 items-stretch">
    <div class="flex flex-col flex-1 w-full h-full justify-stretch">
      <!-- Header Row -->
      <div class="grid grid-cols-[60px_repeat(5,1fr)] w-full" style="border-bottom: 2px solid var(--surface-alt);">
        <div class="bg-slate-800 w-14"></div>
        {#each dayLabels as day, index}
          <div class="py-1 px-2 text-center font-bold bg-slate-800 border-l border-[var(--surface)] text-sm {new Date().getDay() === (index + 1) % 7 ? 'bg-blue-500 text-white' : ''}">{day.toUpperCase()}</div>
        {/each}
      </div>
      <!-- Time grid and lessons -->
      {#if lessons.length}
        <div class="relative flex-1 w-full h-full" style="height: {GRID_HEIGHT}px;">
          <!-- Time labels -->
          <div class="absolute top-0 left-0 z-10 w-14 h-full pointer-events-none">
            {#each getUniqueTimes() as time}
              <div class="absolute left-0 w-full border-t border-slate-800" style="top: {timeToY(timeToMinutes(time), timeBounds().min, timeBounds().max)}px;">
                <span class="text-xs text-slate-400">{time}</span>
              </div>
            {/each}
          </div>
          <!-- Day columns -->
          <div class="grid absolute top-0 right-0 left-14 grid-cols-5 h-full">
            {#each Array(5) as _, dayIdx}
              <div class="relative h-full border-l border-slate-800">
                {#each getLessonsFor(dayIdx) as lesson}
                  <div
                    class="absolute left-1 right-1 rounded-lg shadow-sm border-l-4 bg-slate-800 p-2 flex flex-col justify-center"
                    style="
                      top: {timeToY(timeToMinutes(lesson.from), timeBounds().min, timeBounds().max)}px;
                      height: {timeToY(timeToMinutes(lesson.until), timeBounds().min, timeBounds().max) - timeToY(timeToMinutes(lesson.from), timeBounds().min, timeBounds().max)}px;
                      border-color: {lesson.colour};
                    "
                  >
                    <span class="text-sm font-bold truncate" style="color: var(--text);">{lesson.description}</span>
                    <span class="text-xs truncate" style="color: var(--text-muted);">{lesson.staff}</span>
                    <span class="text-xs truncate" style="color: var(--text-muted);">{lesson.room}</span>
                    {#if lesson.attendanceTitle && lesson.attendanceTitle.trim()}
                      <span class="text-xs italic truncate" style="color: var(--text-muted);">{lesson.attendanceTitle}</span>
                    {/if}
                    <span class="mt-1 font-mono text-xs" style="color: var(--text);">{lesson.from} – {lesson.until}</span>
                  </div>
                {/each}
              </div>
            {/each}
          </div>
        </div>
      {/if}
      {#if loadingLessons}
        <div class="flex absolute inset-0 justify-center items-center"><span class="text-lg text-slate-400">Loading…</span></div>
      {/if}
    </div>
  </div>
  {/key}
</div> 