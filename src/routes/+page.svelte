<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { seqtaFetch } from "../utils/netUtil";
  import { cache } from "../utils/cache";
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
  } from "svelte-hero-icons";
  import Modal from "$lib/components/Modal.svelte";

  const studentId = 69; //! literally changes nothing but was used in the original seqta code.

  let currentSelectedDate: Date = new Date();

  let lessons = $state<any[]>([]);
  let lessonColours = $state<any[]>([]);
  let upcomingAssessments = $state<any[]>([]);
  let activeSubjects = $state<any[]>([]);
  let notices = $state<any[]>([]);

  let subjectFilters = $state<Record<string, boolean>>({});

  let loadingLessons = $state<boolean>(true);
  let loadingAssessments = $state<boolean>(true);
  let loadingNotices = $state<boolean>(true);

  const filteredAssessments = $derived(
    upcomingAssessments.filter((a: any) => subjectFilters[a.code])
  );

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
    { name: "Outlook", icon: "📅", url: "https://outlook.office.com" },
    { name: "Office365", icon: "🏢", url: "https://office365.com" },
    { name: "Google", icon: "🌐", url: "https://google.com" },
  ]);

  let weatherEnabled = $state(false);
  let weatherLocation = $state("");
  let weatherData: any = $state(null);
  let loadingWeather = $state(false);
  let weatherError = $state("");

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
    const m = (date.getMonth() + 1).toString().padStart(2, "0");
    const d = date.getDate().toString().padStart(2, "0");
    return `${y}-${m}-${d}`;
  }

  async function loadLessonColours() {
    if (lessonColours.length) return lessonColours;
    const res = await seqtaFetch("/seqta/student/load/prefs?", {
      method: "POST",
      headers: { "Content-Type": "application/json; charset=utf-8" },
      body: { request: "userPrefs", asArray: true, user: studentId },
    });
    lessonColours = JSON.parse(res).payload;
    return lessonColours;
  }

  async function loadLessons() {
    loadingLessons = true;
    const dateStr = formatDate(currentSelectedDate);

    const res = await seqtaFetch("/seqta/student/load/timetable?", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: { from: dateStr, until: dateStr, student: studentId },
    });

    const colours = await loadLessonColours();

    lessons = JSON.parse(res)
      .payload.items.sort((a: any, b: any) => a.from.localeCompare(b.from))
      .map((lesson: any) => {
        const colourPrefName = `timetable.subject.colour.${lesson.code}`;
        const subjectColour = colours.find(
          (c: any) => c.name === colourPrefName
        );

        lesson.colour = subjectColour
          ? `${subjectColour.value}`
          : `transparent`;

        lesson.from = lesson.from.substring(0, 5);
        lesson.until = lesson.until.substring(0, 5);

        lesson.attendanceTitle = lesson.attendance
          ? lesson.attendance.label
          : " ";
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
      const [sh, sm] = l.from.split(":").map(Number);
      const [eh, em] = l.until.split(":").map(Number);

      const start = new Date(currentSelectedDate);
      start.setHours(sh, sm, 0, 0);
      const end = new Date(currentSelectedDate);
      end.setHours(eh, em, 0, 0);

      l.active =
        now >= start &&
        now <= end &&
        now.toDateString() === currentSelectedDate.toDateString();
      return l;
    });
  }

  async function loadAssessments() {
    loadingAssessments = true;

    try {
      // Check cache first
      const cachedData = cache.get<{
        assessments: any[];
        subjects: any[];
        filters: Record<string, boolean>;
      }>('upcoming_assessments_data');
      
      if (cachedData) {
        upcomingAssessments = cachedData.assessments;
        activeSubjects = cachedData.subjects;
        subjectFilters = cachedData.filters;
        loadingAssessments = false;
        return;
      }

      const [assessmentsRes, classesRes] = await Promise.all([
        seqtaFetch("/seqta/student/assessment/list/upcoming?", {
          method: "POST",
          headers: { "Content-Type": "application/json; charset=utf-8" },
          body: { student: studentId },
        }),
        seqtaFetch("/seqta/student/load/subjects?", {
          method: "POST",
          headers: { "Content-Type": "application/json; charset=utf-8" },
          body: {},
        }),
      ]);

      const colours = await loadLessonColours();

      const classesResJson = JSON.parse(classesRes);
      const activeClass = classesResJson.payload.find((c: any) => c.active);
      activeSubjects = activeClass ? activeClass.subjects : [];

      activeSubjects.forEach((s: any) => {
        if (!(s.code in subjectFilters)) subjectFilters[s.code] = true;
      });

      const activeCodes = activeSubjects.map((s: any) => s.code);

      upcomingAssessments = JSON.parse(assessmentsRes)
        .payload.filter((a: any) => activeCodes.includes(a.code))
        .filter((a: any) => new Date(a.due) >= new Date())
        .map((a: any) => {
          const prefName = `timetable.subject.colour.${a.code}`;
          const c = colours.find((p: any) => p.name === prefName);
          a.colour = c ? c.value : "#8e8e8e";
          return a;
        })
        .sort((a: any, b: any) => (a.due < b.due ? -1 : 1));

      // Cache all the data for 1 hour
      cache.set('upcoming_assessments_data', {
        assessments: upcomingAssessments,
        subjects: activeSubjects,
        filters: subjectFilters
      }, 60);
    } catch (e) {
      console.error('Error loading assessments:', e);
    } finally {
      loadingAssessments = false;
    }
  }

  async function loadNotices(dateStr: string) {
    loadingNotices = true;

    const prefsRes = await seqtaFetch("/seqta/student/load/prefs?", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: { asArray: true, request: "userPrefs" },
    });

    const prefsResJson = JSON.parse(prefsRes);
    const filters = prefsResJson.payload.find(
      (p: any) => p.name === "notices.filters"
    );
    const labelArray = filters ? filters.value.split(" ") : [];

    const res = await seqtaFetch("/seqta/student/load/notices?", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: { date: dateStr },
    });

    notices = res.payload?.filter((n: any) =>
      labelArray.includes(JSON.stringify(n.label))
    );
    loadingNotices = false;
  }

  function prevDay() {
    currentSelectedDate = new Date(currentSelectedDate.valueOf() - 86_400_000);
    loadLessons();
  }

  function nextDay() {
    currentSelectedDate = new Date(currentSelectedDate.valueOf() + 86_400_000);
    loadLessons();
  }

  function buildAssessmentURL(
    programmeID: number,
    metaID: number,
    itemID?: number
  ) {
    const base = `../#?page=/assessments/${programmeID}:${metaID}`;
    return itemID ? `${base}&item=${itemID}` : base;
  }

  function getStatusBadge(status: string, due: string) {
    const dueDate = new Date(due);
    const now = new Date();

    if (status === "MARKS_RELEASED") {
      return { text: "Marked", color: "bg-green-500" };
    } else if (dueDate < now) {
      return { text: "Overdue", color: "bg-red-500" };
    } else if (dueDate.getTime() - now.getTime() < 7 * 24 * 60 * 60 * 1000) {
      return { text: "Due Soon", color: "bg-yellow-500" };
    } else {
      return { text: "Upcoming", color: "bg-blue-500" };
    }
  }

  async function fetchHomepageLabels() {
    const response = await seqtaFetch("/seqta/student/load/notices?", {
      method: "POST",
      body: { mode: "labels" },
    });
    const data = typeof response === "string" ? JSON.parse(response) : response;
    homepageLabels = Array.isArray(data?.payload) ? data.payload : [];
  }

  async function fetchHomepageNotices() {
    loadingHomepageNotices = true;
    const response = await seqtaFetch("/seqta/student/load/notices?", {
      method: "POST",
      body: { date: formatDate(new Date()) },
    });
    const data = typeof response === "string" ? JSON.parse(response) : response;
    homepageNotices = Array.isArray(data?.payload)
      ? data.payload.slice(0, 50)
      : [];
    loadingHomepageNotices = false;
  }

  function getHomepageLabelColor(labelId: number): string {
    return homepageLabels.find((l) => l.id === labelId)?.colour || "#910048";
  }
  function getHomepageLabelTitle(labelId: number): string {
    return homepageLabels.find((l) => l.id === labelId)?.title || "";
  }

  async function loadHomepageShortcuts() {
    try {
      const settings = await invoke<{ shortcuts: Shortcut[] }>("get_settings");
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
      }>("get_settings");
      weatherEnabled = settings.weather_enabled ?? false;
      weatherLocation = settings.weather_location ?? "";
    } catch (e) {
      weatherEnabled = false;
      weatherLocation = "";
    }
  }

  async function fetchWeather() {
    if (!weatherEnabled || !weatherLocation) {
      weatherData = null;
      return;
    }
    loadingWeather = true;
    weatherError = "";
    try {
      const geoRes = await fetch(
        `https://geocoding-api.open-meteo.com/v1/search?name=${encodeURIComponent(weatherLocation)}&count=1&language=en&format=json`
      );
      const geoJson = await geoRes.json();
      if (!geoJson.results || !geoJson.results.length)
        throw new Error("Location not found");
      const { latitude, longitude, name, country } = geoJson.results[0];

      const weatherRes = await fetch(
        `https://api.open-meteo.com/v1/forecast?latitude=${latitude}&longitude=${longitude}&current_weather=true&timezone=auto`
      );
      const weatherJson = await weatherRes.json();
      weatherData = {
        ...weatherJson.current_weather,
        location: name,
        country,
      };
    } catch (e) {
      weatherError = "Failed to load weather.";
      weatherData = null;
    } finally {
      loadingWeather = false;
    }
  }

  async function loadPortal() {
    try {
      const response = await seqtaFetch('/seqta/student/load/portals?', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json; charset=utf-8' },
        body: { splash: true }
      });

      const data = JSON.parse(response);
      if (data.status === '200' && data.payload?.url) {
        portalUrl = data.payload.url;
      } else {
        portalError = 'Failed to load portal URL';
      }
    } catch (e) {
      portalError = 'Error loading portal';
    } finally {
      loadingPortal = false;
    }
  }

  async function fetchHomeworkData() {
    try {
      loadingHomework = true;
      homeworkError = null;
      const response = await seqtaFetch('/seqta/student/dashlet/summary/homework', {
        method: 'POST',
        body: {},
        params: {"majhvjju": ""}
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
    newSubtasks = newSubtasks.filter(st => st.id !== id);
  }

  function addTodo() {
    if (newTodoText.trim()) {
      todos = [...todos, {
        id: Date.now(),
        text: newTodoText.trim(),
        completed: false,
        dueDate: newTodoDueDate || undefined,
        subtasks: newSubtasks.map(st => ({ ...st, completed: false })),
        priority: newTodoPriority,
        tags: newTodoTags.split(',').map(t => t.trim()).filter(Boolean),
        recurring: newTodoRecurring
      }];
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
    todos = todos.map(todo => 
      todo.id === id ? { ...todo, completed: !todo.completed } : todo
    );
    saveTodos();
  }

  function toggleSubtask(todoId: number, subtaskId: number) {
    todos = todos.map(todo =>
      todo.id === todoId
        ? {
            ...todo,
            subtasks: todo.subtasks?.map(st =>
              st.id === subtaskId ? { ...st, completed: !st.completed } : st
            )
          }
        : todo
    );
    saveTodos();
  }

  function deleteTodo(id: number) {
    todos = todos.filter(todo => todo.id !== id);
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
      loadLessons(),
      loadAssessments(),
      loadNotices(formatDate(new Date())),
      fetchHomepageLabels(),
      fetchHomepageNotices(),
      loadHomepageShortcuts(),
      loadWeatherSettings(),
      loadPortal(),
      fetchHomeworkData(),
      loadTodos()
    ]);
    if (weatherEnabled && weatherLocation) fetchWeather();
  });

  onDestroy(() => {
    if (lessonInterval) clearInterval(lessonInterval);
    if (timerInterval) clearInterval(timerInterval);
  });

  function lessonsSubtitle() {
    const today = new Date();
    const diff = ~~(
      (today.getTime() - currentSelectedDate.getTime()) /
      86_400_000
    );
    if (diff === 0) return "Today's Lessons";
    if (diff === -1) return "Tomorrow's Lessons";
    if (diff === 1) return "Yesterday's Lessons";
    return currentSelectedDate.toLocaleDateString("en-AU", {
      weekday: "short",
      year: "numeric",
      month: "numeric",
      day: "numeric",
    });
  }

  $effect(() => {
    if (weatherEnabled && weatherLocation) fetchWeather();
  });
</script>

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

<div
  class="p-8 mx-auto min-h-screen"
>
  <div class="space-y-8">
    <!-- Shortcuts Section -->
    <div class="grid grid-cols-2 gap-3 sm:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 sm:gap-4">
      {#each homepageShortcuts as shortcut}
        <a
          href={shortcut.url}
          target="_blank"
          rel="noopener noreferrer"
          class="flex relative flex-col justify-center items-center p-6 rounded-2xl border border-gray-200 shadow-sm transition-all duration-200 cursor-pointer group bg-white/80 dark:bg-slate-900/60 dark:border-slate-800 hover:accent-bg hover:shadow-lg hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 accent-ring"
          tabindex="0"
          aria-label={shortcut.name}
        >
          <div class="flex relative justify-center items-center mb-4 w-16 h-16 text-3xl text-white rounded-2xl shadow-lg transition-all duration-200 accent-bg group-hover:scale-110 group-active:scale-95">
            {shortcut.icon}
          </div>
          <span class="relative mt-1 text-base font-semibold text-center text-gray-900 transition-colors dark:text-white">{shortcut.name}</span>
        </a>
      {/each}
    </div>

    <!-- Today's Lessons Section -->
    <div
      class="overflow-hidden rounded-2xl border shadow-xl backdrop-blur-sm bg-white/80 dark:bg-slate-800/30 border-gray-300/50 dark:border-slate-700/50"
    >
      <div
        class="flex flex-col gap-4 justify-between items-start px-3 py-3 bg-gradient-to-r border-b sm:flex-row sm:items-center sm:px-4 border-gray-300/50 dark:border-slate-700/50 from-gray-100/70 dark:from-slate-800/70 to-gray-100/30 dark:to-slate-800/30"
      >
        <span class="text-xl font-semibold text-gray-900 dark:text-white">{lessonsSubtitle()}</span>
        <div class="flex gap-3">
          <button
            onclick={prevDay}
            class="flex justify-center items-center w-9 h-9 text-gray-600 rounded-full border transition-all duration-300 hover:accent-bg-hover dark:text-slate-400 hover:text-white border-gray-300/50 dark:border-slate-700/50 hover:accent-border hover:accent-shadow"
          >
            <Icon src={ChevronLeft} class="w-5 h-5" />
          </button>
          <button
            onclick={nextDay}
            class="flex justify-center items-center w-9 h-9 text-gray-600 rounded-full border transition-all duration-300 hover:accent-bg-hover dark:text-slate-400 hover:text-white border-gray-300/50 dark:border-slate-700/50 hover:accent-border hover:accent-shadow"
          >
            <Icon src={ChevronRight} class="w-5 h-5" />
          </button>
        </div>
      </div>

      {#if loadingLessons}
        <div class="flex flex-col justify-center items-center py-16">
          <div
            class="w-16 h-16 rounded-full border-4 animate-spin border-indigo-500/30 border-t-indigo-500"
          ></div>
          <p class="mt-4 text-gray-600 dark:text-slate-400">Loading your schedule...</p>
        </div>
      {:else if lessons.length === 0}
        <div class="flex flex-col gap-6 justify-center items-center px-4 py-16 w-full sm:flex-row sm:gap-12 sm:px-14 sm:py-20">
          <div class="flex justify-center items-center w-20 h-20 text-white rounded-full shadow-lg sm:w-28 sm:h-28 accent-bg">
            <span class="text-4xl sm:text-6xl">📚</span>
          </div>
          <div class="flex flex-col items-center">
            <p class="mb-2 text-2xl font-bold text-center text-gray-800 dark:text-white">No lessons today!</p>
            <p class="text-lg text-center text-gray-600 dark:text-slate-300">Enjoy your free time or check your other tasks.</p>
          </div>
        </div>
      {:else}
        <div class="flex overflow-x-scroll">
          {#each lessons as lesson, i}
            <div
              class="flex relative flex-col w-full max-w-xs border-t-4 group"
              style="border-color: {lesson.colour}; box-shadow: inset 0px 10px 10px -10px {lesson.colour};"
            >
              <div
                class="flex relative flex-col flex-1 gap-2 p-3 backdrop-blur-sm sm:p-4"
              >
                <div class="flex justify-between items-center">
                                  <span class="text-base font-bold text-gray-900 truncate sm:text-lg dark:text-white"
                  >{lesson.description}</span
                >
                  {#if lesson.active}
                    <span
                      class="px-2.5 py-1 ml-2 text-xs font-medium text-white bg-gradient-to-r from-green-500 to-emerald-600 rounded-full shadow-sm animate-gradient"
                      >Now</span
                    >
                  {/if}
                </div>
                <div class="flex items-center mt-1 text-sm text-gray-700 sm:text-base dark:text-slate-300">
                  <Icon
                    src={AcademicCap}
                    class="mr-1.5 w-4 h-4 text-gray-600 dark:text-slate-400"
                  />
                  <span class="truncate">{lesson.staff}</span>
                </div>
                <div class="flex items-center text-sm text-gray-700 sm:text-base dark:text-slate-300">
                  <Icon
                    src={BuildingOffice}
                    class="mr-1.5 w-4 h-4 text-gray-600 dark:text-slate-400"
                  />
                  <span class="truncate">{lesson.room}</span>
                </div>
                <div
                  class="inline-flex items-center px-3 py-1.5 mt-3 mb-auto font-mono text-sm rounded-lg bg-gray-200/50 dark:bg-slate-800/50 w-fit"
                >
                  <Icon src={Clock} class="mr-1.5 w-4 h-4 text-indigo-400" />
                  {lesson.from} – {lesson.until}
                </div>
                {#if lesson.attendanceTitle && lesson.attendanceTitle.trim()}
                  <div class="text-xs text-gray-600 dark:text-slate-400">
                    {lesson.attendanceTitle}
                  </div>
                {/if}

                {#if lesson.programmeID !== 0}
                  <div class="flex gap-3">
                    <button
                      class="flex justify-center items-center w-9 h-9 text-gray-700 rounded-lg border transition-all duration-300 bg-gray-200/70 dark:bg-slate-800/70 hover:accent-bg-hover dark:text-slate-300 hover:text-white border-gray-300/50 dark:border-slate-700/50 hover:accent-border"
                      aria-label="View Assessment"
                      onclick={() =>
                        (location.href = `/assessments?code=${lesson.code}&date=${lesson.date}`)}
                    >
                      <Icon src={DocumentText} class="w-5 h-5" />
                    </button>
                    <button
                      class="flex justify-center items-center w-9 h-9 text-gray-700 rounded-lg border transition-all duration-300 bg-gray-200/70 dark:bg-slate-800/70 hover:accent-bg-hover dark:text-slate-300 hover:text-white border-gray-300/50 dark:border-slate-700/50 hover:accent-border"
                      aria-label="View Course"
                      onclick={() =>
                        (location.href = `/courses?code=${lesson.code}&date=${lesson.date}`)}
                    >
                      <Icon src={BookOpen} class="w-5 h-5" />
                    </button>
                  </div>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Notices Widget -->
    <div
      class="overflow-hidden relative rounded-2xl border shadow-xl backdrop-blur-sm bg-white/80 dark:bg-slate-800/30 border-gray-300/50 dark:border-slate-700/50"
    >
        <div
          class="flex justify-between items-center px-4 py-3 bg-gradient-to-br border-b from-gray-100/70 dark:from-slate-800/70 to-gray-100/30 dark:to-slate-800/30 border-gray-300/50 dark:border-slate-700/50"
        >
          <h3 class="text-xl font-semibold text-gray-900 dark:text-white">Notices</h3>
        <a
          href="/notices"
          class="px-3 py-1.5 text-sm rounded-lg transition-all duration-300 text-nowrap accent-text hover:accent-bg-hover hover:text-white"
        >
          View all
          <Icon src={ArrowTopRightOnSquare} class="inline ml-1 w-4 h-4" />
        </a>
      </div>
      <div
        class="overflow-y-auto px-6 py-4 max-h-80 scrollbar-thin scrollbar-thumb-indigo-500/30 scrollbar-track-slate-800/10"
      >
        {#if loadingHomepageNotices}
          <div class="py-10 text-center">
            <div
              class="mx-auto w-12 h-12 rounded-full border-4 animate-spin border-indigo-500/30 border-t-indigo-500"
            ></div>
            <p class="mt-4 text-gray-600 dark:text-slate-400">Loading notices...</p>
          </div>
        {:else if homepageNotices.length === 0}
          <div class="py-10 text-center text-gray-600 dark:text-slate-400">
            No notices available.
          </div>
        {:else}
          {#each homepageNotices as notice}
            <div
              class="p-4 mb-4 rounded-xl border transition-all duration-300 last:mb-0 bg-gray-100/60 dark:bg-slate-800/60 hover:bg-gray-200/80 dark:hover:bg-slate-800/80 border-gray-300/50 dark:border-slate-700/50 hover:border-gray-400/50 dark:hover:border-slate-600/50"
            >
              <div class="flex gap-2 items-center mb-2">
                <span
                  class="px-2.5 py-1 text-xs font-medium rounded-full animate-gradient"
                  style="background: linear-gradient(135deg, {getHomepageLabelColor(
                    notice.label
                  )}, {getHomepageLabelColor(notice.label)}dd); color: white;"
                >
                  {getHomepageLabelTitle(notice.label)}
                </span>
                <span class="text-xs text-gray-600 dark:text-slate-400">{notice.staff}</span>
              </div>
                              <div class="mb-2 text-base font-bold text-gray-900 dark:text-white">
                  {notice.title}
                </div>
                <div class="text-sm text-gray-700 dark:text-slate-300 line-clamp-2">
                  {@html notice.contents}
                </div>
            </div>
          {/each}
        {/if}
      </div>
    </div>

    <!-- Upcoming Assessments -->
    <div
      class="overflow-hidden relative rounded-2xl border shadow-xl backdrop-blur-sm bg-white/80 dark:bg-slate-800/30 border-gray-300/50 dark:border-slate-700/50"
    >
              <div
          class="flex justify-between items-center px-4 py-3 bg-gradient-to-br border-b from-gray-100/70 dark:from-slate-800/70 to-gray-100/30 dark:to-slate-800/30 border-gray-300/50 dark:border-slate-700/50"
        >
          <span class="pr-4 text-xl font-semibold text-gray-900 dark:text-white text-nowrap"
            >Upcoming Assessments</span
          >
        <div class="flex overflow-x-scroll gap-2" id="upcoming-filters">
          {#each activeSubjects as subj}
                          <label
                class="flex items-center px-2.5 py-1.5 text-xs rounded-full border transition-all duration-300 cursor-pointer sm:px-3 sm:text-sm bg-gray-200/70 dark:bg-slate-800/70 border-gray-300/50 dark:border-slate-700/50 hover:border-indigo-500/50"
              >
                              <input
                  type="checkbox"
                  bind:checked={subjectFilters[subj.code]}
                  class="mr-2 w-3.5 h-3.5 text-indigo-500 rounded border-gray-300 sm:w-4 sm:h-4 form-checkbox dark:border-slate-700 focus:ring-indigo-500 focus:ring-offset-gray-100 dark:focus:ring-offset-slate-900"
                />
              <span style="color: {subj.colour}">{subj.code}</span>
            </label>
          {/each}
        </div>
      </div>

      {#if loadingAssessments}
        <div class="flex flex-col justify-center items-center py-12 sm:py-16">
          <div
            class="w-12 h-12 rounded-full border-4 animate-spin sm:w-16 sm:h-16 border-indigo-500/30 border-t-indigo-500"
          ></div>
                      <p class="mt-4 text-sm text-gray-600 sm:text-base dark:text-slate-400">Loading assessments...</p>
        </div>
      {:else if filteredAssessments.length === 0}
        <div class="flex flex-col justify-center items-center py-12 sm:py-16">
          <div
            class="w-16 h-16 sm:w-20 sm:h-20 flex items-center justify-center rounded-full bg-gradient-to-br from-indigo-500 to-purple-600 text-2xl sm:text-3xl shadow-[0_0_20px_rgba(99,102,241,0.3)] animate-gradient"
          >
            🎉
          </div>
          <p class="mt-4 text-lg text-gray-700 sm:text-xl dark:text-slate-300">Nothing coming up!</p>
        </div>
      {:else}
        <div class="grid grid-cols-1 gap-4 p-4 sm:grid-cols-2 lg:grid-cols-3 sm:p-6">
          {#each filteredAssessments as a}
            <div
              class="flex flex-col gap-4 p-4 sm:p-5 rounded-xl transition-all duration-300 hover:scale-[1.02] hover:shadow-[0_0_15px_rgba(99,102,241,0.2)] relative group"
            >
              <div
                class="absolute inset-0 bg-gradient-to-br rounded-xl opacity-30 animate-gradient"
                style="background: linear-gradient(135deg, {a.colour}20, {a.colour}05);"
              ></div>
              <div
                class="absolute inset-0 rounded-xl border"
                style="border: 1px solid {a.colour}30;"
              ></div>

              <div class="flex relative z-10 gap-4 items-center">
                <div
                  class="flex justify-center items-center w-12 h-12 bg-gradient-to-br rounded-xl shadow-lg sm:h-14 sm:w-14 animate-gradient"
                  style="background: linear-gradient(135deg, {a.colour}, {a.colour}dd);"
                >
                  <Icon src={DocumentText} class="w-6 h-6 text-white" />
                </div>

                <div class="flex-1 min-w-0">
                  <div class="flex flex-wrap gap-2 items-center">
                    <div class="text-sm font-bold dark:text-white sm:text-base">
                      {new Date(a.due).toLocaleDateString("en-AU", {
                        weekday: "short",
                        month: "short",
                        day: "numeric",
                      })}
                    </div>
                    <span
                      class="px-2 py-0.5 rounded-full text-xs font-medium text-white shadow-sm {getStatusBadge(
                        a.status,
                        a.due
                      ).color}"
                    >
                      {getStatusBadge(a.status, a.due).text}
                    </span>
                  </div>
                  <div class="mt-1">
                    <span
                      class="block text-xs font-semibold text-gray-600 uppercase dark:text-slate-400"
                      >{a.subject}</span
                    >
                    <span class="block text-sm font-semibold text-gray-900 truncate dark:text-white sm:text-base"
                      >{a.title}</span
                    >
                  </div>
                </div>
              </div>

              {#if a.description}
                <div class="relative z-10 text-sm text-gray-700 dark:text-slate-300 line-clamp-2">
                  {a.description}
                </div>
              {/if}
            </div>
          {/each}
        </div>
      {/if}
    </div>

    <!-- Welcome Portal Window -->
    <div
      class="overflow-hidden relative rounded-2xl border shadow-xl backdrop-blur-sm bg-white/80 dark:bg-slate-800/30 border-gray-300/50 dark:border-slate-700/50"
    >
      <div
        class="flex justify-between items-center px-4 py-3 bg-gradient-to-br border-b from-gray-100/70 dark:from-slate-800/70 to-gray-100/30 dark:to-slate-800/30 border-gray-300/50 dark:border-slate-700/50"
      >
        <h3 class="text-xl font-semibold text-gray-900 dark:text-white">Welcome Portal</h3>
        <button
          onclick={() => showPortalModal = true}
          class="px-3 py-1.5 text-sm rounded-lg transition-all duration-300 text-nowrap accent-text hover:accent-bg-hover hover:text-white"
        >
          Open Full Screen
          <Icon src={ArrowTopRightOnSquare} class="inline ml-1 w-4 h-4" />
        </button>
      </div>

      <div class="h-[400px]">
        {#if loadingPortal}
          <div class="flex flex-col justify-center items-center h-full">
            <div class="w-16 h-16 rounded-full border-4 animate-spin border-indigo-500/30 border-t-indigo-500"></div>
            <p class="mt-4 text-slate-400">Loading welcome portal...</p>
          </div>
        {:else if portalError}
          <div class="flex flex-col justify-center items-center h-full">
            <div class="w-20 h-20 flex items-center justify-center rounded-full bg-gradient-to-br from-red-500 to-red-600 text-3xl shadow-[0_0_20px_rgba(239,68,68,0.3)] animate-gradient">
              ⚠️
            </div>
            <p class="mt-4 text-xl text-slate-300">{portalError}</p>
          </div>
        {:else if portalUrl}
          <iframe
            src={portalUrl}
            class="w-full h-full border-0"
            title="Welcome Portal"
          ></iframe>
        {/if}
      </div>
    </div>

    <!-- Dashboard Grid -->
    <div class="grid grid-cols-1 gap-8 lg:grid-cols-2">
      <!-- Left Column: Homework and Todo List -->
      <div class="space-y-8">
        <!-- Homework Section -->
        <div class="overflow-hidden relative rounded-2xl border shadow-xl backdrop-blur-sm bg-white/80 dark:bg-slate-800/30 border-gray-300/50 dark:border-slate-700/50">
          <div class="flex justify-between items-center px-4 py-3 bg-gradient-to-br border-b from-gray-100/70 dark:from-slate-800/70 to-gray-100/30 dark:to-slate-800/30 border-gray-300/50 dark:border-slate-700/50">
            <h3 class="text-xl font-semibold text-gray-900 dark:text-white">Homework</h3>
          </div>
          <div class="p-6">
            {#if loadingHomework}
              <div class="flex justify-center items-center py-12">
                <div class="w-16 h-16 rounded-full border-4 animate-spin border-indigo-500/30 border-t-indigo-500"></div>
                <p class="mt-4 text-gray-600 dark:text-slate-400">Loading homework data...</p>
              </div>
            {:else if homeworkError}
              <div class="px-6 py-4 mb-4 text-red-700 bg-red-100 rounded-lg border border-red-200">
                <p>Error: {homeworkError}</p>
              </div>
            {:else if homeworkData}
              <div class="flex flex-col gap-6">
                {#each homeworkData.payload as homework}
                  <div class="rounded-xl border-l-8 shadow-lg backdrop-blur-sm bg-gray-100/80 dark:bg-slate-800/50" style="border-color: var(--accent);">
                    <div class="px-6 pt-5 pb-3">
                      <h3 class="mb-2 text-lg font-bold text-gray-900 dark:text-white">{homework.title}</h3>
                      <div class="flex flex-col gap-3">
                        {#each homework.items as item}
                          <div class="flex gap-2 items-start px-4 py-3 rounded-lg border border-gray-300 backdrop-blur-sm bg-gray-200/80 dark:bg-slate-700/50 dark:border-slate-600">
                            <span class="mt-1 text-xl accent-text">•</span>
                            <span class="text-gray-800 dark:text-slate-50">{item}</span>
                          </div>
                        {/each}
                      </div>
                    </div>
                  </div>
                {/each}
              </div>
            {:else}
              <p class="text-center text-gray-600 dark:text-slate-400">No homework data available</p>
            {/if}
          </div>
        </div>

        <!-- Todo List Section -->
        <div class="overflow-hidden relative rounded-2xl border shadow-xl backdrop-blur-sm bg-white/80 dark:bg-slate-800/30 border-gray-300/50 dark:border-slate-700/50">
          <div class="flex justify-between items-center px-4 py-3 bg-gradient-to-br border-b from-gray-100/70 dark:from-slate-800/70 to-gray-100/30 dark:to-slate-800/30 border-gray-300/50 dark:border-slate-700/50">
            <h3 class="text-xl font-semibold text-gray-900 dark:text-white">Todo List</h3>
          </div>
          <div class="p-6">
            <form onsubmit={(e) => { e.preventDefault(); addTodo(); }} class="mb-6">
              <div class="flex flex-col gap-6 p-4 rounded-lg border border-gray-300 bg-gray-100/60 dark:bg-slate-800/40 dark:border-slate-700">
                <!-- Main Task -->
                <div class="flex flex-col gap-4 items-stretch sm:flex-row">
                  <input
                    type="text"
                    bind:value={newTodoText}
                    placeholder="Add a new task..."
                    class="flex-1 px-4 py-2 text-gray-900 bg-white rounded-lg border border-gray-300 shadow-sm dark:bg-slate-900/60 dark:text-white dark:border-slate-700 focus:outline-none focus:ring-2 focus:ring-accent"
                  />
                  <input
                    type="date"
                    bind:value={newTodoDueDate}
                    class="px-4 py-2 text-gray-900 bg-white rounded-lg border border-gray-300 shadow-sm dark:bg-slate-900/60 dark:text-white dark:border-slate-700 focus:outline-none focus:ring-2 focus:ring-accent"
                  />
                </div>
                <!-- Details -->
                <div class="flex flex-col gap-4 items-stretch sm:flex-row">
                  <div class="flex relative flex-1 items-center">
                    <span class="absolute left-3 top-1/2 -translate-y-1/2">
                      <svg width="16" height="16" fill="none" viewBox="0 0 24 24" class="{newTodoPriority === 'high' ? 'text-red-500' : newTodoPriority === 'medium' ? 'text-yellow-400' : 'text-green-500'}"><circle cx="12" cy="12" r="10" fill="currentColor"/></svg>
                    </span>
                    <select bind:value={newTodoPriority} class="py-2 pr-4 pl-8 w-full text-gray-900 bg-white rounded-lg border border-gray-300 dark:bg-slate-900/60 dark:text-white dark:border-slate-700">
                      <option value="low">Low Priority</option>
                      <option value="medium">Medium Priority</option>
                      <option value="high">High Priority</option>
                    </select>
                  </div>
                  <div class="flex relative flex-1 items-center">
                    <span class="absolute left-3 top-1/2 -translate-y-1/2">
                      <svg width="16" height="16" fill="none" viewBox="0 0 24 24" class="text-blue-400"><rect x="4" y="11" width="16" height="2" rx="1" fill="currentColor"/><rect x="11" y="4" width="2" height="16" rx="1" fill="currentColor"/></svg>
                    </span>
                    <input
                      type="text"
                      bind:value={newTodoTags}
                      placeholder="Tags (comma separated, e.g. school,math)"
                      class="py-2 pr-4 pl-8 w-full text-gray-900 bg-white rounded-lg border border-gray-300 dark:bg-slate-900/60 dark:text-white dark:border-slate-700"
                    />
                  </div>
                  <div class="flex relative flex-1 items-center">
                    <span class="absolute left-3 top-1/2 -translate-y-1/2">
                      <svg width="16" height="16" fill="none" viewBox="0 0 24 24" class="text-purple-400"><circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2"/><path d="M12 6v6l4 2" stroke="currentColor" stroke-width="2" stroke-linecap="round"/></svg>
                    </span>
                    <select bind:value={newTodoRecurring} class="py-2 pr-4 pl-8 w-full text-gray-900 bg-white rounded-lg border border-gray-300 dark:bg-slate-900/60 dark:text-white dark:border-slate-700">
                      <option value="none">No Repeat</option>
                      <option value="daily">Daily</option>
                      <option value="weekly">Weekly</option>
                      <option value="monthly">Monthly</option>
                    </select>
                  </div>
                </div>
                <!-- Subtasks -->
                <div class="p-3 rounded-lg bg-gray-200/60 dark:bg-slate-900/40">
                  <div class="flex gap-2 items-center mb-2">
                    <svg width="18" height="18" fill="none" viewBox="0 0 24 24" class="text-gray-600 dark:text-slate-400"><rect x="4" y="11" width="16" height="2" rx="1" fill="currentColor"/></svg>
                    <input
                      type="text"
                      bind:value={newSubtaskText}
                      placeholder="Add subtask (e.g. Read chapter 1)"
                      class="flex-1 px-4 py-2 text-gray-900 bg-white rounded-lg border border-gray-300 dark:bg-slate-900/60 dark:text-white dark:border-slate-700"
                    />
                    <button type="button" onclick={addSubtask} class="px-4 py-2 text-white rounded-lg shadow transition-colors bg-accent hover:bg-accent/90">Add Subtask</button>
                  </div>
                  <div class="flex flex-wrap gap-2">
                    {#each newSubtasks as st (st.id)}
                      <span class="flex gap-2 items-center px-3 py-1 text-gray-800 bg-gray-300 rounded-lg shadow-sm dark:bg-slate-700 dark:text-white">
                        {st.text}
                        <button type="button" onclick={() => removeSubtask(st.id)} class="ml-2 text-red-400 hover:text-red-600">×</button>
                      </span>
                    {/each}
                  </div>
                </div>
                <div class="flex justify-end">
                  <button
                    type="submit"
                    class="px-8 py-2 font-semibold text-white rounded-lg shadow-lg transition-colors bg-accent hover:bg-accent/90"
                  >
                    Add Task
                  </button>
                </div>
              </div>
            </form>

            <div class="space-y-4">
              {#each todos as todo (todo.id)}
                <div class="flex flex-col gap-2 p-4 rounded-xl border border-gray-200 backdrop-blur-sm transition-shadow bg-white/80 dark:bg-slate-800/60 dark:border-slate-700 hover:shadow-lg group">
                  <div class="flex gap-3 items-center">
                    <input
                      type="checkbox"
                      checked={todo.completed}
                      onchange={() => toggleTodo(todo.id)}
                      class="w-5 h-5 rounded border-gray-300 dark:border-slate-600 text-accent focus:ring-accent accent-bg"
                    />
                    <div class="flex-1">
                      <div class="flex gap-2 items-center">
                        <p class="text-gray-900 dark:text-white {todo.completed ? 'line-through text-gray-500 dark:text-slate-400' : ''} font-semibold">
                          {todo.text}
                        </p>
                        {#if todo.priority}
                          <span class="inline-block px-2 py-0.5 rounded-full text-xs font-semibold {todo.priority === 'high' ? 'bg-red-600' : todo.priority === 'medium' ? 'bg-yellow-500' : 'bg-green-600'} text-white">
                            {todo.priority.charAt(0).toUpperCase() + todo.priority.slice(1)}
                          </span>
                        {/if}
                        {#if todo.tags && todo.tags.length}
                          <span class="flex gap-1 ml-2">
                            {#each todo.tags as tag}
                              <span class="px-2 py-0.5 text-xs text-blue-100 rounded-full bg-blue-700/80">#{tag}</span>
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
                        <p class="mt-1 text-sm text-gray-600 dark:text-slate-400">
                          Due: {new Date(todo.dueDate).toLocaleDateString()}
                        </p>
                      {/if}
                    </div>
                    <button
                      onclick={() => deleteTodo(todo.id)}
                      class="p-2 text-gray-600 transition-colors dark:text-slate-400 hover:text-red-500 hover:scale-125 focus:outline-none"
                      title="Delete task"
                    >
                      <svg width="20" height="20" fill="none" viewBox="0 0 24 24"><path d="M6 6l12 12M6 18L18 6" stroke="currentColor" stroke-width="2" stroke-linecap="round"/></svg>
                    </button>
                  </div>
                  {#if todo.subtasks && todo.subtasks.length}
                    <div class="flex flex-col gap-1 p-2 mt-2 ml-8 rounded-lg bg-gray-200/60 dark:bg-slate-900/40">
                      {#each todo.subtasks as st (st.id)}
                        <div class="flex gap-2 items-center">
                          <input
                            type="checkbox"
                            checked={st.completed}
                            onchange={() => toggleSubtask(todo.id, st.id)}
                            class="w-4 h-4 rounded border-gray-300 dark:border-slate-600 text-accent accent-bg"
                          />
                          <span class="text-sm text-gray-900 dark:text-white {st.completed ? 'line-through text-gray-500 dark:text-slate-400' : ''}">{st.text}</span>
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
      <div class="overflow-hidden relative rounded-2xl border shadow-xl backdrop-blur-sm bg-white/80 dark:bg-slate-800/30 border-gray-300/50 dark:border-slate-700/50">
        <div class="flex justify-between items-center px-4 py-3 bg-gradient-to-br border-b from-gray-100/70 dark:from-slate-800/70 to-gray-100/30 dark:to-slate-800/30 border-gray-300/50 dark:border-slate-700/50">
          <h3 class="text-xl font-semibold text-gray-900 dark:text-white">Focus Timer</h3>
        </div>
        <div class="p-6">
          <div class="flex flex-col items-center">
            <div class="mb-8 font-mono text-7xl font-bold text-gray-900 dark:text-white">
              {String(timerMinutes).padStart(2, '0')}:{String(timerSeconds).padStart(2, '0')}
            </div>
            <div class="flex gap-4 justify-center mt-6">
              <button 
                onclick={startTimer}
                class="px-6 py-3 font-semibold text-white rounded-lg transition-all duration-200 hover:scale-105 hover:shadow-lg accent-bg"
              >
                Start Timer
              </button>
              <button
                onclick={stopTimer}
                disabled={!isTimerRunning}
                class="px-8 py-3 text-white bg-red-600 rounded-lg transition-colors hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                Stop
              </button>
              <button
                onclick={resetTimer}
                class="px-8 py-3 text-gray-900 bg-gray-300 rounded-lg transition-colors dark:bg-slate-700 dark:text-white hover:bg-gray-400 dark:hover:bg-slate-600">
              >
                Reset
              </button>
            </div>
            <div class="grid grid-cols-3 gap-4 mt-8">
              <button
                onclick={() => { timerMinutes = 25; timerSeconds = 0; }}
                class="px-6 py-3 text-gray-900 rounded-lg backdrop-blur-sm transition-colors bg-gray-200/80 dark:bg-slate-800/50 dark:text-white hover:bg-gray-300 dark:hover:bg-slate-700"
              >
                25m
              </button>
              <button
                onclick={() => { timerMinutes = 45; timerSeconds = 0; }}
                class="px-6 py-3 text-gray-900 rounded-lg backdrop-blur-sm transition-colors bg-gray-200/80 dark:bg-slate-800/50 dark:text-white hover:bg-gray-300 dark:hover:bg-slate-700"
              >
                45m
              </button>
              <button
                onclick={() => { timerMinutes = 60; timerSeconds = 0; }}
                class="px-6 py-3 text-gray-900 rounded-lg backdrop-blur-sm transition-colors bg-gray-200/80 dark:bg-slate-800/50 dark:text-white hover:bg-gray-300 dark:hover:bg-slate-700"
              >
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
                  class="px-4 py-3 w-28 text-gray-900 bg-white rounded-lg border border-gray-300 backdrop-blur-sm dark:bg-slate-800/50 dark:text-white dark:border-slate-700 focus:outline-none focus:ring-2 focus:ring-accent"
                />
                <span class="text-2xl text-gray-900 dark:text-white">:</span>
                <input
                  type="number"
                  bind:value={customSeconds}
                  placeholder="Seconds"
                  min="0"
                  max="59"
                  class="px-4 py-3 w-28 text-gray-900 bg-white rounded-lg border border-gray-300 backdrop-blur-sm dark:bg-slate-800/50 dark:text-white dark:border-slate-700 focus:outline-none focus:ring-2 focus:ring-accent"
                />
              </div>
              <button
                onclick={setCustomTime}
                class="px-8 py-3 text-gray-900 rounded-lg backdrop-blur-sm transition-colors bg-gray-200/80 dark:bg-slate-800/50 dark:text-white hover:bg-gray-300 dark:hover:bg-slate-700"
              >
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
  ariaLabel="Welcome Portal Modal"
>
  {#if portalUrl}
    <iframe
      src={portalUrl}
      class="w-full h-full rounded-2xl border-0"
      title="Welcome Portal"
    ></iframe>
  {/if}
</Modal>
