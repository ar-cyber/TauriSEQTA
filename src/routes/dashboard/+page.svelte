<script lang="ts">
    import { onMount } from 'svelte';
    import { seqtaFetch } from '../../utils/netUtil';
    import { writable } from 'svelte/store';

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
    let error = $state<string | null>(null);
    let loading = $state(true);

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

    async function fetchHomeworkData() {
        try {
            loading = true;
            error = null;
            console.log('Making POST request to homework endpoint...');
            const response = await seqtaFetch('/seqta/student/dashlet/summary/homework', {
                method: 'POST',
                body: {},
                params: {"majhvjju": ""}
            });
            homeworkData = JSON.parse(response);
        } catch (e: any) {
            console.error('Error details:', e);
            error = e.toString();
        } finally {
            loading = false;
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

    onMount(() => {
        console.log('Dashboard component mounted');
        fetchHomeworkData();
        loadTodos();
    });
</script>

<div class="min-h-screen bg-slate-950">
    <div class="max-w-[2000px] mx-auto px-4 py-8">
        <div class="flex justify-between items-center mb-8">
            <h1 class="text-3xl font-bold text-white">Dashboard</h1>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
            <!-- Left Column: Homework and Todo List -->
            <div class="space-y-8">
                <!-- Homework Section -->
                <div class="bg-slate-900/50 backdrop-blur-sm rounded-xl shadow-lg border border-slate-800">
                    <div class="px-6 py-4 border-b border-slate-800">
                        <h2 class="text-xl font-bold text-white">Homework</h2>
                    </div>
                    <div class="p-6">
                        {#if loading}
                            <div class="flex justify-center items-center py-12">
                                <p class="text-slate-400">Loading homework data…</p>
                            </div>
                        {:else if error}
                            <div class="bg-red-100 text-red-700 rounded-lg px-6 py-4 mb-4 border border-red-200">
                                <p>Error: {error}</p>
                            </div>
                        {:else if homeworkData}
                            <div class="flex flex-col gap-6">
                                {#each homeworkData.payload as homework}
                                    <div class="bg-slate-800/50 backdrop-blur-sm rounded-xl shadow-lg border-l-8" style="border-color: var(--accent);">
                                        <div class="px-6 pt-5 pb-3">
                                            <h3 class="text-lg font-bold mb-2 text-white">{homework.title}</h3>
                                            <div class="flex flex-col gap-3">
                                                {#each homework.items as item}
                                                    <div class="flex gap-2 items-start bg-slate-700/50 backdrop-blur-sm rounded-lg px-4 py-3 border border-slate-600">
                                                        <span class="text-xl accent-text mt-1">•</span>
                                                        <span class="text-slate-50">{item}</span>
                                                    </div>
                                                {/each}
                                            </div>
                                        </div>
                                    </div>
                                {/each}
                            </div>
                        {:else}
                            <p class="text-center text-slate-400">No homework data available</p>
                        {/if}
                    </div>
                </div>

                <!-- Todo List Section -->
                <div class="bg-slate-900/50 backdrop-blur-sm rounded-xl shadow-lg border border-slate-800">
                    <div class="px-6 py-4 border-b border-slate-800">
                        <h2 class="text-xl font-bold text-white">Todo List</h2>
                    </div>
                    <div class="p-6">
                        <form onsubmit={(e) => { e.preventDefault(); addTodo(); }} class="mb-6">
                            <div class="flex flex-col gap-4">
                                <div class="flex gap-4">
                                    <input
                                        type="text"
                                        bind:value={newTodoText}
                                        placeholder="Add a new task..."
                                        class="flex-1 px-4 py-2 rounded-lg bg-slate-800/50 backdrop-blur-sm text-white border border-slate-700 focus:outline-none focus:ring-2 focus:ring-accent"
                                    />
                                    <input
                                        type="date"
                                        bind:value={newTodoDueDate}
                                        class="px-4 py-2 rounded-lg bg-slate-800/50 backdrop-blur-sm text-white border border-slate-700 focus:outline-none focus:ring-2 focus:ring-accent"
                                    />
                                </div>
                                <div class="flex gap-4">
                                    <select bind:value={newTodoPriority} class="px-4 py-2 rounded-lg bg-slate-800/50 text-white border border-slate-700">
                                        <option value="low">Low Priority</option>
                                        <option value="medium">Medium Priority</option>
                                        <option value="high">High Priority</option>
                                    </select>
                                    <input
                                        type="text"
                                        bind:value={newTodoTags}
                                        placeholder="Tags (comma separated)"
                                        class="flex-1 px-4 py-2 rounded-lg bg-slate-800/50 text-white border border-slate-700"
                                    />
                                    <select bind:value={newTodoRecurring} class="px-4 py-2 rounded-lg bg-slate-800/50 text-white border border-slate-700">
                                        <option value="none">No Repeat</option>
                                        <option value="daily">Daily</option>
                                        <option value="weekly">Weekly</option>
                                        <option value="monthly">Monthly</option>
                                    </select>
                                </div>
                                <div>
                                    <div class="flex gap-2 mb-2">
                                        <input
                                            type="text"
                                            bind:value={newSubtaskText}
                                            placeholder="Add subtask..."
                                            class="flex-1 px-4 py-2 rounded-lg bg-slate-800/50 text-white border border-slate-700"
                                        />
                                        <button type="button" onclick={addSubtask} class="px-4 py-2 rounded-lg bg-accent text-white">Add Subtask</button>
                                    </div>
                                    <div class="flex flex-wrap gap-2">
                                        {#each newSubtasks as st (st.id)}
                                            <span class="bg-slate-700 text-white px-3 py-1 rounded-lg flex items-center gap-2">
                                                {st.text}
                                                <button type="button" onclick={() => removeSubtask(st.id)} class="text-red-400 ml-2">×</button>
                                            </span>
                                        {/each}
                                    </div>
                                </div>
                                <button
                                    type="submit"
                                    class="px-6 py-2 rounded-lg bg-accent text-white hover:bg-accent/90 transition-colors self-end"
                                >
                                    Add
                                </button>
                            </div>
                        </form>

                        <div class="space-y-3">
                            {#each todos as todo (todo.id)}
                                <div class="flex flex-col gap-2 p-4 bg-slate-800/50 backdrop-blur-sm rounded-lg border border-slate-700">
                                    <div class="flex items-center gap-3">
                                        <input
                                            type="checkbox"
                                            checked={todo.completed}
                                            onchange={() => toggleTodo(todo.id)}
                                            class="w-5 h-5 rounded border-slate-600 text-accent focus:ring-accent"
                                        />
                                        <div class="flex-1">
                                            <p class="text-white {todo.completed ? 'line-through text-slate-400' : ''}">
                                                {todo.text}
                                            </p>
                                            {#if todo.dueDate}
                                                <p class="text-sm text-slate-400 mt-1">
                                                    Due: {new Date(todo.dueDate).toLocaleDateString()}
                                                </p>
                                            {/if}
                                            {#if todo.priority}
                                                <span class="inline-block px-2 py-0.5 rounded-full text-xs font-semibold mt-1 {todo.priority === 'high' ? 'bg-red-600' : todo.priority === 'medium' ? 'bg-yellow-500' : 'bg-green-600'} text-white">
                                                    {todo.priority.charAt(0).toUpperCase() + todo.priority.slice(1)} Priority
                                                </span>
                                            {/if}
                                            {#if todo.tags && todo.tags.length}
                                                <span class="inline-block ml-2 text-xs text-blue-300">
                                                    {#each todo.tags as tag, i}
                                                        #{tag}{i < todo.tags.length - 1 ? ', ' : ''}
                                                    {/each}
                                                </span>
                                            {/if}
                                            {#if todo.recurring && todo.recurring !== 'none'}
                                                <span class="inline-block ml-2 text-xs text-purple-300">
                                                    {todo.recurring.charAt(0).toUpperCase() + todo.recurring.slice(1)}
                                                </span>
                                            {/if}
                                        </div>
                                        <button
                                            onclick={() => deleteTodo(todo.id)}
                                            class="p-2 text-slate-400 hover:text-red-500 transition-colors"
                                        >
                                            ×
                                        </button>
                                    </div>
                                    {#if todo.subtasks && todo.subtasks.length}
                                        <div class="ml-8 flex flex-col gap-1 mt-2">
                                            {#each todo.subtasks as st (st.id)}
                                                <div class="flex items-center gap-2">
                                                    <input
                                                        type="checkbox"
                                                        checked={st.completed}
                                                        onchange={() => toggleSubtask(todo.id, st.id)}
                                                        class="w-4 h-4 rounded border-slate-600 text-accent"
                                                    />
                                                    <span class="text-sm text-white {st.completed ? 'line-through text-slate-400' : ''}">{st.text}</span>
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
            <div class="bg-slate-900/50 backdrop-blur-sm rounded-xl shadow-lg border border-slate-800">
                <div class="px-6 py-4 border-b border-slate-800">
                    <h2 class="text-xl font-bold text-white">Focus Timer</h2>
                </div>
                <div class="p-6">
                    <div class="flex flex-col items-center">
                        <div class="text-7xl font-bold text-white mb-8 font-mono">
                            {String(timerMinutes).padStart(2, '0')}:{String(timerSeconds).padStart(2, '0')}
                        </div>
                        <div class="flex justify-center gap-4 mt-6">
                            <button 
                                onclick={startTimer}
                                class="px-6 py-3 rounded-lg text-white font-semibold transition-all duration-200 hover:scale-105 hover:shadow-lg accent-bg"
                            >
                                Start Timer
                            </button>
                            <button
                                onclick={stopTimer}
                                disabled={!isTimerRunning}
                                class="px-8 py-3 rounded-lg bg-red-600 text-white hover:bg-red-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                            >
                                Stop
                            </button>
                            <button
                                onclick={resetTimer}
                                class="px-8 py-3 rounded-lg bg-slate-700 text-white hover:bg-slate-600 transition-colors"
                            >
                                Reset
                            </button>
                        </div>
                        <div class="mt-8 grid grid-cols-3 gap-4">
                            <button
                                onclick={() => { timerMinutes = 25; timerSeconds = 0; }}
                                class="px-6 py-3 rounded-lg bg-slate-800/50 backdrop-blur-sm text-white hover:bg-slate-700 transition-colors"
                            >
                                25m
                            </button>
                            <button
                                onclick={() => { timerMinutes = 45; timerSeconds = 0; }}
                                class="px-6 py-3 rounded-lg bg-slate-800/50 backdrop-blur-sm text-white hover:bg-slate-700 transition-colors"
                            >
                                45m
                            </button>
                            <button
                                onclick={() => { timerMinutes = 60; timerSeconds = 0; }}
                                class="px-6 py-3 rounded-lg bg-slate-800/50 backdrop-blur-sm text-white hover:bg-slate-700 transition-colors"
                            >
                                60m
                            </button>
                        </div>
                        <div class="mt-8 flex flex-col items-center gap-4">
                            <div class="flex gap-4 items-center">
                                <input
                                    type="number"
                                    bind:value={customMinutes}
                                    placeholder="Minutes"
                                    min="0"
                                    class="w-28 px-4 py-3 rounded-lg bg-slate-800/50 backdrop-blur-sm text-white border border-slate-700 focus:outline-none focus:ring-2 focus:ring-accent"
                                />
                                <span class="text-white text-2xl">:</span>
                                <input
                                    type="number"
                                    bind:value={customSeconds}
                                    placeholder="Seconds"
                                    min="0"
                                    max="59"
                                    class="w-28 px-4 py-3 rounded-lg bg-slate-800/50 backdrop-blur-sm text-white border border-slate-700 focus:outline-none focus:ring-2 focus:ring-accent"
                                />
                            </div>
                            <button
                                onclick={setCustomTime}
                                class="px-8 py-3 rounded-lg bg-slate-800/50 backdrop-blur-sm text-white hover:bg-slate-700 transition-colors"
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

<style>
    :global(body) {
        background: var(--background);
    }
</style> 