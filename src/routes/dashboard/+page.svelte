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
    }

    let homeworkData = $state<HomeworkResponse | null>(null);
    let error = $state<string | null>(null);
    let loading = $state(true);

    // Todo list state
    let todos = $state<TodoItem[]>([]);
    let newTodoText = $state('');
    let newTodoDueDate = $state('');

    // Timer state
    let timerMinutes = $state(25);
    let timerSeconds = $state(0);
    let isTimerRunning = $state(false);
    let timerInterval: ReturnType<typeof setInterval> | null = null;

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

    function addTodo() {
        if (newTodoText.trim()) {
            todos = [...todos, {
                id: Date.now(),
                text: newTodoText.trim(),
                completed: false,
                dueDate: newTodoDueDate || undefined
            }];
            newTodoText = '';
            newTodoDueDate = '';
            saveTodos();
        }
    }

    function toggleTodo(id: number) {
        todos = todos.map(todo => 
            todo.id === id ? { ...todo, completed: !todo.completed } : todo
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
                    new Audio('/notification.mp3').play().catch(() => {});
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

    onMount(() => {
        console.log('Dashboard component mounted');
        fetchHomeworkData();
        loadTodos();
    });
</script>

<div class="space-y-8 max-w-6xl mx-auto py-10 px-4">
    <div class="flex justify-between items-center mb-6">
        <h1 class="text-2xl font-bold text-white">Dashboard</h1>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <!-- Left Column: Homework and Todo List -->
        <div class="space-y-8">
            <!-- Homework Section -->
            <div class="bg-slate-900 rounded-xl shadow-lg border border-slate-800">
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
                                <div class="bg-slate-800 rounded-xl shadow-lg border-l-8" style="border-color: #1a73e8;">
                                    <div class="px-6 pt-5 pb-3">
                                        <h3 class="text-lg font-bold mb-2 text-white">{homework.title}</h3>
                                        <div class="flex flex-col gap-3">
                                            {#each homework.items as item}
                                                <div class="flex gap-2 items-start bg-slate-700 rounded-lg px-4 py-3 border border-slate-600">
                                                    <span class="text-xl text-blue-500 mt-1">•</span>
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
            <div class="bg-slate-900 rounded-xl shadow-lg border border-slate-800">
                <div class="px-6 py-4 border-b border-slate-800">
                    <h2 class="text-xl font-bold text-white">Todo List</h2>
                </div>
                <div class="p-6">
                    <form onsubmit={(e) => { e.preventDefault(); addTodo(); }} class="mb-6">
                        <div class="flex gap-4">
                            <input
                                type="text"
                                bind:value={newTodoText}
                                placeholder="Add a new task..."
                                class="flex-1 px-4 py-2 rounded-lg bg-slate-800 text-white border border-slate-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
                            />
                            <input
                                type="date"
                                bind:value={newTodoDueDate}
                                class="px-4 py-2 rounded-lg bg-slate-800 text-white border border-slate-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
                            />
                            <button
                                type="submit"
                                class="px-6 py-2 rounded-lg bg-blue-600 text-white hover:bg-blue-700 transition-colors"
                            >
                                Add
                            </button>
                        </div>
                    </form>

                    <div class="space-y-3">
                        {#each todos as todo (todo.id)}
                            <div class="flex items-center gap-3 p-4 bg-slate-800 rounded-lg border border-slate-700">
                                <input
                                    type="checkbox"
                                    checked={todo.completed}
                                    onchange={() => toggleTodo(todo.id)}
                                    class="w-5 h-5 rounded border-slate-600 text-blue-600 focus:ring-blue-500"
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
                                </div>
                                <button
                                    onclick={() => deleteTodo(todo.id)}
                                    class="p-2 text-slate-400 hover:text-red-500 transition-colors"
                                >
                                    ×
                                </button>
                            </div>
                        {/each}
                    </div>
                </div>
            </div>
        </div>

        <!-- Right Column: Timer -->
        <div class="bg-slate-900 rounded-xl shadow-lg border border-slate-800">
            <div class="px-6 py-4 border-b border-slate-800">
                <h2 class="text-xl font-bold text-white">Focus Timer</h2>
            </div>
            <div class="p-6">
                <div class="flex flex-col items-center">
                    <div class="text-6xl font-bold text-white mb-8">
                        {String(timerMinutes).padStart(2, '0')}:{String(timerSeconds).padStart(2, '0')}
                    </div>
                    <div class="flex gap-4">
                        <button
                            onclick={startTimer}
                            disabled={isTimerRunning}
                            class="px-6 py-3 rounded-lg bg-blue-600 text-white hover:bg-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            Start
                        </button>
                        <button
                            onclick={stopTimer}
                            disabled={!isTimerRunning}
                            class="px-6 py-3 rounded-lg bg-red-600 text-white hover:bg-red-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            Stop
                        </button>
                        <button
                            onclick={resetTimer}
                            class="px-6 py-3 rounded-lg bg-slate-700 text-white hover:bg-slate-600 transition-colors"
                        >
                            Reset
                        </button>
                    </div>
                    <div class="mt-8 grid grid-cols-3 gap-4">
                        <button
                            onclick={() => { timerMinutes = 25; timerSeconds = 0; }}
                            class="px-4 py-2 rounded-lg bg-slate-800 text-white hover:bg-slate-700 transition-colors"
                        >
                            25m
                        </button>
                        <button
                            onclick={() => { timerMinutes = 45; timerSeconds = 0; }}
                            class="px-4 py-2 rounded-lg bg-slate-800 text-white hover:bg-slate-700 transition-colors"
                        >
                            45m
                        </button>
                        <button
                            onclick={() => { timerMinutes = 60; timerSeconds = 0; }}
                            class="px-4 py-2 rounded-lg bg-slate-800 text-white hover:bg-slate-700 transition-colors"
                        >
                            60m
                        </button>
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