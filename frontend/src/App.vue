<template>
  <div class="app-layout">
    <HeaderBar :status="connectionStatus" />
    
    <main class="main-content">
      <div class="workspace">
        <!-- Left Sidebar: Connection Panel -->
        <aside class="sidebar">
          <section class="panel connection-panel">
            <div class="panel-header">
              <div class="panel-title">
                <div class="panel-icon connection-icon">
                  <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M12 2v4m0 12v4M4.93 4.93l2.83 2.83m8.48 8.48l2.83 2.83M2 12h4m12 0h4M4.93 19.07l2.83-2.83m8.48-8.48l2.83-2.83"/>
                  </svg>
                </div>
                <div>
                  <h2>连接配置</h2>
                  <p class="panel-subtitle">配置 MCP 服务器连接</p>
                </div>
              </div>
            </div>

            <div class="connection-form">
              <!-- Server URL -->
              <div class="form-group">
                <label class="form-label">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/>
                    <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/>
                  </svg>
                  服务器地址
                </label>
                <input 
                  class="form-input" 
                  v-model="serverUrl" 
                  placeholder="http://localhost:8000/mcp"
                  :disabled="connectionStatus === 'connected'"
                />
              </div>

              <!-- Headers -->
              <div class="form-group">
                <label class="form-label">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/>
                    <rect x="8" y="2" width="8" height="4" rx="1" ry="1"/>
                  </svg>
                  请求头 (可选)
                </label>
                <div class="headers-list">
                  <div class="header-row" v-for="(header, index) in headerInputs" :key="index">
                    <input
                      v-model="header.name"
                      class="form-input header-name"
                      placeholder="Header 名称"
                      :disabled="connectionStatus === 'connected'"
                    />
                    <input
                      v-model="header.value"
                      class="form-input header-value"
                      placeholder="Header 值"
                      :disabled="connectionStatus === 'connected'"
                    />
                    <button 
                      class="btn-icon btn-remove" 
                      @click="removeHeaderRow(index)"
                      :disabled="connectionStatus === 'connected'"
                      title="移除"
                    >
                      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <line x1="18" y1="6" x2="6" y2="18"/>
                        <line x1="6" y1="6" x2="18" y2="18"/>
                      </svg>
                    </button>
                  </div>
                </div>
                <button 
                  class="btn-text" 
                  @click="addHeaderRow"
                  v-if="connectionStatus !== 'connected'"
                >
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <line x1="12" y1="5" x2="12" y2="19"/>
                    <line x1="5" y1="12" x2="19" y2="12"/>
                  </svg>
                  添加请求头
                </button>
              </div>

              <!-- Action Buttons -->
              <div class="action-buttons">
                <button 
                  v-if="connectionStatus !== 'connected'"
                  class="btn btn-primary" 
                  :disabled="isConnecting" 
                  @click="connectToServer"
                >
                  <svg v-if="!isConnecting" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/>
                  </svg>
                  <span v-else class="loading-spinner"></span>
                  {{ isConnecting ? '连接中...' : '连接服务器' }}
                </button>
                <button 
                  v-else
                  class="btn btn-danger" 
                  @click="disconnect"
                >
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <line x1="18" y1="6" x2="6" y2="18"/>
                    <line x1="6" y1="6" x2="18" y2="18"/>
                  </svg>
                  断开连接
                </button>
                <button class="btn btn-secondary" @click="restartConnection" title="重启连接">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.2"/>
                  </svg>
                </button>
              </div>

              <!-- Connection Info -->
              <div v-if="lastConnectSummary" class="connection-info">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="12" cy="12" r="10"/>
                  <path d="M12 16v-4"/>
                  <path d="M12 8h.01"/>
                </svg>
                <span>{{ lastConnectSummary }}</span>
              </div>
            </div>
          </section>
        </aside>

        <!-- Main Area -->
        <div class="main-area">
          <!-- Inspector Panel -->
          <section class="panel inspector-panel">
            <div class="panel-header inspector-header">
              <div class="panel-title">
                <div class="panel-icon inspector-icon">
                  <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/>
                    <path d="M12 22V12"/>
                    <path d="M3.3 7L12 12l8.7-5"/>
                  </svg>
                </div>
                <div>
                  <h2>Inspector</h2>
                  <p class="panel-subtitle">探索工具、资源和提示</p>
                </div>
              </div>
              
              <!-- Tab Switcher -->
              <div class="tab-switcher">
                <button
                  v-for="tab in tabOptions"
                  :key="tab.value"
                  class="tab-button"
                  :class="{ active: activeTab === tab.value }"
                  @click="activeTab = tab.value"
                >
                  <svg v-if="tab.value === 'tools'" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/>
                  </svg>
                  <svg v-else-if="tab.value === 'resources'" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z"/>
                    <path d="M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z"/>
                  </svg>
                  <svg v-else width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
                  </svg>
                  {{ tab.label }}
                  <span v-if="getTabCount(tab.value)" class="tab-badge">
                    {{ getTabCount(tab.value) }}
                  </span>
                </button>
              </div>
            </div>

            <!-- Tab Content -->
            <div class="tab-content">
              <!-- Tools Tab -->
              <div v-if="activeTab === 'tools'" class="tab-panel animate-fadeIn">
                <ToolList 
                  :tools="tools" 
                  :callingTools="callingTools"
                  :results="toolResults"
                  @call="handleCallTool" 
                />
              </div>

              <!-- Resources Tab -->
              <div v-else-if="activeTab === 'resources'" class="tab-panel animate-fadeIn">
                <ResourcePanel :resources="resources" />
              </div>

              <!-- Prompts Tab -->
              <div v-else-if="activeTab === 'prompts'" class="tab-panel animate-fadeIn">
                <PromptPanel :prompts="prompts" />
              </div>
            </div>
          </section>

          <!-- History Panel -->
          <section class="panel history-panel">
            <div class="panel-header history-header">
              <div class="panel-title compact">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="12" cy="12" r="10"/>
                  <polyline points="12 6 12 12 16 14"/>
                </svg>
                <h3>活动日志</h3>
              </div>
              <div class="history-actions">
                <span class="count-badge small">{{ logs.length }}</span>
                <button class="btn-icon-small" @click="clearLogs" title="清空日志">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline points="3 6 5 6 21 6"/>
                    <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
                  </svg>
                </button>
              </div>
            </div>
            <div class="history-content">
              <div v-if="logs.length === 0" class="empty-logs">
                <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <rect x="3" y="4" width="18" height="16" rx="2"/>
                  <line x1="7" y1="9" x2="17" y2="9"/>
                  <line x1="7" y1="13" x2="13" y2="13"/>
                </svg>
                <p>暂无活动记录</p>
              </div>
              <div v-else class="log-list">
                <div
                  v-for="(entry, idx) in visibleHistory"
                  :key="'log-' + idx"
                  class="log-item"
                >
                  <details class="log-details">
                    <summary>
                      <span class="log-index">{{ idx + 1 }}</span>
                      <span class="log-text">{{ summarizeLog(entry) }}</span>
                    </summary>
                    <pre class="log-full">{{ entry }}</pre>
                  </details>
                </div>
              </div>
              <div v-if="logs.length > historyLimit" class="history-footer">
                <button class="btn-text small" @click="historyExpanded = !historyExpanded">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <polyline :points="historyExpanded ? '18 15 12 9 6 15' : '6 9 12 15 18 9'"/>
                  </svg>
                  {{ historyExpanded ? '收起' : `显示全部 ${logs.length} 条` }}
                </button>
              </div>
            </div>
          </section>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import HeaderBar from './components/HeaderBar.vue';
