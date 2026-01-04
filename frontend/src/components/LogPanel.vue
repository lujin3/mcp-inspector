<template>
  <div class="log-wrapper">
    <h2>Logs</h2>
    <div class="log-list">
      <p v-for="(log, index) in logs" :key="index">{{ log }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{ logs: string[] }>();
</script>

<style scoped>
.log-wrapper {
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.log-list {
  flex: 1;
  background: linear-gradient(180deg, rgba(22, 24, 44, 0.9), rgba(8, 10, 24, 0.85));
  border-radius: 16px;
  padding: 16px;
  overflow-y: auto;
  font-size: 0.9rem;
  color: rgba(255, 255, 255, 0.85);
  display: flex;
  flex-direction: column;
  gap: 6px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.05);
}

.log-list::-webkit-scrollbar {
  width: 6px;
}

.log-list::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.2);
  border-radius: 3px;
}
</style>
