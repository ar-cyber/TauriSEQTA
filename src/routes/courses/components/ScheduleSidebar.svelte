<script lang="ts">
import { formatLessonDate, formatTime, isLessonReleased } from '../utils';
import type { TermSchedule, Lesson, CoursePayload } from '../types';

let { 
  schedule = [], 
  selectedLesson = null, 
  showingOverview = false, 
  coursePayload = null,
  onSelectLesson,
  onSelectOverview
}: {
  schedule?: TermSchedule[];
  selectedLesson?: Lesson | null;
  showingOverview?: boolean;
  coursePayload?: CoursePayload | null;
  onSelectLesson: (data: { termSchedule: TermSchedule; lesson: Lesson; lessonIndex: number }) => void;
  onSelectOverview: () => void;
} = $props();

let filteredSchedule = $derived(schedule.map(termSchedule => ({
  ...termSchedule,
  l: termSchedule.l.filter(isLessonReleased)
})).filter(termSchedule => termSchedule.l.length > 0));

function findTodayLesson() {
  const today = new Date();
  const todayDateString = today.toISOString().split('T')[0];
  
  // First, look for lessons happening today that are released
  for (const termSchedule of filteredSchedule) {
    for (let lessonIndex = 0; lessonIndex < termSchedule.l.length; lessonIndex++) {
      const lesson = termSchedule.l[lessonIndex];
      const lessonDate = new Date(lesson.d).toISOString().split('T')[0];
      
      if (lessonDate === todayDateString) {
        const lessonDateTime = new Date(`${lesson.d}T${lesson.s}`);
        const lessonEndTime = new Date(`${lesson.d}T${lesson.e}`);
        
        // If lesson is currently happening or hasn't started yet today
        if (lessonEndTime >= today) {
          // Find the original termSchedule and lessonIndex from the unfiltered schedule
          const originalTermSchedule = schedule.find(ts => ts.t === termSchedule.t && ts.w === termSchedule.w);
          if (originalTermSchedule) {
            const originalLessonIndex = originalTermSchedule.l.findIndex(l => l === lesson);
            if (originalLessonIndex >= 0) {
              return { 
                termSchedule: originalTermSchedule, 
                lesson, 
                lessonIndex: originalLessonIndex,
                type: 'today'
              };
            }
          }
        }
      }
    }
  }
  
  // If no lesson found for today, try to find next upcoming released lesson
  for (const termSchedule of filteredSchedule) {
    for (let lessonIndex = 0; lessonIndex < termSchedule.l.length; lessonIndex++) {
      const lesson = termSchedule.l[lessonIndex];
      const lessonDateTime = new Date(`${lesson.d}T${lesson.s}`);
      
      if (lessonDateTime > today) {
        // Find the original termSchedule and lessonIndex
        const originalTermSchedule = schedule.find(ts => ts.t === termSchedule.t && ts.w === termSchedule.w);
        if (originalTermSchedule) {
          const originalLessonIndex = originalTermSchedule.l.findIndex(l => l === lesson);
          if (originalLessonIndex >= 0) {
            return { 
              termSchedule: originalTermSchedule, 
              lesson, 
              lessonIndex: originalLessonIndex,
              type: 'upcoming'
            };
          }
        }
      }
    }
  }
  
  // If no upcoming lessons, find the last (most recent) lesson
  let lastLesson = null;
  for (const termSchedule of filteredSchedule) {
    for (let lessonIndex = 0; lessonIndex < termSchedule.l.length; lessonIndex++) {
      const lesson = termSchedule.l[lessonIndex];
      const originalTermSchedule = schedule.find(ts => ts.t === termSchedule.t && ts.w === termSchedule.w);
      if (originalTermSchedule) {
        const originalLessonIndex = originalTermSchedule.l.findIndex(l => l === lesson);
        if (originalLessonIndex >= 0) {
          lastLesson = { 
            termSchedule: originalTermSchedule, 
            lesson, 
            lessonIndex: originalLessonIndex,
            type: 'last'
          };
        }
      }
    }
  }
  
  return lastLesson;
}

function jumpToToday() {
  const todayLesson = findTodayLesson();
  if (todayLesson) {
    onSelectLesson({
      termSchedule: todayLesson.termSchedule,
      lesson: todayLesson.lesson,
      lessonIndex: todayLesson.lessonIndex
    });
  }
}

let jumpTarget = $derived(findTodayLesson());

function getJumpButtonText(target: any) {
  if (!target) return { title: '', subtitle: '' };
  
  switch (target.type) {
    case 'today':
      return {
        title: 'Jump to: Today\'s Lesson',
        subtitle: target.lesson.p
      };
    case 'upcoming':
      return {
        title: 'Jump to: Next Lesson',
        subtitle: `${target.lesson.p} ${formatLessonDate(target.lesson.d)}`
      };
    case 'last':
      return {
        title: 'Jump to: Latest Lesson',
        subtitle: `${target.lesson.p} ${formatLessonDate(target.lesson.d)}`
      };
    default:
      return { title: '', subtitle: '' };
  }
}
</script>

