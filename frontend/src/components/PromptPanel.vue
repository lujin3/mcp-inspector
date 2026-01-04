<template>
  <div class="prompt-panel">
    <!-- Empty State -->
    <div v-if="prompts.length === 0" class="empty-state">
      <div class="empty-icon">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
        </svg>
      </div>
      <h3>暂无 Prompts</h3>
      <p>连接 MCP 服务器后，Prompts 列表将显示在这里</p>
    </div>

    <!-- Prompt List -->
    <div v-else class="prompt-list">
      <article 
        v-for="prompt in prompts" 
        :key="prompt.name" 
        class="prompt-card"
      >
        <div class="prompt-header">
          <div class="prompt-icon">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
            </svg>
          </div>
          <h3>{{ prompt.name }}</h3>
        </div>
        
        <p v-if="prompt.description" class="prompt-desc">
          {{ prompt.description }}
        </p>

        <div v-if="prompt.arguments?.length" class="prompt-args">
          <span class="args-label">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
              <line x1="3" y1="9" x2="21" y2="9"/>
              <line x1="9" y1="21" x2="9" y2="9"/>
            </svg>
            参数
          </span>
          <div class="args-list">
            <span 
              v-for="arg in prompt.arguments" 
              :key="arg.name" 
              class="arg-badge"
              :class="{ required: arg.required }"
            >
              {{ arg.name }}
              <span v-if="arg.required" class="required-dot"></span>
            </span>
          </div>
        </div>
      </article>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Prompt } from '@/types';
defineProps<{ prompts: Prompt[] }>();
</script>

<style scoped>
.prompt-panel {
  display: flex;
  flex-direction: column;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 48px 24px;
  text-align: center;
  color: var(--text-muted);
}

.empty-icon {
  width: 80px;
  height: 80px;
  background: var(--success-bg);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 16px;
  color: var(--success);
}

.empty-state h3 {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-secondary);
  margin-bottom: 8px;
}

.empty-state p {
  font-size: 0.9rem;
  color: var(--text-muted);
}

.prompt-list {
  display: grid;
  gap: 12px;
}

.prompt-card {
  background: var(--bg-card);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-lg);
  padding: 16px;
  transition: all var(--transition-normal);
}

.prompt-card:hover {
  border-color: rgba(16, 185, 129, 0.3);
  box-shadow: 0 0 24px rgba(16, 185, 129, 0.1);
}

.prompt-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 10px;
}

.prompt-icon {
  width: 36px;
  height: 36px;
  background: linear-gradient(135deg, #059669, #10b981);
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  flex-shrink: 0;
}

.prompt-header h3 {
  font-size: 0.95rem;
  font-weight: 600;
  margin: 0;
  color: var(--text-primary);
}

.prompt-desc {
  font-size: 0.85rem;
  color: var(--text-muted);
  margin: 0 0 12px;
  line-height: 1.5;
}

.prompt-args {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.args-label {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 0.75rem;
  font-weight: 500;
  color: var(--text-dim);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.args-list {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.arg-badge {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 0.75rem;
  padding: 4px 10px;
  background: var(--badge-bg);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-full);
  color: var(--text-secondary);
  font-family: 'JetBrains Mono', monospace;
}

.arg-badge.required {
  background: var(--success-bg);
  border-color: rgba(16, 185, 129, 0.3);
  color: var(--success);
}

.required-dot {
  width: 4px;
  height: 4px;
  background: var(--success);
  border-radius: 50%;
}
</style>
