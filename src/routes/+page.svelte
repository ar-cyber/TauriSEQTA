<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { seqtaFetch } from '../utils/netUtil';
  import { cache } from '../utils/cache';
  import {
    Icon,
    ChevronLeft,
    ChevronRight,
    AcademicCap,
    Clock,
    DocumentText,
    BookOpen,
    BuildingOffice,
    ArrowTopRightOnSquare,
    XMark,
  } from 'svelte-hero-icons';
  import Modal from '$lib/components/Modal.svelte';
  import TodaySchedule from '$lib/components/TodaySchedule.svelte';
  import NoticesPane from '$lib/components/NoticesPane.svelte';
  import UpcomingAssessments from '$lib/components/UpcomingAssessments.svelte';
  import WelcomePortal from '$lib/components/WelcomePortal.svelte';

  const studentId = 69; //! literally changes nothing but was used in the original seqta code.

  let currentSelectedDate: Date = new Date();

  let lessons = $state<any[]>([]);
  let lessonColours = $state<any[]>([]);
  let notices = $state<any[]>([]);

  let loadingLessons = $state<boolean>(true);
  let loadingNotices = $state<boolean>(true);

  let lessonInterval: ReturnType<typeof setInterval> | null = null;

  let homepageNotices = $state<any[]>([]);
  let homepageLabels = $state<any[]>([]);
  let loadingHomepageNotices = $state(true);

  interface Shortcut {
    name: string;
    icon: string;
    url: string;
  }

  let homepageShortcuts = $state<Shortcut[]>([
    { name: 'Outlook', icon: '📅', url: 'https://outlook.office.com' },
    { name: 'Office365', icon: '🏢', url: 'https://office365.com' },
    { name: 'Google', icon: '🌐', url: 'https://google.com' },
  ]);

  let weatherEnabled = $state(false);
  let weatherLocation = $state('');
  let weatherData: any = $state(null);
  let loadingWeather = $state(false);
  let weatherError = $state('');

  let selectedTab = $state<'list' | 'board'>('list');
  let showPortalModal = $state(false);

  let portalUrl = $state<string>('');
  let loadingPortal = $state<boolean>(true);
  let portalError = $state<string>('');

  // Dashboard state
  interface HomeworkItem {
    meta: number;
    id: number;
    title: string;
    items: string[];
  }

  interface HomeworkResponse {
    payload: HomeworkItem[];
    status: string;
  }

  interface TodoItem {
    id: number;
    text: string;
    completed: boolean;
    dueDate?: string;
    subtasks?: { id: number; text: string; completed: boolean }[];
    priority?: 'low' | 'medium' | 'high';
    tags?: string[];
    recurring?: 'none' | 'daily' | 'weekly' | 'monthly';
  }

  let homeworkData = $state<HomeworkResponse | null>(null);
  let homeworkError = $state<string | null>(null);
  let loadingHomework = $state(true);

  // Todo list state
  let todos = $state<TodoItem[]>([]);
  let newTodoText = $state('');
  let newTodoDueDate = $state('');
  let newTodoPriority = $state<'low' | 'medium' | 'high'>('medium');
  let newTodoTags = $state(''); // comma-separated
  let newTodoRecurring = $state<'none' | 'daily' | 'weekly' | 'monthly'>('none');
  let newSubtasks = $state<{ id: number; text: string }[]>([]);
  let newSubtaskText = $state('');

  // Timer state
  let timerMinutes = $state(25);
  let timerSeconds = $state(0);
  let isTimerRunning = $state(false);
  let timerInterval: ReturnType<typeof setInterval> | null = null;
  let customMinutes = $state('');
  let customSeconds = $state('');

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
      body: { request: 'userPrefs', asArray: true, user: studentId },
    });
    lessonColours = JSON.parse(res).payload;
    return lessonColours;
  }

  async function loadLessons() {
    loadingLessons = true;
    const dateStr = formatDate(currentSelectedDate);

    const res = await seqtaFetch('/seqta/student/load/timetable?', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: { from: dateStr, until: dateStr, student: studentId },
    });

    const colours = await loadLessonColours();

    lessons = JSON.parse(res)
      .payload.items.sort((a: any, b: any) => a.from.localeCompare(b.from))
      .map((lesson: any) => {
        const colourPrefName = `timetable.subject.colour.${lesson.code}`;
        const subjectColour = colours.find((c: any) => c.name === colourPrefName);

        lesson.colour = subjectColour ? `${subjectColour.value}` : `transparent`;

        lesson.from = lesson.from.substring(0, 5);
        lesson.until = lesson.until.substring(0, 5);

        lesson.attendanceTitle = lesson.attendance ? lesson.attendance.label : ' ';
        return lesson;
      });

    loadingLessons = false;

    if (lessonInterval) clearInterval(lessonInterval);
    checkCurrentLessons();
    lessonInterval = setInterval(checkCurrentLessons, 60_000);
  }

  function checkCurrentLessons() {
    const now = new Date();
    lessons = lessons.map((l: any) => {
      const [sh, sm] = l.from.split(':').map(Number);
      const [eh, em] = l.until.split(':').map(Number);

      const start = new Date(currentSelectedDate);
      start.setHours(sh, sm, 0, 0);
      const end = new Date(currentSelectedDate);
      end.setHours(eh, em, 0, 0);

      l.active =
        now >= start && now <= end && now.toDateString() === currentSelectedDate.toDateString();
      return l;
    });
  }

  async function fetchHomepageLabels() {
    const response = await seqtaFetch('/seqta/student/load/notices?', {
      method: 'POST',
      body: { mode: 'labels' },
    });
    const data = typeof response === 'string' ? JSON.parse(response) : response;
    homepageLabels = Array.isArray(data?.payload) ? data.payload : [];
  }

  async function fetchHomepageNotices() {
    loadingHomepageNotices = true;
    const response = await seqtaFetch('/seqta/student/load/notices?', {
      method: 'POST',
      body: { date: formatDate(new Date()) },
    });
    const data = typeof response === 'string' ? JSON.parse(response) : response;
    homepageNotices = Array.isArray(data?.payload) ? data.payload.slice(0, 50) : [];
    loadingHomepageNotices = false;
  }

  function getHomepageLabelColor(labelId: number): string {
    return homepageLabels.find((l) => l.id === labelId)?.colour || '#910048';
  }
  function getHomepageLabelTitle(labelId: number): string {
    return homepageLabels.find((l) => l.id === labelId)?.title || '';
  }

  async function loadHomepageShortcuts() {
    try {
      const settings = await invoke<{ shortcuts: Shortcut[] }>('get_settings');
      if (settings.shortcuts && settings.shortcuts.length > 0) {
        homepageShortcuts = settings.shortcuts;
      }
    } catch (e) {}
  }

  async function loadWeatherSettings() {
    try {
      const settings = await invoke<{
        weather_enabled: boolean;
        weather_location: string;
      }>('get_settings');
      weatherEnabled = settings.weather_enabled ?? false;
      weatherLocation = settings.weather_location ?? '';
    } catch (e) {
      weatherEnabled = false;
      weatherLocation = '';
    }
  }

  async function fetchWeather() {
    if (!weatherEnabled || !weatherLocation) {
      weatherData = null;
      return;
    }
    loadingWeather = true;
    weatherError = '';
    try {
      const geoRes = await fetch(
        `https://geocoding-api.open-meteo.com/v1/search?name=${encodeURIComponent(weatherLocation)}&count=1&language=en&format=json`,
      );
      const geoJson = await geoRes.json();
      if (!geoJson.results || !geoJson.results.length) throw new Error('Location not found');
      const { latitude, longitude, name, country } = geoJson.results[0];

      const weatherRes = await fetch(
        `https://api.open-meteo.com/v1/forecast?latitude=${latitude}&longitude=${longitude}&current_weather=true&timezone=auto`,
      );
      const weatherJson = await weatherRes.json();
      weatherData = {
        ...weatherJson.current_weather,
        location: name,
        country,
      };
    } catch (e) {
      weatherError = 'Failed to load weather.';
      weatherData = null;
    } finally {
      loadingWeather = false;
    }
  }

  async function fetchHomeworkData() {
    try {
      loadingHomework = true;
      homeworkError = null;
      const response = await seqtaFetch('/seqta/student/dashlet/summary/homework', {
        method: 'POST',
        body: {},
        params: { majhvjju: '' },
      });
      homeworkData = JSON.parse(response);
    } catch (e: any) {
      console.error('Error details:', e);
      homeworkError = e.toString();
    } finally {
      loadingHomework = false;
    }
  }

  function addSubtask() {
    if (newSubtaskText.trim()) {
      newSubtasks = [...newSubtasks, { id: Date.now(), text: newSubtaskText.trim() }];
      newSubtaskText = '';
    }
  }

  function removeSubtask(id: number) {
    newSubtasks = newSubtasks.filter((st) => st.id !== id);
  }

  function addTodo() {
    if (newTodoText.trim()) {
      todos = [
        ...todos,
        {
          id: Date.now(),
          text: newTodoText.trim(),
          completed: false,
          dueDate: newTodoDueDate || undefined,
          subtasks: newSubtasks.map((st) => ({ ...st, completed: false })),
          priority: newTodoPriority,
          tags: newTodoTags
            .split(',')
            .map((t) => t.trim())
            .filter(Boolean),
          recurring: newTodoRecurring,
        },
      ];
      newTodoText = '';
      newTodoDueDate = '';
      newTodoPriority = 'medium';
      newTodoTags = '';
      newTodoRecurring = 'none';
      newSubtasks = [];
      saveTodos();
    }
  }

  function toggleTodo(id: number) {
    todos = todos.map((todo) => (todo.id === id ? { ...todo, completed: !todo.completed } : todo));
    saveTodos();
  }

  function toggleSubtask(todoId: number, subtaskId: number) {
    todos = todos.map((todo) =>
      todo.id === todoId
        ? {
            ...todo,
            subtasks: todo.subtasks?.map((st) =>
              st.id === subtaskId ? { ...st, completed: !st.completed } : st,
            ),
          }
        : todo,
    );
    saveTodos();
  }

  function deleteTodo(id: number) {
    todos = todos.filter((todo) => todo.id !== id);
    saveTodos();
  }

  function saveTodos() {
    localStorage.setItem('todos', JSON.stringify(todos));
  }

  function loadTodos() {
    const savedTodos = localStorage.getItem('todos');
    if (savedTodos) {
      todos = JSON.parse(savedTodos);
    }
  }

  function startTimer() {
    if (!isTimerRunning) {
      isTimerRunning = true;
      timerInterval = setInterval(() => {
        if (timerSeconds > 0) {
          timerSeconds--;
        } else if (timerMinutes > 0) {
          timerMinutes--;
          timerSeconds = 59;
        } else {
          stopTimer();
          // Play notification sound
          new Audio('/timer.mp3').play().catch(() => {});
        }
      }, 1000);
    }
  }

  function stopTimer() {
    if (timerInterval) {
      clearInterval(timerInterval);
      timerInterval = null;
    }
    isTimerRunning = false;
  }

  function resetTimer() {
    stopTimer();
    timerMinutes = 25;
    timerSeconds = 0;
  }

  function setCustomTime() {
    const minutes = parseInt(customMinutes) || 0;
    const seconds = parseInt(customSeconds) || 0;
    if (minutes >= 0 && seconds >= 0 && seconds < 60) {
      timerMinutes = minutes;
      timerSeconds = seconds;
      customMinutes = '';
      customSeconds = '';
    }
  }

  function closeModal() {
    showPortalModal = false;
  }

  onMount(async () => {
    await Promise.all([
      loadHomepageShortcuts(),
      loadWeatherSettings(),
      fetchHomeworkData(),
      loadTodos(),
    ]);
    if (weatherEnabled && weatherLocation) fetchWeather();
  });

  onDestroy(() => {
    if (lessonInterval) clearInterval(lessonInterval);
    if (timerInterval) clearInterval(timerInterval);
  });

  function lessonsSubtitle() {
    const today = new Date();
    const diff = ~~((today.getTime() - currentSelectedDate.getTime()) / 86_400_000);
    if (diff === 0) return "Today's Lessons";
    if (diff === -1) return "Tomorrow's Lessons";
    if (diff === 1) return "Yesterday's Lessons";
    return currentSelectedDate.toLocaleDateString('en-AU', {
      weekday: 'short',
      year: 'numeric',
      month: 'numeric',
      day: 'numeric',
    });
  }

  $effect(() => {
    if (weatherEnabled && weatherLocation) fetchWeather();
  });
