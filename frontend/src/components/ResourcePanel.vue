<template>
  <div class="resource-panel">
    <!-- Empty State -->
    <div v-if="resources.length === 0" class="empty-state">
      <div class="empty-icon">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"/>
          <path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"/>
        </svg>
      </div>
      <h3>暂无资源</h3>
      <p>连接 MCP 服务器后，资源列表将显示在这里</p>
    </div>

    <!-- Resource List -->
    <div v-else class="resource-list">
      <article 
        v-for="resource in resources" 
        :key="resource.uri" 
        class="resource-card"
      >
        <div class="resource-header">
          <div class="resource-icon">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
              <polyline points="14 2 14 8 20 8"/>
            </svg>
          </div>
          <div class="resource-info">
            <h3>{{ resource.name }}</h3>
            <span v-if="resource.mime_type" class="mime-badge">{{ resource.mime_type }}</span>
          </div>
        </div>
        
        <div class="resource-uri">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/>
            <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/>
          </svg>
          <code>{{ resource.uri }}</code>
        </div>
        
        <p v-if="resource.description" class="resource-desc">
          {{ resource.description }}
        </p>
      </article>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Resource } from '@/types';
defineProps<{ resources: Resource[] }>();
</script>

<style scoped>
.resource-panel {
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
  background: rgba(6, 182, 212, 0.1);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 16px;
  color: var(--accent-color);
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

.resource-list {
  display: grid;
  gap: 12px;
}

.resource-card {
  background: var(--bg-card);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-lg);
  padding: 16px;
  transition: all var(--transition-normal);
}

.resource-card:hover {
  border-color: rgba(6, 182, 212, 0.3);
  box-shadow: 0 0 24px rgba(6, 182, 212, 0.1);
}

.resource-header {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  margin-bottom: 12px;
}

.resource-icon {
  width: 36px;
  height: 36px;
  background: linear-gradient(135deg, #0891b2, #06b6d4);
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  flex-shrink: 0;
}

.resource-info {
  flex: 1;
  min-width: 0;
  display: flex;
  align-items: center;
  gap: 10px;
  flex-wrap: wrap;
}

.resource-info h3 {
  font-size: 0.95rem;
  font-weight: 600;
  margin: 0;
  color: var(--text-primary);
}

.mime-badge {
  font-size: 0.7rem;
  padding: 3px 8px;
  background: rgba(6, 182, 212, 0.15);
  color: var(--accent-color);
  border-radius: var(--radius-full);
  font-family: 'JetBrains Mono', monospace;
}

.resource-uri {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  background: rgba(0, 0, 0, 0.25);
  border-radius: var(--radius-md);
  margin-bottom: 10px;
}

.resource-uri svg {
  color: var(--text-dim);
  flex-shrink: 0;
}

.resource-uri code {
  font-size: 0.8rem;
  color: var(--text-secondary);
  word-break: break-all;
  font-family: 'JetBrains Mono', monospace;
}

.resource-desc {
  font-size: 0.85rem;
  color: var(--text-muted);
  margin: 0;
  line-height: 1.5;
}
</style>