import ToolList from './components/ToolList.vue';
import ResourcePanel from './components/ResourcePanel.vue';
import PromptPanel from './components/PromptPanel.vue';
import type { Tool, Resource, Prompt, HeaderPair, ConnectResult, ToolCallPayload } from '@/types';
import { mcpApi, isRunningInTauri } from '@/services/mcpApi';

const serverUrl = ref('http://localhost:8000/mcp');
const headerInputs = ref<HeaderPair[]>([{ name: '', value: '' }]);
const connectionStatus = ref<'disconnected' | 'connecting' | 'connected' | 'error'>('disconnected');
const isConnecting = ref(false);
const lastConnectSummary = ref('');
const tools = ref<Tool[]>([]);
const resources = ref<Resource[]>([]);
const prompts = ref<Prompt[]>([]);
const logs = ref<string[]>([]);
const historyLimit = 5;
const HISTORY_SUMMARY_LENGTH = 100;
const historyExpanded = ref(false);
const runningInTauri = ref(isRunningInTauri());

type InspectorTab = 'tools' | 'resources' | 'prompts';

interface ToolResultEntry {
  id: string;
  toolName: string;
  status: 'success' | 'error';
  message: string;
  payload?: string;
  timestamp: string;
}

const tabOptions = [
  { value: 'tools', label: 'Tools' },
  { value: 'resources', label: 'Resources' },
  { value: 'prompts', label: 'Prompts' },
] as const;

