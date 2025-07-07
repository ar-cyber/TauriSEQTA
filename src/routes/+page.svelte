<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import Modal from '$lib/components/Modal.svelte';
  import TodaySchedule from '$lib/components/TodaySchedule.svelte';
  import NoticesPane from '$lib/components/NoticesPane.svelte';
  import UpcomingAssessments from '$lib/components/UpcomingAssessments.svelte';
  import WelcomePortal from '$lib/components/WelcomePortal.svelte';
  import TodoList from '$lib/components/TodoList.svelte';
  import FocusTimer from '$lib/components/FocusTimer.svelte';
  import Homework from '$lib/components/Homework.svelte';
  import ShortcutsWidget from '$lib/components/ShortcutsWidget.svelte';
  import { Icon } from 'svelte-hero-icons';
  import { ArrowsPointingOut, ArrowsPointingIn, XMark } from 'svelte-hero-icons';

  let currentSelectedDate: Date = new Date();
  let lessons = $state<any[]>([]);

  let lessonInterval: ReturnType<typeof setInterval> | null = null;

  interface Shortcut {
    name: string;
    icon: string;
    url: string;
  }

  interface WidgetLayout {
    id: string;
    x: number;
    y: number;
    width: number; // 1 = half width, 2 = full width
    height: number; // 1 = normal height, 2 = double height
    enabled: boolean;
  }

  interface Widget {
    id: string;
    component: any;
    title: string;
    icon: string;
    defaultWidth: number;
    defaultHeight: number;
  }

  let homepageShortcuts = $state<Shortcut[]>([
    { name: 'Outlook', icon: '📅', url: 'https://outlook.office.com' },
    { name: 'Office365', icon: '🏢', url: 'https://office365.com' },
    { name: 'Google', icon: '🌐', url: 'https://google.com' },
  ]);

  let showPortalModal = $state(false);
  let portalUrl = $state<string>('');
  let widgetLayouts = $state<WidgetLayout[]>([]);
  let isEditMode = $state(false);
  let draggedWidget = $state<string | null>(null);
  let dragOffset = $state({ x: 0, y: 0 });
  let showWidgetPicker = $state(false);

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

  // Widget definitions
  const widgets: Widget[] = [
    { id: 'shortcuts', component: ShortcutsWidget, title: 'Shortcuts', icon: '🔗', defaultWidth: 2, defaultHeight: 1 },
    { id: 'today_schedule', component: TodaySchedule, title: 'Today\'s Schedule', icon: '📅', defaultWidth: 2, defaultHeight: 1 },
    { id: 'notices', component: NoticesPane, title: 'Notices', icon: '📢', defaultWidth: 2, defaultHeight: 1 },
    { id: 'upcoming_assessments', component: UpcomingAssessments, title: 'Upcoming Assessments', icon: '📝', defaultWidth: 2, defaultHeight: 1 },
    { id: 'welcome_portal', component: WelcomePortal, title: 'Welcome Portal', icon: '🏠', defaultWidth: 2, defaultHeight: 1 },
    { id: 'homework', component: Homework, title: 'Homework', icon: '📚', defaultWidth: 1, defaultHeight: 1 },
    { id: 'todo_list', component: TodoList, title: 'Todo List', icon: '✅', defaultWidth: 1, defaultHeight: 1 },
    { id: 'focus_timer', component: FocusTimer, title: 'Focus Timer', icon: '⏱️', defaultWidth: 1, defaultHeight: 2 },
  ];

  async function loadHomepageShortcuts() {
    try {
      const settings = await invoke<{ shortcuts: Shortcut[] }>('get_settings');
      if (settings.shortcuts && settings.shortcuts.length > 0) {
        homepageShortcuts = settings.shortcuts;
      }
    } catch (e) {}
  }

  async function loadWidgetLayouts() {
    try {
      const settings = await invoke<{ widget_layout: WidgetLayout[] }>('get_settings');
      if (settings.widget_layout && settings.widget_layout.length > 0) {
        widgetLayouts = settings.widget_layout;
      } else {
        // Use default layout
        widgetLayouts = widgets.map((widget, index) => ({
          id: widget.id,
          x: index < 5 ? 0 : (index === 5 || index === 6 ? 0 : 1),
          y: index,
          width: widget.defaultWidth,
          height: widget.defaultHeight,
          enabled: true,
        }));
      }
    } catch (e) {
      console.error('Failed to load widget layouts:', e);
    }
  }

  async function saveWidgetLayouts() {
    try {
      const currentSettings = await invoke<any>('get_settings');
      const newSettings = {
        ...currentSettings,
        widget_layout: widgetLayouts,
      };
      await invoke('save_settings', { newSettings });
    } catch (e) {
      console.error('Failed to save widget layouts:', e);
    }
  }

  function toggleEditMode() {
    isEditMode = !isEditMode;
  }

  function handleDragStart(event: MouseEvent, widgetId: string) {
    if (!isEditMode) return;
    
    const target = event.currentTarget as HTMLElement;
    const rect = target.getBoundingClientRect();
    dragOffset.x = event.clientX - rect.left;
    dragOffset.y = event.clientY - rect.top;
    draggedWidget = widgetId;
    
    event.preventDefault();
  }

  function handleDragMove(event: MouseEvent) {
    if (!isEditMode || !draggedWidget) return;
    
    const container = document.querySelector('.widget-grid') as HTMLElement;
    if (!container) return;
    
    const containerRect = container.getBoundingClientRect();
    const x = Math.floor((event.clientX - containerRect.left - dragOffset.x) / 200); // 200px per grid cell
    const y = Math.floor((event.clientY - containerRect.top - dragOffset.y) / 200);
    
    const widget = widgetLayouts.find(w => w.id === draggedWidget);
    if (widget) {
      widget.x = Math.max(0, Math.min(1, x)); // Constrain to 0-1 for x
      widget.y = Math.max(0, y); // Allow unlimited y
    }
  }

  function handleDragEnd() {
    if (draggedWidget) {
      saveWidgetLayouts();
      draggedWidget = null;
    }
  }

  function toggleWidgetSize(widgetId: string) {
    const widget = widgetLayouts.find(w => w.id === widgetId);
    if (widget) {
      widget.width = widget.width === 1 ? 2 : 1;
      saveWidgetLayouts();
    }
  }

  function toggleWidgetEnabled(widgetId: string) {
    const widget = widgetLayouts.find(w => w.id === widgetId);
    if (widget) {
      widget.enabled = !widget.enabled;
      saveWidgetLayouts();
    }
  }

  function addWidget() {
    showWidgetPicker = true;
  }

  function selectWidget(widgetId: string) {
    const widget = getWidgetById(widgetId);
    if (widget) {
      // Find the next available position
      const maxY = Math.max(...widgetLayouts.map(w => w.y + w.height), 0);
      const newLayout: WidgetLayout = {
        id: widgetId,
        x: 0,
        y: maxY,
        width: widget.defaultWidth,
        height: widget.defaultHeight,
        enabled: true,
      };
      widgetLayouts = [...widgetLayouts, newLayout];
      saveWidgetLayouts();
    }
    showWidgetPicker = false;
  }

  function getAvailableWidgets() {
    const usedIds = new Set(widgetLayouts.filter(w => w.enabled).map(w => w.id));
    return widgets.filter(w => !usedIds.has(w.id));
  }

  function getWidgetById(id: string): Widget | undefined {
    return widgets.find(w => w.id === id);
  }

  function renderWidget(widgetId: string) {
    const widget = getWidgetById(widgetId);
    const layout = widgetLayouts.find(w => w.id === widgetId);
    
    if (!widget || !layout || !layout.enabled) return null;
    
    if (widgetId === 'shortcuts') {
      return {
        component: widget.component,
        props: { shortcuts: homepageShortcuts }
      };
    }
    
    return {
      component: widget.component,
      props: {}
    };
  }

  function closeModal() {
    showPortalModal = false;
  }

  onMount(() => {
    loadHomepageShortcuts();
    loadWidgetLayouts();
    
    // Add global mouse event listeners for dragging
    document.addEventListener('mousemove', handleDragMove);
    document.addEventListener('mouseup', handleDragEnd);
  });

  onDestroy(() => {
    if (lessonInterval) clearInterval(lessonInterval);
    document.removeEventListener('mousemove', handleDragMove);
    document.removeEventListener('mouseup', handleDragEnd);
  });

  onDestroy(() => {
    if (lessonInterval) clearInterval(lessonInterval);
  });
