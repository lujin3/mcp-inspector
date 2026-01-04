<template>
  <header class="header">
    <div class="header-inner">
      <!-- Logo & Brand -->
      <div class="brand">
        <div class="logo">
          <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/>
            <path d="M12 22V12"/>
            <path d="M3.3 7L12 12l8.7-5"/>
          </svg>
        </div>
        <div class="brand-text">
          <h1>MCP Inspector</h1>
          <span class="version">v1.0.0</span>
        </div>
      </div>

      <!-- Status & Time -->
      <div class="header-right">
        <div class="status-badge" :class="status">
          <span class="status-dot"></span>
          <span class="status-text">{{ statusText }}</span>
        </div>
        <div class="time-display">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10"/>
            <polyline points="12 6 12 12 16 14"/>
          </svg>
          <span>{{ currentTime }}</span>
        </div>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';

const props = defineProps<{ status: string }>();

const currentTime = ref(new Date().toLocaleTimeString('zh-CN', { 
  hour: '2-digit', 
  minute: '2-digit',
  second: '2-digit'
}));

let timeInterval: number | null = null;

onMounted(() => {
  timeInterval = setInterval(() => {
    currentTime.value = new Date().toLocaleTimeString('zh-CN', { 
      hour: '2-digit', 
      minute: '2-digit',
      second: '2-digit'
    });
  }, 1000);
});

onUnmounted(() => {
  if (timeInterval) {
    clearInterval(timeInterval);
  }
});

const statusText = computed(() => {
  const statusMap: Record<string, string> = {
    disconnected: '未连接',
    connecting: '连接中',
    connected: '已连接',
    error: '连接错误'
  };
  return statusMap[props.status] || '未知状态';
});
</script>

<style scoped>
.header {
  background: linear-gradient(180deg, rgba(17, 24, 39, 0.98) 0%, rgba(17, 24, 39, 0.95) 100%);
  border-bottom: 1px solid var(--border-subtle);
  backdrop-filter: blur(20px);
  position: sticky;
  top: 0;
  z-index: 100;
}

.header-inner {
  max-width: 1440px;
  margin: 0 auto;
  padding: 16px 24px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.brand {
  display: flex;
  align-items: center;
  gap: 14px;
}

.logo {
  width: 48px;
  height: 48px;
  background: var(--primary-gradient);
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  box-shadow: 0 4px 16px rgba(139, 92, 246, 0.3);
  transition: transform var(--transition-normal), box-shadow var(--transition-normal);
}

.logo:hover {
  transform: scale(1.05);
  box-shadow: 0 6px 24px rgba(139, 92, 246, 0.4);
}

.brand-text h1 {
  font-size: 1.35rem;
  font-weight: 700;
  margin: 0;
  background: var(--primary-gradient);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  letter-spacing: -0.02em;
}

.version {
  font-size: 0.75rem;
  color: var(--text-muted);
  font-weight: 500;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.status-badge {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 14px;
  border-radius: var(--radius-full);
  font-size: 0.85rem;
  font-weight: 600;
  transition: all var(--transition-normal);
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  transition: background var(--transition-normal);
}

.status-badge.disconnected {
  background: rgba(107, 114, 128, 0.15);
  color: var(--text-muted);
  border: 1px solid rgba(107, 114, 128, 0.2);
}

.status-badge.disconnected .status-dot {
  background: var(--text-muted);
}

.status-badge.connecting {
  background: var(--warning-bg);
  color: var(--warning);
  border: 1px solid rgba(245, 158, 11, 0.3);
}

.status-badge.connecting .status-dot {
  background: var(--warning);
  animation: pulse 1.5s ease-in-out infinite;
}

.status-badge.connected {
  background: var(--success-bg);
  color: var(--success);
  border: 1px solid rgba(16, 185, 129, 0.3);
}

.status-badge.connected .status-dot {
  background: var(--success);
  box-shadow: 0 0 8px var(--success);
}

.status-badge.error {
  background: var(--error-bg);
  color: var(--error);
  border: 1px solid rgba(239, 68, 68, 0.3);
}

.status-badge.error .status-dot {
  background: var(--error);
}

.time-display {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: var(--radius-md);
  font-size: 0.85rem;
  color: var(--text-secondary);
  font-family: 'JetBrains Mono', monospace;
}

.time-display svg {
  opacity: 0.7;
}

@keyframes pulse {
  0%, 100% { opacity: 1; transform: scale(1); }
  50% { opacity: 0.5; transform: scale(0.9); }
}
</style>