const activeTab = ref<InspectorTab>('tools');
const toolResults = ref<ToolResultEntry[]>([]);
const MAX_TOOL_RESULTS = 1;
const MAX_LOGS = 200;
const callingTools = ref(new Set<string>());

const visibleHistory = computed(() => {
  if (historyExpanded.value) return logs.value;
  return logs.value.slice(0, historyLimit);
});

function getTabCount(tab: InspectorTab) {
  if (tab === 'tools') return tools.value.length || null;
  if (tab === 'resources') return resources.value.length || null;
  if (tab === 'prompts') return prompts.value.length || null;
  return null;
}

function pushLog(message: string) {
  logs.value.unshift(message);
  if (logs.value.length > MAX_LOGS) logs.value.pop();
}

function formatError(err: unknown) {
  let message: string;
  if (err instanceof Error) {
    message = err.message;
  } else if (typeof err === 'string') {
    message = err;
  } else {
    try {
      message = JSON.stringify(err);
    } catch {
      message = '未知错误';
    }
  }
  if (!runningInTauri.value && message === 'Load failed') {
    return `${message}（可能因 CORS 限制导致）`;
  }
  return message;
}

function summarizeLog(entry: string) {
  const firstLine = entry.split('\n')[0] || entry;
  if (firstLine.length <= HISTORY_SUMMARY_LENGTH) {
    return firstLine;
  }
  return `${firstLine.slice(0, HISTORY_SUMMARY_LENGTH)}…`;
}

function addHeaderRow() {
  headerInputs.value.push({ name: '', value: '' });
}

function removeHeaderRow(index: number) {
  if (headerInputs.value.length === 1) {
    headerInputs.value[0] = { name: '', value: '' };
    return;
  }
  headerInputs.value.splice(index, 1);
}

function clearLogs() {
  logs.value = [];
}

function pushToolResult(entry: ToolResultEntry) {
  toolResults.value.unshift(entry);
  if (toolResults.value.length > MAX_TOOL_RESULTS) {
    toolResults.value.pop();
  }
}

async function refreshCatalog() {
  try {
    tools.value = await mcpApi.listTools();
  } catch (err) {
    pushLog(`[${new Date().toLocaleTimeString()}] list_tools 错误: ${formatError(err)}`);
  }
  try {
    resources.value = await mcpApi.listResources();
  } catch (err) {
    pushLog(`[${new Date().toLocaleTimeString()}] list_resources 错误: ${formatError(err)}`);
  }
  try {
    prompts.value = await mcpApi.listPrompts();
  } catch (err) {
    pushLog(`[${new Date().toLocaleTimeString()}] list_prompts 错误: ${formatError(err)}`);
  }
}

