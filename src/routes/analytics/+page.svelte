<script lang="ts">
import { seqtaFetch } from '../../utils/netUtil';
import { invoke } from '@tauri-apps/api/core';

let loading = false;
let progress = 0;
let total = 0;
let error = '';
let analyticsData: any[] = [];
let saved = false;

const studentId = 69;

async function fetchAllAssessments() {
  loading = true;
  error = '';
  saved = false;
  analyticsData = [];
  progress = 0;
  try {
    // Fetch all assessments (upcoming + past for all subjects)
    const subjectsRes = await seqtaFetch('/seqta/student/load/subjects?', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json; charset=utf-8' },
      body: {}
    });
    const subjects = JSON.parse(subjectsRes).payload?.find((c: any) => c.active)?.subjects || [];
    const activeCodes = subjects.map((s: any) => s.code);

    const upcomingRes = await seqtaFetch('/seqta/student/assessment/list/upcoming?', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json; charset=utf-8' },
      body: { student: studentId }
    });
    const upcoming = JSON.parse(upcomingRes).payload.filter((a: any) => activeCodes.includes(a.code));

    // Fetch all past assessments for each subject
    const pastPromises = subjects.map((subject: any) =>
      seqtaFetch('/seqta/student/assessment/list/past?', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json; charset=utf-8' },
        body: {
          programme: subject.programme,
          metaclass: subject.metaclass,
          student: studentId
        }
      })
    );
    const pastResponses = await Promise.all(pastPromises);
    const past = pastResponses.map((res: any) => JSON.parse(res).payload.tasks || []).flat();

    // Combine and deduplicate
    const allAssessments = [...upcoming, ...past];
    const uniqueMap = new Map();
    allAssessments.forEach((a: any) => {
      if (!uniqueMap.has(a.id)) uniqueMap.set(a.id, a);
    });
    const uniqueAssessments = Array.from(uniqueMap.values());
    total = uniqueAssessments.length;

    // For each assessment, fetch details and submissions
    for (let i = 0; i < uniqueAssessments.length; i++) {
      const a = uniqueAssessments[i];
      try {
        const detailsRes = await seqtaFetch('/seqta/student/assessment/get?', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json; charset=utf-8' },
          body: {
            assessment: a.id,
            student: studentId,
            metaclass: a.metaclass
          }
        });
        const details = JSON.parse(detailsRes).payload;
        const submissionsRes = await seqtaFetch('/seqta/student/assessment/submissions/get?', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json; charset=utf-8' },
          body: {
            assessment: a.id,
            student: studentId,
            metaclass: a.metaclass
          }
        });
        const submissions = JSON.parse(submissionsRes).payload;
        analyticsData.push({
          ...a,
          details,
          submissions
        });
      } catch (e) {
        // Continue on error, but log
        console.error('Error fetching details for assessment', a.id, e);
      }
      progress = i + 1;
    }

    // Save to appdata using Tauri backend
    await invoke('save_analytics', { data: JSON.stringify(analyticsData) });
    saved = true;
  } catch (e) {
    error = 'Failed to fetch analytics data: ' + e;
  } finally {
    loading = false;
  }
}
</script>

<div class="max-w-2xl mx-auto p-8">
  <h1 class="text-2xl font-bold mb-6">Assessment Analytics Data Export</h1>
  <button class="px-6 py-3 rounded-lg bg-indigo-600 text-white font-semibold hover:bg-indigo-700 transition-colors disabled:opacity-60 disabled:cursor-not-allowed mb-6" onclick={fetchAllAssessments} disabled={loading}>
    {loading ? 'Fetching...' : 'Fetch and Save All Assessment Data'}
  </button>
  {#if loading}
    <div class="mb-4">Fetching assessment data... {progress} / {total}</div>
    <progress class="w-full" value={progress} max={total}></progress>
  {/if}
  {#if error}
    <div class="text-red-500 mb-4">{error}</div>
  {/if}
  {#if saved}
    <div class="text-green-600 mb-4">All assessment data saved to your app data folder for analytics!</div>
  {/if}
  {#if analyticsData.length}
    <div class="mt-6">
      <h2 class="font-semibold mb-2">Summary</h2>
      <ul class="list-disc ml-6">
        <li>Total assessments: {analyticsData.length}</li>
        <li>File: <code>analytics.json</code> in your app data folder</li>
      </ul>
    </div>
  {/if}
</div> 