<aside class="flex flex-col w-64 h-[calc(100vh-4rem)] border-r bg-white/80 dark:bg-slate-800/50 backdrop-blur-sm border-gray-300/50 dark:border-slate-700/50">
  <div class="px-4 py-3 border-b border-gray-300/50 dark:border-slate-700/50 shrink-0">
    <h3 class="text-lg font-bold text-gray-900 dark:text-white animate-fade-in">Course Content</h3>
  </div>
  <div class="overflow-y-auto flex-1 min-h-0">
    <div class="sticky top-0 z-10 bg-white/90 dark:bg-slate-800/80 backdrop-blur-md border-b border-gray-300/50 dark:border-slate-700/50">
      <button 
        class="w-full px-4 py-3 text-left hover:bg-white/50 dark:hover:bg-slate-800/50 border-l-2 border-transparent hover:accent-border transition-all duration-200 hover:translate-x-1 active:scale-95 animate-slide-in group"
        onclick={onSelectOverview}>
        <div class="font-medium text-gray-900 dark:text-white text-sm group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors duration-300">📚 Course Overview</div>
        <div class="text-xs text-gray-600 dark:text-slate-400 mt-1 group-hover:text-gray-900 dark:group-hover:text-slate-300 transition-colors duration-300">
          Main course content and resources
        </div>
      </button>

      {#if jumpTarget}
        {@const buttonText = getJumpButtonText(jumpTarget)}
        <button 
          class="w-full px-4 py-3 text-left hover:bg-white/50 dark:hover:bg-slate-800/50 border-l-2 border-transparent hover:accent-border transition-all duration-200 hover:translate-x-1 active:scale-95 animate-slide-in group"
          style="animation-delay: 0.1s;"
          onclick={jumpToToday}>
          <div class="font-medium text-gray-900 dark:text-white text-sm group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors duration-300">🕐 {buttonText.title}</div>
          <div class="text-xs text-gray-600 dark:text-slate-400 mt-1 group-hover:text-gray-900 dark:group-hover:text-slate-300 transition-colors duration-300">
            {buttonText.subtitle}
          </div>
        </button>
      {/if}
    </div>

    {#each filteredSchedule as termSchedule, i}
      <div class="mb-4 animate-slide-in" style="animation-delay: {0.2 + i * 0.1}s;">
        <div class="px-4 py-2 accent-bg backdrop-blur-sm text-sm font-semibold text-white border border-gray-300/50 dark:border-slate-700/50 group-hover:brightness-110 transition-all duration-300">
          Term {termSchedule.t} - Week {termSchedule.w}
        </div>
        {#each termSchedule.l as lesson, lessonIndex}
          {@const currentLessonContent = coursePayload?.w?.[termSchedule.n]?.[lessonIndex]}
          <button
            class="w-full px-4 py-2 text-left text-sm hover:bg-white/50 dark:hover:bg-slate-800/50 border-l-2 border-transparent hover:accent-border transition-all duration-200 hover:translate-x-1 active:scale-95 animate-slide-in group {selectedLesson === lesson && !showingOverview ? 'bg-white/50 dark:bg-slate-800/50 accent-border' : ''}"
            style="animation-delay: {0.3 + lessonIndex * 0.05}s;"
            onclick={() => onSelectLesson({ termSchedule, lesson, lessonIndex })}>
            <div class="font-medium text-gray-900 dark:text-white text-sm group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors duration-300">{lesson.p} {formatLessonDate(lesson.d)}</div>
            <div class="text-xs text-gray-600 dark:text-slate-400 mt-1 group-hover:text-gray-900 dark:group-hover:text-slate-300 transition-colors duration-300">
              {#if currentLessonContent?.t}
                {currentLessonContent.t}
              {:else}
                {formatTime(lesson.s)}-{formatTime(lesson.e)}
              {/if}
            </div>
          </button>
        {/each}
      </div>
    {/each}
    
    {#if filteredSchedule.length === 0}
      <div class="px-4 py-8 text-center text-gray-600 dark:text-slate-400 text-sm animate-fade-in">
        <div class="w-16 h-16 mx-auto mb-4 rounded-full border-4 border-accent/30 border-t-accent animate-spin"></div>
        No released lessons available
      </div>
    {/if}
  </div>
</aside> 

<style>
  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .animate-fade-in {
    opacity: 0;
    animation: fadeIn 0.3s ease-out forwards;
  }

  .animate-slide-in {
    opacity: 0;
    animation: slideIn 0.3s ease-out forwards;
  }

  .animate-bounce {
    animation: bounce 0.8s infinite;
  }

  @keyframes bounce {
    0%, 100% {
      transform: translateY(0);
    }
    50% {
      transform: translateY(-5px);
    }
  }
</style> 