async function connectToServer() {
  if (isConnecting.value) return;
  isConnecting.value = true;
  connectionStatus.value = 'connecting';

  const preparedHeaders = headerInputs.value
    .map((header) => ({ name: header.name.trim(), value: header.value.trim() }))
    .filter((header) => header.name && header.value);

  try {
    const connectTask = (async () => {
      pushLog(`[${new Date().toLocaleTimeString()}] 正在连接: ${serverUrl.value}`);
      return await mcpApi.connect(serverUrl.value, preparedHeaders);
    })();

    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => reject(new Error('连接超时 (15s)')), 15000);
    });

    const result = await Promise.race([connectTask, timeoutPromise]) as ConnectResult;
    
    connectionStatus.value = result.connected ? 'connected' : 'disconnected';
    lastConnectSummary.value = `${result.url}`;
    pushLog(`[${new Date().toLocaleTimeString()}] 连接成功`);
    if (result.connected) {
      await refreshCatalog();
    }
  } catch (err) {
    connectionStatus.value = 'error';
    tools.value = [];
    resources.value = [];
    prompts.value = [];
    pushLog(`[${new Date().toLocaleTimeString()}] 连接失败: ${formatError(err)}`);
  } finally {
    isConnecting.value = false;
  }
}

async function disconnect() {
  connectionStatus.value = 'disconnected';
  tools.value = [];
  resources.value = [];
  prompts.value = [];
  toolResults.value = [];
  logs.value = [];
  lastConnectSummary.value = '';
}

async function restartConnection() {
  await disconnect();
  await connectToServer();
}

