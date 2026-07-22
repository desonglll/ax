<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { Activity, Clock3, Gauge, RefreshCw, Route } from "lucide-vue-next";
import { statsApi } from "../api";
import type { SystemStats } from "../types";

const stats = ref<SystemStats>({ requestCount: 0, responseTimes: {} });
const endpoints = computed(() => Object.entries(stats.value.responseTimes).map(([route, times]) => ({
  route,
  requests: times.length,
  average: times.length ? times.reduce((sum, value) => sum + value, 0) / times.length : 0,
  maximum: times.length ? Math.max(...times) : 0,
})).sort((a, b) => b.requests - a.requests));
const overallAverage = computed(() => {
  const values = Object.values(stats.value.responseTimes).flat();
  return values.length ? values.reduce((sum, value) => sum + value, 0) / values.length : 0;
});
const load = async () => { stats.value = await statsApi.get(); };
onMounted(load);
</script>

<template>
  <div class="ax-container space-y-6">
    <header class="flex items-end justify-between"><div><p class="ax-section-title">Administration</p><h1 class="ax-page-title">System telemetry</h1></div><button class="btn btn-outline btn-sm" @click="load"><RefreshCw :size="16" /> Refresh</button></header>
    <div class="stats stats-vertical w-full border border-base-300 bg-base-100 shadow-sm lg:stats-horizontal"><div class="stat"><div class="stat-figure text-primary"><Activity /></div><div class="stat-title">Requests</div><div class="stat-value text-primary">{{ stats.requestCount }}</div><div class="stat-desc">Since server startup</div></div><div class="stat"><div class="stat-figure text-secondary"><Route /></div><div class="stat-title">Tracked routes</div><div class="stat-value text-secondary">{{ endpoints.length }}</div><div class="stat-desc">With timing samples</div></div><div class="stat"><div class="stat-figure text-accent"><Clock3 /></div><div class="stat-title">Average latency</div><div class="stat-value text-accent">{{ overallAverage.toFixed(1) }}</div><div class="stat-desc">Milliseconds</div></div></div>
    <section class="ax-panel"><div class="card-body"><h2 class="card-title"><Gauge :size="20" /> Endpoint performance</h2><div class="overflow-x-auto"><table class="table"><thead><tr><th>Route</th><th>Samples</th><th>Average</th><th>Slowest</th></tr></thead><tbody><tr v-for="item in endpoints" :key="item.route"><td class="font-mono text-xs">{{ item.route }}</td><td>{{ item.requests }}</td><td>{{ item.average.toFixed(2) }} ms</td><td><span class="badge" :class="item.maximum > 1000 ? 'badge-error' : item.maximum > 300 ? 'badge-warning' : 'badge-success'">{{ item.maximum.toFixed(2) }} ms</span></td></tr></tbody></table><p v-if="!endpoints.length" class="py-14 text-center ax-muted">No timing data has been collected yet.</p></div></div></section>
  </div>
</template>