</script>

<div class="p-8 mx-auto min-h-screen">
  <!-- Edit Mode Toggle -->
  <div class="flex justify-between items-center mb-4">
    {#if isEditMode}
      <div class="flex gap-2">
        <button
          onclick={() => addWidget()}
          class="flex items-center gap-2 px-4 py-2 rounded-lg bg-accent-500 text-white hover:bg-accent-600 transition-all duration-200 transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 focus:ring-accent-500 focus:ring-offset-2"
        >
          + Add Widget
        </button>
      </div>
    {/if}
    <button
      onclick={toggleEditMode}
      class="flex items-center gap-2 px-4 py-2 rounded-lg bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 hover:bg-slate-200 dark:hover:bg-slate-700 transition-all duration-200 transform hover:scale-105 active:scale-95 focus:outline-none focus:ring-2 focus:ring-accent-500 focus:ring-offset-2"
    >
      {#if isEditMode}
        <Icon src={ArrowsPointingIn} class="w-4 h-4" />
        Exit Edit Mode
      {:else}
        <Icon src={ArrowsPointingOut} class="w-4 h-4" />
        Edit Layout
      {/if}
    </button>
  </div>

  <!-- Widget Grid -->
  <div class="widget-grid relative" style="display: grid; grid-template-columns: 1fr 1fr; gap: 2rem; min-height: 100vh;">
    {#each widgetLayouts.filter(w => w.enabled).sort((a, b) => a.y - b.y || a.x - b.x) as layout (layout.id)}
      {@const widget = getWidgetById(layout.id)}
      {@const renderedWidget = renderWidget(layout.id)}
      
      {#if renderedWidget}
        <div
          class="widget-container relative {isEditMode ? 'cursor-move' : ''} bg-white/80 dark:bg-slate-900/60 rounded-2xl border border-slate-200 dark:border-slate-800 shadow-sm p-4 {draggedWidget === layout.id ? 'opacity-75 shadow-lg' : ''}"
          style="grid-column: {layout.x + 1} / span {layout.width}; grid-row: {layout.y + 1} / span {layout.height};"
          onmousedown={(e) => handleDragStart(e, layout.id)}
        >
          <!-- Widget Header (Edit Mode) -->
          {#if isEditMode}
            <div class="absolute top-2 right-2 z-10 flex gap-1">
              <button
                onclick={() => toggleWidgetSize(layout.id)}
                class="p-1 rounded bg-slate-800/80 text-white hover:bg-slate-700/80 transition-colors"
                title={layout.width === 1 ? 'Expand' : 'Shrink'}
              >
                {layout.width === 1 ? '↔' : '↕'}
              </button>
              <button
                onclick={() => toggleWidgetEnabled(layout.id)}
                class="p-1 rounded bg-red-600/80 text-white hover:bg-red-500/80 transition-colors"
                title="Remove Widget"
              >
                <Icon src={XMark} class="w-3 h-3" />
              </button>
            </div>
          {/if}

          <!-- Widget Content -->
          <div class="h-full">
            {#if renderedWidget.component}
              <svelte:component this={renderedWidget.component} {...renderedWidget.props} />
            {/if}
          </div>
        </div>
      {/if}
    {/each}
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

<!-- Widget Picker Modal -->
<Modal
  bind:open={showWidgetPicker}
  onclose={() => showWidgetPicker = false}
  maxWidth="w-96"
  maxHeight="h-auto"
  customClasses="bg-white dark:bg-slate-900 rounded-2xl shadow-2xl"
  ariaLabel="Widget Picker Modal">
  <div class="p-6">
    <h2 class="text-xl font-semibold text-slate-900 dark:text-white mb-4">Add Widget</h2>
    <div class="grid grid-cols-1 gap-3">
      {#each getAvailableWidgets() as widget}
        <button
          onclick={() => selectWidget(widget.id)}
          class="flex items-center gap-3 p-3 rounded-lg border border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-800 hover:bg-slate-100 dark:hover:bg-slate-700 transition-all duration-200 transform hover:scale-[1.02] focus:outline-none focus:ring-2 focus:ring-accent-500 focus:ring-offset-2"
        >
          <span class="text-2xl">{widget.icon}</span>
          <div class="text-left">
            <div class="font-medium text-slate-900 dark:text-white">{widget.title}</div>
            <div class="text-sm text-slate-500 dark:text-slate-400">
              {widget.defaultWidth === 1 ? 'Half width' : 'Full width'} • 
              {widget.defaultHeight === 1 ? 'Normal height' : 'Double height'}
            </div>
          </div>
        </button>
      {/each}
    </div>
  </div>
</Modal>