function createToolResultId() {
  return `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
}

function buildPayloadString(value: unknown) {
  if (value === undefined) return '';
  try {
    return JSON.stringify(value, null, 2);
  } catch {
    return String(value);
  }
}

async function handleCallTool(payload: ToolCallPayload) {
  const timestamp = new Date().toLocaleTimeString();
  callingTools.value.add(payload.name);
  try {
    const result = await mcpApi.callTool(payload.name, payload.args);
    pushLog(`[${timestamp}] Tool "${payload.name}" 成功`);
    pushToolResult({
      id: createToolResultId(),
      toolName: payload.name,
      status: 'success',
      message: '调用成功',
      payload: buildPayloadString(result),
      timestamp,
    });
  } catch (err) {
    const message = formatError(err);
    pushLog(`[${timestamp}] Tool "${payload.name}" 失败: ${message}`);
    pushToolResult({
      id: createToolResultId(),
      toolName: payload.name,
      status: 'error',
      message,
      timestamp,
    });
  } finally {
    callingTools.value.delete(payload.name);
  }
}
</script>

<style scoped>
.app-layout {
  min-height: 100vh;
  background: 
    radial-gradient(ellipse 80% 50% at 20% -10%, rgba(139, 92, 246, 0.15), transparent),
    radial-gradient(ellipse 60% 40% at 80% 10%, rgba(6, 182, 212, 0.12), transparent),
    var(--bg-primary);
  display: flex;
  flex-direction: column;
}

.main-content {
  flex: 1;
  max-width: 1440px;
  width: 100%;
  margin: 0 auto;
  padding: 24px;
}

.workspace {
  display: grid;
  grid-template-columns: 340px 1fr;
  gap: 24px;
  min-height: calc(100vh - 140px);
}

.sidebar {
  display: flex;
  flex-direction: column;
}

.panel {
  background: rgba(17, 24, 39, 0.85);
  backdrop-filter: blur(20px);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-xl);
  overflow: hidden;
}

.connection-panel {
  position: sticky;
  top: 100px;
}

.panel-header {
  padding: 20px;
  border-bottom: 1px solid var(--border-subtle);
}

.panel-title {
  display: flex;
  align-items: flex-start;
  gap: 12px;
}

.panel-title.compact {
  align-items: center;
  gap: 8px;
}

.panel-title.compact h3 {
  margin: 0;
  font-size: 0.95rem;
}

.panel-icon {
  width: 42px;
  height: 42px;
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  flex-shrink: 0;
}

.connection-icon {
  background: linear-gradient(135deg, #8b5cf6, #6366f1);
}

.inspector-icon {
  background: var(--primary-gradient);
  width: 46px;
  height: 46px;
}

.panel-title h2 {
  margin: 0;
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-primary);
}

.panel-subtitle {
  margin: 2px 0 0;
  font-size: 0.82rem;
  color: var(--text-muted);
}

.connection-form {
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 18px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-label {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 0.82rem;
  font-weight: 500;
  color: var(--text-secondary);
}

.form-input {
  width: 100%;
  padding: 11px 14px;
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border-light);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-size: 0.9rem;
  transition: all var(--transition-fast);
}

.form-input:focus {
  outline: none;
  border-color: var(--border-focus);
  box-shadow: 0 0 0 3px rgba(139, 92, 246, 0.15);
}

.form-input:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.form-input::placeholder {
  color: var(--text-dim);
}

.headers-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.header-row {
  display: grid;
  grid-template-columns: 1fr 1fr 32px;
  gap: 8px;
  align-items: center;
}

.btn-icon {
  width: 32px;
  height: 32px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-sm);
  color: var(--text-muted);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-fast);
}

.btn-icon:hover:not(:disabled) {
  background: var(--error-bg);
  border-color: rgba(239, 68, 68, 0.3);
  color: var(--error);
}

.btn-icon:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.btn-text {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 0;
  background: none;
  border: none;
  color: var(--primary-color);
  font-size: 0.82rem;
  font-weight: 500;
  cursor: pointer;
  transition: color var(--transition-fast);
}

.btn-text:hover {
  color: var(--primary-hover);
}

.btn-text.small {
  font-size: 0.8rem;
}

.action-buttons {
  display: flex;
  gap: 10px;
  margin-top: 4px;
}

.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 12px 18px;
  border: none;
  border-radius: var(--radius-md);
  font-size: 0.9rem;
  font-weight: 600;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.btn-primary {
  flex: 1;
  background: var(--primary-gradient);
  color: white;
  box-shadow: 0 4px 12px rgba(139, 92, 246, 0.25);
}

.btn-primary:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 6px 16px rgba(139, 92, 246, 0.35);
}

.btn-primary:disabled {
  opacity: 0.7;
  cursor: not-allowed;
  transform: none;
}

.btn-danger {
  flex: 1;
  background: var(--error-bg);
  color: var(--error);
  border: 1px solid rgba(239, 68, 68, 0.3);
}

.btn-danger:hover {
  background: rgba(239, 68, 68, 0.25);
}

.btn-secondary {
  background: rgba(255, 255, 255, 0.06);
  color: var(--text-secondary);
  border: 1px solid var(--border-subtle);
  padding: 12px;
}

.btn-secondary:hover {
  background: rgba(255, 255, 255, 0.1);
  border-color: var(--border-light);
}

.loading-spinner {
  width: 16px;
  height: 16px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-top-color: white;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.connection-info {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  background: var(--info-bg);
  border: 1px solid rgba(59, 130, 246, 0.2);
  border-radius: var(--radius-md);
  font-size: 0.82rem;
  color: var(--info);
}

.connection-info svg {
  flex-shrink: 0;
}

.main-area {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.inspector-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.inspector-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  flex-wrap: wrap;
  gap: 16px;
}

.tab-switcher {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}

.tab-button {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  background: transparent;
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-full);
  color: var(--text-muted);
  font-size: 0.85rem;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.tab-button:hover {
  border-color: var(--border-light);
  color: var(--text-secondary);
}

.tab-button.active {
  background: rgba(139, 92, 246, 0.15);
  border-color: rgba(139, 92, 246, 0.4);
  color: var(--primary-color);
}

.tab-badge {
  padding: 2px 7px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: var(--radius-full);
  font-size: 0.72rem;
  font-weight: 600;
}

.tab-button.active .tab-badge {
  background: rgba(139, 92, 246, 0.3);
}

.tab-content {
  flex: 1;
  padding: 20px;
}

.tab-panel {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.results-section {
  background: rgba(0, 0, 0, 0.2);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-lg);
  padding: 16px;
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 12px;
}

.section-header h3 {
  display: flex;
  align-items: center;
  gap: 8px;
  margin: 0;
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--text-secondary);
}

.count-badge {
  padding: 3px 10px;
  background: rgba(139, 92, 246, 0.15);
  color: var(--primary-color);
  border-radius: var(--radius-full);
  font-size: 0.75rem;
  font-weight: 600;
}

.count-badge.small {
  padding: 2px 8px;
  font-size: 0.7rem;
}

.results-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 12px;
}

.result-card {
  background: var(--bg-card);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  padding: 14px;
  transition: all var(--transition-fast);
}

.result-card.success {
  border-left: 3px solid var(--success);
}

.result-card.error {
  border-left: 3px solid var(--error);
}

.result-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 8px;
}

.result-title {
  display: flex;
  align-items: center;
  gap: 8px;
}

.result-status-icon {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.result-status-icon.success {
  background: var(--success-bg);
  color: var(--success);
}

.result-status-icon.error {
  background: var(--error-bg);
  color: var(--error);
}

.result-title strong {
  font-size: 0.9rem;
  color: var(--text-primary);
}

.result-header time {
  font-size: 0.75rem;
  color: var(--text-dim);
  font-family: 'JetBrains Mono', monospace;
}

.result-message {
  margin: 0 0 8px;
  font-size: 0.85rem;
}

.result-message.success {
  color: var(--success);
}

.result-message.error {
  color: var(--error);
}

.result-payload {
  margin: 0;
  padding: 10px;
  background: rgba(0, 0, 0, 0.3);
  border-radius: var(--radius-sm);
  font-size: 0.78rem;
  color: var(--text-secondary);
  overflow: auto;
  max-height: 150px;
  font-family: 'JetBrains Mono', monospace;
}

.history-panel {
  max-height: 300px;
  display: flex;
  flex-direction: column;
}

.history-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 14px 16px;
}

.history-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.btn-icon-small {
  width: 28px;
  height: 28px;
  background: rgba(255, 255, 255, 0.05);
  border: none;
  border-radius: var(--radius-sm);
  color: var(--text-muted);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-fast);
}

.btn-icon-small:hover {
  background: var(--error-bg);
  color: var(--error);
}

.history-content {
  flex: 1;
  overflow-y: auto;
  padding: 0 16px 16px;
}

.empty-logs {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 24px;
  color: var(--text-dim);
  text-align: center;
}

.empty-logs svg {
  margin-bottom: 8px;
  opacity: 0.5;
}

.empty-logs p {
  margin: 0;
  font-size: 0.85rem;
}

.log-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.log-item {
  background: rgba(0, 0, 0, 0.2);
  border-radius: var(--radius-sm);
  overflow: hidden;
}

.log-details summary {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  cursor: pointer;
  transition: background var(--transition-fast);
}

.log-details summary:hover {
  background: rgba(255, 255, 255, 0.03);
}

.log-index {
  width: 22px;
  height: 22px;
  background: rgba(139, 92, 246, 0.15);
  color: var(--primary-color);
  border-radius: var(--radius-sm);
  font-size: 0.7rem;
  font-weight: 600;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.log-text {
  font-size: 0.82rem;
  color: var(--text-secondary);
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.log-full {
  margin: 0;
  padding: 12px;
  background: rgba(0, 0, 0, 0.3);
  font-size: 0.78rem;
  color: var(--text-muted);
  white-space: pre-wrap;
  word-break: break-all;
  font-family: 'JetBrains Mono', monospace;
}

.history-footer {
  padding-top: 8px;
  text-align: center;
}

.animate-fadeIn {
  animation: fadeIn var(--transition-normal) ease-out;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@media (max-width: 1024px) {
  .workspace {
    grid-template-columns: 1fr;
  }
  
  .connection-panel {
    position: static;
  }
}

@media (max-width: 640px) {
  .main-content {
    padding: 16px;
  }
  
  .action-buttons {
    flex-direction: column;
  }
  
  .btn-primary,
  .btn-danger {
    flex: auto;
  }
  
  .inspector-header {
    flex-direction: column;
    gap: 12px;
  }
  
  .tab-switcher {
    width: 100%;
  }
  
  .tab-button {
    flex: 1;
    justify-content: center;
  }
}
</style>
