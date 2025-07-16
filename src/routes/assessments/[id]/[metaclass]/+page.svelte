<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { seqtaFetch, uploadSeqtaFile } from '../../../../utils/netUtil';
  import {
    Icon,
    ArrowLeft,
    DocumentText,
    VideoCamera,
    PresentationChartLine,
    Photo,
    Plus,
  } from 'svelte-hero-icons';
  import { openUrl } from '@tauri-apps/plugin-opener';
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';

  let assessmentData: any = $state(null);
  let loading = $state(true);
  let error = $state('');
  let tab = $state('overview'); // default tab
  let firstCriterion: any = $state(null);
  let teacherFiles: any[] = $state([]);
  let allSubmissions: any[] = $state([]);
  let uploading = $state(false);
  let uploadError = $state('');

  // Define available tabs based on assessment data
  const availableTabs = $derived([
    { id: 'overview', label: 'Overview', icon: '📋' },
    { id: 'details', label: 'Details', icon: '📊' },
    { id: 'submissions', label: 'Submissions', icon: '📁' }
  ]);

  async function loadAssessmentDetails() {
    try {
      const res = await seqtaFetch('/seqta/student/assessment/get?', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json; charset=utf-8' },
        body: {
          assessment: parseInt($page.params.id),
          student: 69,
          metaclass: parseInt($page.params.metaclass),
        },
      });
      assessmentData = JSON.parse(res).payload;
      firstCriterion = assessmentData?.criteria?.[0] ?? null;

      // Fetch teacher files (submissions)
      const subRes = await seqtaFetch('/seqta/student/assessment/submissions/get?', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json; charset=utf-8' },
        body: {
          assessment: parseInt($page.params.id),
          student: 69,
          metaclass: parseInt($page.params.metaclass),
        },
      });
      const submissions = JSON.parse(subRes).payload;
      allSubmissions = submissions;
      teacherFiles = submissions.filter((f: any) => f.staff || f.created_by !== 69);
    } catch (e) {
      console.error('Failed to load assessment details:', e);
      error = 'Failed to load assessment details';
    } finally {
      loading = false;
    }
  }

  function getFileIcon(mimetype: string) {
    if (mimetype.startsWith('video/')) return VideoCamera;
    if (mimetype.includes('presentation')) return PresentationChartLine;
    if (mimetype.includes('image')) return Photo;
    return DocumentText;
  }

  function formatFileSize(size: string) {
    const bytes = parseInt(size);
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024 * 1024) return Math.round(bytes / 1024) + ' KB';
    return Math.round(bytes / (1024 * 1024)) + ' MB';
  }

  async function handleFileUpload() {
    uploading = true;
    uploadError = '';

    try {
      // Open file dialog to select files
      const selected = await open({
        multiple: true,
        filters: [{
          name: 'All Files',
          extensions: ['*']
        }]
      });

      if (!selected) {
        uploading = false;
        return;
      }

      const files = Array.isArray(selected) ? selected : [selected];

      for (const filePath of files) {
        // Extract filename from path
        const fileName = filePath.split(/[/\\]/).pop() || 'unknown';
        
        // First, upload the file
        const uploadResponse = await uploadSeqtaFile(fileName, filePath);
        const uploadResult = JSON.parse(uploadResponse);
        
        if (uploadResult.status === '200' && uploadResult.payload) {
          // Then link the file to the assessment
          const linkResponse = await seqtaFetch('/seqta/student/assessment/submissions/save', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json; charset=utf-8' },
            body: {
              action: 'link',
              assID: parseInt($page.params.id),
              metaclass: parseInt($page.params.metaclass),
              files: [uploadResult.payload.id]
            },
          });
          
          const linkResult = JSON.parse(linkResponse);
          if (linkResult.status === '200') {
            // Reload submissions to show the new file
            await loadAssessmentDetails();
          } else {
            throw new Error('Failed to link file to assessment');
          }
        } else {
          throw new Error('Failed to upload file');
        }
      }
    } catch (e) {
      console.error('File upload error:', e);
      uploadError = e instanceof Error ? e.message : 'Upload failed';
    } finally {
      uploading = false;
    }
  }

  onMount(loadAssessmentDetails);
</script>