</script>

<div class="p-8 mx-auto min-h-screen">
  <div class="space-y-8">
    <!-- Shortcuts Section -->
    <div class="grid grid-cols-2 gap-3 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 sm:gap-4">
      {#each homepageShortcuts as shortcut}
        <a
          href={shortcut.url}
          target="_blank"
          rel="noopener noreferrer"
          class="flex relative flex-col justify-center items-center p-6 rounded-2xl border shadow-sm transition-all duration-200 cursor-pointer border-slate-200 group bg-white/80 dark:bg-slate-900/60 dark:border-slate-800 hover:accent-bg hover:shadow-lg hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 accent-ring"
          tabindex="0"
          aria-label={shortcut.name}>
          <div
            class="flex relative justify-center items-center mb-4 w-16 h-16 text-3xl text-white rounded-2xl shadow-lg transition-all duration-200 accent-bg group-hover:scale-110 group-active:scale-95">
            {shortcut.icon}
          </div>
          <span
            class="relative mt-1 text-base font-semibold text-center transition-colors text-slate-900 dark:text-white"
            >{shortcut.name}</span>
        </a>
      {/each}
    </div>

    <!-- Today's Schedule Section -->
    <TodaySchedule />

    <!-- Notices Section -->
    <NoticesPane />

    <!-- Upcoming Assessments -->
    <UpcomingAssessments />

    <!-- Welcome Portal Window -->
    <WelcomePortal />

    <!-- Dashboard Grid -->
    <div class="grid grid-cols-1 gap-8 lg:grid-cols-2">
      <!-- Left Column: Homework and Todo List -->
      <div class="space-y-8">
        <!-- Homework Section -->
        <div
          class="overflow-hidden relative rounded-2xl border shadow-xl backdrop-blur-sm bg-white/80 dark:bg-slate-800/30 border-slate-300/50 dark:border-slate-700/50">
          <div
            class="flex justify-between items-center px-4 py-3 bg-gradient-to-br border-b from-slate-100/70 dark:from-slate-800/70 to-slate-100/30 dark:to-slate-800/30 border-slate-300/50 dark:border-slate-700/50">
            <h3 class="text-xl font-semibold text-slate-900 dark:text-white">Homework</h3>
          </div>
          <div class="p-6">
            {#if loadingHomework}
              <div class="flex justify-center items-center py-12">
                <div
                  class="w-16 h-16 rounded-full border-4 animate-spin border-indigo-500/30 border-t-indigo-500">
                </div>
                <p class="mt-4 text-slate-600 dark:text-slate-400">Loading homework data...</p>
              </div>
            {:else if homeworkError}
              <div class="px-6 py-4 mb-4 text-red-700 bg-red-100 rounded-lg border border-red-200">
                <p>Error: {homeworkError}</p>
              </div>
            {:else if homeworkData}
              <div class="flex flex-col gap-6">
                {#each homeworkData.payload as homework}
                  <div
                    class="rounded-xl border-l-8 shadow-lg backdrop-blur-sm bg-slate-100/80 dark:bg-slate-800/50"
                    style="border-color: var(--accent);">
                    <div class="px-6 pt-5 pb-3">
                      <h3 class="mb-2 text-lg font-bold text-slate-900 dark:text-white">
                        {homework.title}
                      </h3>
                      <div class="flex flex-col gap-3">
                        {#each homework.items as item}
                          <div
                            class="flex gap-2 items-start px-4 py-3 rounded-lg border backdrop-blur-sm border-slate-300 bg-slate-200/80 dark:bg-slate-700/50 dark:border-slate-600">
                            <span class="mt-1 text-xl accent-text">•</span>
                            <span class="text-slate-800 dark:text-slate-50">{item}</span>
                          </div>
                        {/each}
                      </div>
                    </div>
                  </div>
                {/each}
              </div>
            {:else}
              <p class="text-center text-slate-600 dark:text-slate-400">
                No homework data available
              </p>
            {/if}
          </div>
        </div>

        <!-- Todo List Section -->
        <div
          class="overflow-hidden relative rounded-2xl border shadow-xl backdrop-blur-sm bg-white/80 dark:bg-slate-800/30 border-slate-300/50 dark:border-slate-700/50">
          <div
            class="flex justify-between items-center px-4 py-3 bg-gradient-to-br border-b from-slate-100/70 dark:from-slate-800/70 to-slate-100/30 dark:to-slate-800/30 border-slate-300/50 dark:border-slate-700/50">
            <h3 class="text-xl font-semibold text-slate-900 dark:text-white">Todo List</h3>
          </div>
          <div class="p-6">
            <form
              onsubmit={(e) => {
                e.preventDefault();
                addTodo();
              }}
              class="mb-6">
              <div
                class="flex flex-col gap-6 p-4 rounded-lg border border-slate-300 bg-slate-100/60 dark:bg-slate-800/40 dark:border-slate-700">
                <!-- Main Task -->
                <div class="flex flex-col gap-4 items-stretch sm:flex-row">
                  <input
                    type="text"
                    bind:value={newTodoText}
                    placeholder="Add a new task..."
                    class="flex-1 px-4 py-2 bg-white rounded-lg border shadow-sm text-slate-900 border-slate-300 dark:bg-slate-900/60 dark:text-white dark:border-slate-700 focus:outline-none focus:ring-2 focus:ring-accent" />
                  <input
                    type="date"
                    bind:value={newTodoDueDate}
                    class="px-4 py-2 bg-white rounded-lg border shadow-sm text-slate-900 border-slate-300 dark:bg-slate-900/60 dark:text-white dark:border-slate-700 focus:outline-none focus:ring-2 focus:ring-accent" />
                </div>
                <!-- Details -->
                <div class="flex flex-col gap-4 items-stretch sm:flex-row">
                  <div class="flex relative flex-1 items-center">
                    <span class="absolute left-3 top-1/2 -translate-y-1/2">
                      <svg
                        width="16"
                        height="16"
                        fill="none"
                        viewBox="0 0 24 24"
                        class={newTodoPriority === 'high'
                          ? 'text-red-500'
                          : newTodoPriority === 'medium'
                            ? 'text-yellow-400'
                            : 'text-green-500'}
                        ><circle cx="12" cy="12" r="10" fill="currentColor" /></svg>
                    </span>
                    <select
                      bind:value={newTodoPriority}
                      class="py-2 pr-4 pl-8 w-full bg-white rounded-lg border text-slate-900 border-slate-300 dark:bg-slate-900/60 dark:text-white dark:border-slate-700">
                      <option value="low">Low Priority</option>
                      <option value="medium">Medium Priority</option>
                      <option value="high">High Priority</option>
                    </select>
                  </div>
                  <div class="flex relative flex-1 items-center">
                    <span class="absolute left-3 top-1/2 -translate-y-1/2">
                      <svg
                        width="16"
                        height="16"
                        fill="none"
                        viewBox="0 0 24 24"
                        class="text-blue-400"
                        ><rect x="4" y="11" width="16" height="2" rx="1" fill="currentColor" /><rect
                          x="11"
                          y="4"
                          width="2"
                          height="16"
                          rx="1"
                          fill="currentColor" /></svg>
                    </span>
                    <input
                      type="text"
                      bind:value={newTodoTags}
                      placeholder="Tags (comma separated, e.g. school,math)"
                      class="py-2 pr-4 pl-8 w-full bg-white rounded-lg border text-slate-900 border-slate-300 dark:bg-slate-900/60 dark:text-white dark:border-slate-700" />
                  </div>
                  <div class="flex relative flex-1 items-center">
                    <span class="absolute left-3 top-1/2 -translate-y-1/2">
                      <svg
                        width="16"
                        height="16"
                        fill="none"
                        viewBox="0 0 24 24"
                        class="text-purple-400"
                        ><circle
                          cx="12"
                          cy="12"
                          r="10"
                          stroke="currentColor"
                          stroke-width="2" /><path
                          d="M12 6v6l4 2"
                          stroke="currentColor"
                          stroke-width="2"
                          stroke-linecap="round" /></svg>
                    </span>
                    <select
                      bind:value={newTodoRecurring}
                      class="py-2 pr-4 pl-8 w-full bg-white rounded-lg border text-slate-900 border-slate-300 dark:bg-slate-900/60 dark:text-white dark:border-slate-700">
                      <option value="none">No Repeat</option>
                      <option value="daily">Daily</option>
                      <option value="weekly">Weekly</option>
                      <option value="monthly">Monthly</option>
                    </select>
                  </div>
                </div>
                <!-- Subtasks -->
                <div class="p-3 rounded-lg bg-slate-200/60 dark:bg-slate-900/40">
                  <div class="flex gap-2 items-center mb-2">
                    <svg
                      width="18"
                      height="18"
                      fill="none"
                      viewBox="0 0 24 24"
                      class="text-slate-600 dark:text-slate-400"
                      ><rect x="4" y="11" width="16" height="2" rx="1" fill="currentColor" /></svg>
                    <input
                      type="text"
                      bind:value={newSubtaskText}
                      placeholder="Add subtask (e.g. Read chapter 1)"
                      class="flex-1 px-4 py-2 bg-white rounded-lg border text-slate-900 border-slate-300 dark:bg-slate-900/60 dark:text-white dark:border-slate-700" />
                    <button
                      type="button"
                      onclick={addSubtask}
                      class="px-4 py-2 text-white rounded-lg shadow transition-colors bg-accent hover:bg-accent/90"
                      >Add Subtask</button>
                  </div>
                  <div class="flex flex-wrap gap-2">
                    {#each newSubtasks as st (st.id)}
                      <span
                        class="flex gap-2 items-center px-3 py-1 rounded-lg shadow-sm text-slate-800 bg-slate-300 dark:bg-slate-700 dark:text-white">
                        {st.text}
                        <button
                          type="button"
                          onclick={() => removeSubtask(st.id)}
                          class="ml-2 text-red-400 hover:text-red-600">×</button>
                      </span>
                    {/each}
                  </div>
                </div>
                <div class="flex justify-end">
                  <button
                    type="submit"
                    class="px-8 py-2 font-semibold text-white rounded-lg shadow-lg transition-colors bg-accent hover:bg-accent/90">
                    Add Task
                  </button>
                </div>
              </div>
            </form>

            <div class="space-y-4">
              {#each todos as todo (todo.id)}
                <div
                  class="flex flex-col gap-2 p-4 rounded-xl border backdrop-blur-sm transition-shadow border-slate-200 bg-white/80 dark:bg-slate-800/60 dark:border-slate-700 hover:shadow-lg group">
                  <div class="flex gap-3 items-center">
                    <input
                      type="checkbox"
                      checked={todo.completed}
                      onchange={() => toggleTodo(todo.id)}
                      class="w-5 h-5 rounded border-slate-300 dark:border-slate-600 text-accent focus:ring-accent accent-bg" />
                    <div class="flex-1">
                      <div class="flex gap-2 items-center">
                        <p
                          class="text-slate-900 dark:text-white {todo.completed
                            ? 'line-through text-slate-500 dark:text-slate-400'
                            : ''} font-semibold">
                          {todo.text}
                        </p>
                        {#if todo.priority}
                          <span
                            class="inline-block px-2 py-0.5 rounded-full text-xs font-semibold {todo.priority ===
                            'high'
                              ? 'bg-red-600'
                              : todo.priority === 'medium'
                                ? 'bg-yellow-500'
                                : 'bg-green-600'} text-white">
                            {todo.priority.charAt(0).toUpperCase() + todo.priority.slice(1)}
                          </span>
                        {/if}
                        {#if todo.tags && todo.tags.length}
                          <span class="flex gap-1 ml-2">
                            {#each todo.tags as tag}
                              <span
                                class="px-2 py-0.5 text-xs text-blue-100 rounded-full bg-blue-700/80"
                                >#{tag}</span>
                            {/each}
                          </span>
                        {/if}
                        {#if todo.recurring && todo.recurring !== 'none'}
                          <span class="inline-block ml-2 text-xs text-purple-300">
                            {todo.recurring.charAt(0).toUpperCase() + todo.recurring.slice(1)}
                          </span>
                        {/if}
                      </div>
                      {#if todo.dueDate}
                        <p class="mt-1 text-sm text-slate-600 dark:text-slate-400">
                          Due: {new Date(todo.dueDate).toLocaleDateString()}
                        </p>
                      {/if}
                    </div>
                    <button
                      onclick={() => deleteTodo(todo.id)}
                      class="p-2 transition-colors text-slate-600 dark:text-slate-400 hover:text-red-500 hover:scale-125 focus:outline-none"
                      title="Delete task">
                      <svg width="20" height="20" fill="none" viewBox="0 0 24 24"
                        ><path
                          d="M6 6l12 12M6 18L18 6"
                          stroke="currentColor"
                          stroke-width="2"
                          stroke-linecap="round" /></svg>
                    </button>
                  </div>
                  {#if todo.subtasks && todo.subtasks.length}
                    <div
                      class="flex flex-col gap-1 p-2 mt-2 ml-8 rounded-lg bg-slate-200/60 dark:bg-slate-900/40">
                      {#each todo.subtasks as st (st.id)}
                        <div class="flex gap-2 items-center">
                          <input
                            type="checkbox"
                            checked={st.completed}
                            onchange={() => toggleSubtask(todo.id, st.id)}
                            class="w-4 h-4 rounded border-slate-300 dark:border-slate-600 text-accent accent-bg" />
                          <span
                            class="text-sm text-slate-900 dark:text-white {st.completed
                              ? 'line-through text-slate-500 dark:text-slate-400'
                              : ''}">{st.text}</span>
                        </div>
                      {/each}
                    </div>
                  {/if}
                </div>
              {/each}
            </div>
          </div>
        </div>
      </div>

      <!-- Right Column: Timer -->
      <div
        class="overflow-hidden relative rounded-2xl border shadow-xl backdrop-blur-sm bg-white/80 dark:bg-slate-800/30 border-slate-300/50 dark:border-slate-700/50">
        <div
          class="flex justify-between items-center px-4 py-3 bg-gradient-to-br border-b from-slate-100/70 dark:from-slate-800/70 to-slate-100/30 dark:to-slate-800/30 border-slate-300/50 dark:border-slate-700/50">
          <h3 class="text-xl font-semibold text-slate-900 dark:text-white">Focus Timer</h3>
        </div>
        <div class="p-6">
          <div class="flex flex-col items-center">
            <div class="mb-8 font-mono text-7xl font-bold text-slate-900 dark:text-white">
              {String(timerMinutes).padStart(2, '0')}:{String(timerSeconds).padStart(2, '0')}
            </div>
            <div class="flex gap-4 justify-center mt-6">
              <button
                onclick={startTimer}
                class="px-6 py-3 font-semibold text-white rounded-lg transition-all duration-200 hover:scale-105 hover:shadow-lg accent-bg">
                Start Timer
              </button>
              <button
                onclick={stopTimer}
                disabled={!isTimerRunning}
                class="px-8 py-3 text-white bg-red-600 rounded-lg transition-colors hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed">
                Stop
              </button>
              <button
                onclick={resetTimer}
                class="px-8 py-3 rounded-lg transition-colors text-slate-900 bg-slate-300 dark:bg-slate-700 dark:text-white hover:bg-slate-400 dark:hover:bg-slate-600">
                > Reset
              </button>
            </div>
            <div class="grid grid-cols-3 gap-4 mt-8">
              <button
                onclick={() => {
                  timerMinutes = 25;
                  timerSeconds = 0;
                }}
                class="px-6 py-3 rounded-lg backdrop-blur-sm transition-colors text-slate-900 bg-slate-200/80 dark:bg-slate-800/50 dark:text-white hover:bg-slate-300 dark:hover:bg-slate-700">
                25m
              </button>
              <button
                onclick={() => {
                  timerMinutes = 45;
                  timerSeconds = 0;
                }}
                class="px-6 py-3 rounded-lg backdrop-blur-sm transition-colors text-slate-900 bg-slate-200/80 dark:bg-slate-800/50 dark:text-white hover:bg-slate-300 dark:hover:bg-slate-700">
                45m
              </button>
              <button
                onclick={() => {
                  timerMinutes = 60;
                  timerSeconds = 0;
                }}
                class="px-6 py-3 rounded-lg backdrop-blur-sm transition-colors text-slate-900 bg-slate-200/80 dark:bg-slate-800/50 dark:text-white hover:bg-slate-300 dark:hover:bg-slate-700">
                60m
              </button>
            </div>
            <div class="flex flex-col gap-4 items-center mt-8">
              <div class="flex gap-4 items-center">
                <input
                  type="number"
                  bind:value={customMinutes}
                  placeholder="Minutes"
                  min="0"
                  class="px-4 py-3 w-28 bg-white rounded-lg border backdrop-blur-sm text-slate-900 border-slate-300 dark:bg-slate-800/50 dark:text-white dark:border-slate-700 focus:outline-none focus:ring-2 focus:ring-accent" />
                <span class="text-2xl text-slate-900 dark:text-white">:</span>
                <input
                  type="number"
                  bind:value={customSeconds}
                  placeholder="Seconds"
                  min="0"
                  max="59"
                  class="px-4 py-3 w-28 bg-white rounded-lg border backdrop-blur-sm text-slate-900 border-slate-300 dark:bg-slate-800/50 dark:text-white dark:border-slate-700 focus:outline-none focus:ring-2 focus:ring-accent" />
              </div>
              <button
                onclick={setCustomTime}
                class="px-8 py-3 rounded-lg backdrop-blur-sm transition-colors text-slate-900 bg-slate-200/80 dark:bg-slate-800/50 dark:text-white hover:bg-slate-300 dark:hover:bg-slate-700">
                Set Custom Time
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

<Modal
  bind:open={showPortalModal}
  onclose={closeModal}
  maxWidth="w-[80%]"
  maxHeight="h-[80%]"
  customClasses="bg-white dark:bg-slate-900 rounded-2xl shadow-2xl"
  ariaLabel="Welcome Portal Modal">
  {#if portalUrl}
    <iframe src={portalUrl} class="w-full h-full rounded-2xl border-0" title="Welcome Portal"
    ></iframe>
  {/if}
</Modal>

<style>
  @keyframes gradient-shift {
    0% {
      background-position: 0% 50%;
    }
    50% {
      background-position: 100% 50%;
    }
    100% {
      background-position: 0% 50%;
    }
  }

  @keyframes fade-in {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes fade-out {
    from {
      opacity: 1;
    }
    to {
      opacity: 0;
    }
  }

  .animate-gradient {
    background-size: 200% 200%;
    animation: gradient-shift 8s ease infinite;
  }

  .animate-fade-in {
    animation: fade-in 0.2s ease-out;
  }

  .animate-fade-out {
    animation: fade-out 0.2s ease-out;
  }
</style>