<div class="min-h-screen dark:bg-slate-950 bg-slate-100">
  <!-- Header -->
  <div
    class="flex sticky top-0 z-10 gap-4 items-center px-6 py-4 bg-opacity-80 border-b backdrop-blur-sm dark:bg-slate-900 bg-slate-100 dark:border-slate-800 border-slate-200">
    <a
      href="/assessments"
      class="flex gap-2 items-center transition-colors duration-200 text-slate-400 hover:text-slate-600 dark:hover:text-white">
      <Icon src={ArrowLeft} class="w-5 h-5" />
      <span>Back to Assessments</span>
    </a>
  </div>

  <!-- Tabs -->
  <div class="container px-6 pt-6 mx-auto">
    <div class="flex gap-2 border-b dark:border-slate-800 border-slate-200">
      {#each availableTabs as tabItem}
        <button
          class="flex gap-2 items-center px-4 py-2 text-xs font-semibold tracking-wide uppercase border-b-2 transition-all duration-200 focus:outline-none hover:scale-105"
          onclick={() => (tab = tabItem.id)}
          class:border-accent-ring={tab === tabItem.id}
          class:text-accent-bg={tab === tabItem.id}
          class:text-slate-400={tab !== tabItem.id}
          class:border-transparent={tab !== tabItem.id}>
          <span class="text-sm">{tabItem.icon}</span>
          <span>{tabItem.label}</span>
        </button>
      {/each}
    </div>
  </div>

  <!-- Content -->
  <div class="container px-6 py-8 mx-auto">
    {#if loading}
      <div class="flex justify-center items-center h-64">
        <div class="w-12 h-12 rounded-full border-t-2 border-b-2 border-accent-500 animate-spin">
        </div>
      </div>
    {:else if error}
      <div class="flex justify-center items-center h-64">
        <div class="text-red-500 animate-pulse">{error}</div>
      </div>
    {:else if assessmentData}
      {#if tab === 'overview'}
        <!-- Overview Tab: Description and Resources -->
        <div class="grid gap-8 animate-fade-in">
          <div
            class="grid gap-6 p-6 rounded-2xl transition-all duration-300 dark:bg-slate-900 bg-slate-100 hover:shadow-lg hover:shadow-accent-500/10">
            <h1 class="mb-2 text-2xl font-bold">Assessment Overview</h1>
            
            <!-- Basic Assessment Info -->
            <div class="grid gap-4 p-4 rounded-xl dark:bg-slate-800 bg-slate-200">
              <div class="flex justify-between items-center">
                <span class="text-sm font-medium text-slate-600 dark:text-slate-400">Status:</span>
                <span class="px-2 py-1 text-xs font-medium rounded-full {assessmentData.marked ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200' : 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200'}">
                  {assessmentData.marked ? 'Marked' : 'Not Marked'}
                </span>
              </div>
              {#if assessmentData.submissionSettings}
                <div class="flex justify-between items-center">
                  <span class="text-sm font-medium text-slate-600 dark:text-slate-400">File Submission:</span>
                  <span class="px-2 py-1 text-xs font-medium rounded-full {assessmentData.submissionSettings.fileSubmissionEnabled ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200' : 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200'}">
                    {assessmentData.submissionSettings.fileSubmissionEnabled ? 'Enabled' : 'Disabled'}
                  </span>
                </div>
              {/if}
            </div>
            
            <!-- Description Section -->
            <div>
              <h2 class="text-lg font-semibold mb-3">Description</h2>
              {#if assessmentData.description}
                <div class="max-w-none prose prose-invert">
                  <div class="whitespace-pre-line text-slate-300">
                    {@html assessmentData.description}
                  </div>
                </div>
              {:else}
                <div class="flex flex-col items-center justify-center py-6 text-center">
                  <div class="w-12 h-12 mb-3 rounded-full bg-slate-200 dark:bg-slate-700 flex items-center justify-center">
                    <Icon src={DocumentText} class="w-6 h-6 text-slate-400" />
                  </div>
                  <h3 class="text-base font-semibold text-slate-900 dark:text-white mb-1">No Description Available</h3>
                  <p class="text-sm text-slate-600 dark:text-slate-400">
                    This assessment doesn't have a detailed description yet.
                  </p>
                </div>
              {/if}
            </div>
          </div>
          
          {#if assessmentData.resources?.length}
            <div
              class="grid gap-6 p-6 rounded-2xl transition-all duration-300 dark:bg-slate-900 bg-slate-100 hover:shadow-lg hover:shadow-accent-500/10">
              <h2 class="text-xl font-bold">Resources</h2>
              <div class="grid gap-4">
                {#each assessmentData.resources as resource}
                  <div
                    class="flex items-center gap-4 p-4 rounded-xl dark:bg-slate-800 bg-slate-200 transition-all duration-300 hover:scale-[1.02] hover:shadow-lg hover:shadow-accent-500/5">
                    <Icon
                      src={getFileIcon(resource.userfile.mimetype)}
                      class="w-6 h-6 text-accent-400" />
                    <div class="flex-1 min-w-0">
                      <div class="text-sm font-medium truncate">
                        {resource.name}
                      </div>
                      <div class="text-xs text-slate-400">
                        {formatFileSize(resource.userfile.size)}
                      </div>
                    </div>
                    <button
                      type="button"
                      class="px-3 py-1 text-sm font-medium rounded-lg transition-all duration-200 text-white bg-accent-bg hover:bg-accent-ring"
                      onclick={async () => {
                        try {
                          const url = await invoke('get_seqta_file', {
                            fileType: 'resource',
                            uuid: resource.userfile.uuid,
                          });
                          if (typeof url === 'string') {
                            await openUrl(url);
                          }
                        } catch (e) {
                          // Optionally handle error
                        }
                      }}>
                      Download
                    </button>
                  </div>
                {/each}
              </div>
            </div>
          {:else}
            <div
              class="grid gap-6 p-6 rounded-2xl transition-all duration-300 dark:bg-slate-900 bg-slate-100 hover:shadow-lg hover:shadow-accent-500/10">
              <h2 class="text-xl font-bold">Resources</h2>
              <div class="flex flex-col items-center justify-center py-8 text-center">
                <div class="w-16 h-16 mb-4 rounded-full bg-slate-200 dark:bg-slate-700 flex items-center justify-center">
                  <Icon src={DocumentText} class="w-8 h-8 text-slate-400" />
                </div>
                <h3 class="text-lg font-semibold text-slate-900 dark:text-white mb-2">No Resources Available</h3>
                <p class="text-slate-600 dark:text-slate-400">
                  This assessment doesn't have any attached resources or files.
                </p>
              </div>
            </div>
          {/if}
        </div>
      {:else if tab === 'details'}
        <!-- Details Tab: Grade, Feedback, Teacher Files -->
        <div class="grid gap-8 animate-fade-in">
          <div
            class="p-6 rounded-2xl transition-all duration-300 dark:bg-slate-900 bg-slate-100 hover:shadow-lg hover:shadow-accent-500/10">
            <!-- Grade Bar -->
            {#if assessmentData.marked && firstCriterion && firstCriterion.results}
              <div class="mb-4">
                <div class="mb-2 text-2xl font-bold">Grade</div>
                <div
                  class="overflow-hidden relative w-full h-16 rounded-xl border transition-all duration-300 dark:bg-slate-800 bg-slate-200 dark:border-slate-700 border-slate-200 hover:shadow-lg hover:shadow-accent-500/10">
                  <div
                    class="absolute top-0 left-0 h-full bg-accent-600 transition-all duration-500"
                    style="width: {firstCriterion.results.percentage || 0}%">
                  </div>
                  <div class="flex relative z-10 justify-center items-center h-full">
                    <span
                      class="text-3xl font-extrabold tracking-wide text-white drop-shadow animate-fade-in"
                      style="text-shadow: 0 2px 8px #000a">
                      {firstCriterion.results.grade ||
                        (firstCriterion.results.percentage ? firstCriterion.results.percentage.toFixed(2) + '%' : 'No Grade')}
                    </span>
                  </div>
                </div>
              </div>
            {:else if assessmentData.marked && firstCriterion}
              <div class="mb-4">
                <div class="mb-2 text-2xl font-bold">Grade</div>
                <div class="p-4 rounded-xl border dark:bg-slate-800 bg-slate-200 dark:border-slate-700 border-slate-200">
                  <div class="text-center text-slate-600 dark:text-slate-400">
                    Grade not yet available
                  </div>
                </div>
              </div>
            {/if}
            <!-- End Grade Bar -->
            <div class="flex flex-col gap-4 mb-6 md:flex-row md:items-center md:justify-between">
              <h1 class="text-2xl font-bold">Teacher marking and feedback</h1>
            </div>
            {#if assessmentData.marked && assessmentData.criteria?.length}
              <div class="mb-2 font-semibold">Achievement</div>
            {/if}
            {#if assessmentData.marked && assessmentData.engagement?.feedbackComment}
              <div
                class="p-4 mb-4 rounded-xl transition-all duration-300 dark:bg-slate-800 bg-slate-200 hover:shadow-lg hover:shadow-accent-500/5">
                <div class="mb-1 font-semibold">Teacher feedback</div>
                <div class="dark:text-slate-300 text-slate-700">
                  {assessmentData.engagement.feedbackComment}
                </div>
              </div>
            {:else if assessmentData.marked}
              <div class="flex flex-col items-center justify-center py-8 text-center">
                <div class="w-16 h-16 mb-4 rounded-full bg-slate-200 dark:bg-slate-700 flex items-center justify-center">
                  <Icon src={DocumentText} class="w-8 h-8 text-slate-400" />
                </div>
                <h3 class="text-lg font-semibold text-slate-900 dark:text-white mb-2">No Feedback Available</h3>
                <p class="text-slate-600 dark:text-slate-400">
                  This assessment has been marked but no detailed feedback is available yet.
                </p>
              </div>
            {:else}
              <div class="flex flex-col items-center justify-center py-8 text-center">
                <div class="w-16 h-16 mb-4 rounded-full bg-slate-200 dark:bg-slate-700 flex items-center justify-center">
                  <Icon src={DocumentText} class="w-8 h-8 text-slate-400" />
                </div>
                <h3 class="text-lg font-semibold text-slate-900 dark:text-white mb-2">Assessment Not Yet Marked</h3>
                <p class="text-slate-600 dark:text-slate-400">
                  This assessment hasn't been marked yet. Check back later for grades and feedback.
                </p>
              </div>
            {/if}
          </div>
        </div>
      {:else if tab === 'submissions'}
        <!-- Submissions Tab -->
        <div class="grid gap-8 animate-fade-in">
          <div
            class="p-6 rounded-2xl transition-all duration-300 dark:bg-slate-900 bg-slate-100 hover:shadow-lg hover:shadow-accent-500/10">
            <div class="flex justify-between items-center mb-4">
              <h1 class="text-2xl font-bold">Submissions</h1>
              <button
                type="button"
                class="flex gap-2 items-center px-4 py-2 text-sm font-medium rounded-lg transition-all duration-200 text-white bg-accent-bg hover:bg-accent-ring disabled:opacity-50 disabled:cursor-not-allowed"
                onclick={handleFileUpload}
                disabled={uploading}>
                {#if uploading}
                  <div class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                  Uploading...
                {:else}
                  <Icon src={Plus} class="w-4 h-4" />
                  Upload Files
                {/if}
              </button>
            </div>
            
            {#if uploadError}
              <div class="p-3 mb-4 rounded-lg bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-700">
                <p class="text-sm text-red-700 dark:text-red-400">{uploadError}</p>
              </div>
            {/if}
            
            {#if allSubmissions.filter((f) => !f.staff).length === 0}
              <div class="flex flex-col items-center justify-center py-12 text-center">
                <div class="w-16 h-16 mb-4 rounded-full bg-slate-200 dark:bg-slate-700 flex items-center justify-center">
                  <Icon src={DocumentText} class="w-8 h-8 text-slate-400" />
                </div>
                <h3 class="text-lg font-semibold text-slate-900 dark:text-white mb-2">No Submissions Yet</h3>
                <p class="text-slate-600 dark:text-slate-400 mb-6 max-w-md">
                  You haven't submitted any files for this assessment yet. Use the upload button above to add your work.
                </p>
              </div>
            {:else}
              <div class="grid gap-3">
                {#each allSubmissions.filter((f) => !f.staff) as file}
                  <div
                    class="flex items-center gap-4 p-3 rounded-xl border transition-all duration-300 hover:scale-[1.02] hover:shadow-lg bg-slate-200/50 dark:bg-slate-800">
                    <Icon src={getFileIcon(file.mimetype)} class="w-5 h-5 text-accent-400" />
                    <div class="flex-1 min-w-0">
                      <div class="text-sm font-medium truncate">
                        {file.filename}
                      </div>
                      <div class="flex gap-2 items-center text-xs text-slate-400">
                        <span>{formatFileSize(file.size)}</span>
                        <span>•</span>
                        <span>Student</span>
                        <span>•</span>
                        <span>{new Date(file.created_date).toLocaleString()}</span>
                      </div>
                    </div>
                    <button
                      type="button"
                      disabled
                      class="px-3 py-1 text-sm font-medium rounded-lg transition-all duration-200 cursor-not-allowed text-slate-300 bg-slate-600 hover:bg-slate-500">
                      Download
                    </button>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        </div>
      {/if}
    {/if}
  </div>
</div>

<style>
  @keyframes fade-in {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .animate-fade-in {
    animation: fade-in 0.3s ease-out;
  }

  /* Add any additional styles here */
</style>
