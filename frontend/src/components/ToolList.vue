<template>
  <div class="tool-list">
    <!-- Empty State -->
    <div v-if="tools.length === 0" class="empty-state">
      <div class="empty-icon">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/>
        </svg>
      </div>
      <h3>暂无可用工具</h3>
      <p>连接 MCP 服务器后，工具列表将显示在这里</p>
    </div>

    <!-- Tool Cards -->
    <article 
      v-for="tool in tools" 
      :key="tool.name" 
      class="tool-card"
      :class="{ 'expanded': expandedTool === tool.name }"
    >
      <!-- Tool Header -->
      <div class="tool-header" @click="toggleTool(tool.name)">
        <div class="tool-info">
          <div class="tool-icon">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"/>
            </svg>
          </div>
          <div class="tool-meta">
            <h3>{{ tool.title || tool.name }}</h3>
            <p v-if="tool.description && expandedTool !== tool.name" class="tool-desc-preview">{{ tool.description }}</p>
          </div>
        </div>
        <div class="tool-actions">
          <span class="fields-count" v-if="fields(tool).length">
            {{ fields(tool).length }} 参数
          </span>
          <button class="expand-btn" :class="{ 'rotated': expandedTool === tool.name }">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="6 9 12 15 18 9"/>
            </svg>
          </button>
        </div>
      </div>

      <!-- Tool Body (Expandable) -->
      <div class="tool-body" v-show="expandedTool === tool.name">
        <!-- Markdown Description -->
        <div v-if="tool.description" class="markdown-description" v-html="renderMarkdown(tool.description)"></div>
        <!-- Raw JSON Input -->
        <div v-if="fields(tool).length === 0" class="json-input-section">
          <label class="input-label">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/>
              <rect x="8" y="2" width="8" height="4" rx="1" ry="1"/>
            </svg>
            参数 (JSON 格式)
          </label>
          <div class="textarea-wrapper">
            <textarea
              v-model="jsonArgs[tool.name]"
              placeholder='{"key": "value"}'
              rows="4"
              class="json-textarea"
            ></textarea>
          </div>
          <p class="json-error" v-if="jsonErrors[tool.name]">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="10"/>
              <line x1="12" y1="8" x2="12" y2="12"/>
              <line x1="12" y1="16" x2="12.01" y2="16"/>
            </svg>
            {{ jsonErrors[tool.name] }}
          </p>
        </div>

        <!-- Schema Fields -->
        <div v-else class="schema-fields">
          <label class="input-label">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
              <line x1="3" y1="9" x2="21" y2="9"/>
              <line x1="9" y1="21" x2="9" y2="9"/>
            </svg>
            参数配置
          </label>
          <div class="fields-grid">
            <div class="schema-field" v-for="field in fields(tool)" :key="field">
              <div class="field-header">
                <div class="field-info">
                  <span class="field-name">{{ field }}</span>
                  <span v-if="isRequired(tool, field)" class="required-badge">必填</span>
                </div>
                <span v-if="fieldType(tool, field)" class="field-type">{{ fieldType(tool, field) }}</span>
              </div>
              <p v-if="fieldDescription(tool, field)" class="field-desc">{{ fieldDescription(tool, field) }}</p>
              
              <!-- String Input -->
              <input
                v-if="getInputType(tool, field) === 'text'"
                type="text"
                v-model="schemaEntry(tool)[field]"
                :placeholder="getPlaceholder(tool, field)"
                class="field-input"
              />
              
              <!-- Number Input -->
              <input
                v-else-if="getInputType(tool, field) === 'number'"
                type="number"
                v-model="schemaEntry(tool)[field]"
                :placeholder="getPlaceholder(tool, field)"
                :min="getFieldMin(tool, field)"
                :max="getFieldMax(tool, field)"
                :step="getFieldStep(tool, field)"
                class="field-input"
              />
              
              <!-- Boolean Toggle -->
              <div v-else-if="getInputType(tool, field) === 'boolean'" class="toggle-wrapper">
                <label class="toggle">
                  <input 
                    type="checkbox" 
                    v-model="booleanEntry(tool)[field]"
                  />
                  <span class="toggle-slider"></span>
                </label>
                <span class="toggle-label">{{ booleanEntry(tool)[field] ? '是' : '否' }}</span>
              </div>
              
              <!-- Enum Select -->
              <select
                v-else-if="getInputType(tool, field) === 'select'"
                v-model="schemaEntry(tool)[field]"
                class="field-select"
              >
                <option value="" disabled>请选择...</option>
                <option 
                  v-for="option in getEnumOptions(tool, field)" 
                  :key="option" 
                  :value="option"
                >
                  {{ option }}
                </option>
              </select>
              
              <!-- Array/Object Textarea -->
              <textarea
                v-else-if="getInputType(tool, field) === 'json'"
                v-model="schemaEntry(tool)[field]"
                :placeholder="getPlaceholder(tool, field)"
                rows="3"
                class="json-textarea small"
              ></textarea>
            </div>
          </div>
        </div>

        <!-- Run Button -->
        <button class="run-button" type="button" @click="handleCall(tool)">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polygon points="5 3 19 12 5 21 5 3"/>
          </svg>
          执行工具
        </button>
      </div>
    </article>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue';
import { marked } from 'marked';
import type { Tool, ToolCallPayload } from '@/types';

const props = defineProps<{ tools: Tool[] }>();

const emits = defineEmits<{
  (event: 'call', payload: ToolCallPayload): void;
}>();

const jsonArgs = reactive<Record<string, string>>({});
const schemaArgs = reactive<Record<string, Record<string, string>>>({});
const booleanArgs = reactive<Record<string, Record<string, boolean>>>({});
const jsonErrors = reactive<Record<string, string | null>>({});
const expandedTool = ref<string | null>(null);

function renderMarkdown(text: string) {
  try {
    return marked.parse(text);
  } catch (e) {
    return text;
  }
}

function toggleTool(toolName: string) {
  expandedTool.value = expandedTool.value === toolName ? null : toolName;
}

function fields(tool: Tool): string[] {
  const schema = tool.input_schema as Record<string, any> | undefined;
  if (!schema) return [];
  const properties = schema.properties as Record<string, any> | undefined;
  if (!properties) return [];
  return Object.keys(properties);
}

function propertySchema(tool: Tool, field: string): Record<string, any> | undefined {
  const schema = tool.input_schema as Record<string, any> | undefined;
  const properties = schema?.properties as Record<string, any> | undefined;
  return properties?.[field];
}

function normalizeType(type: unknown): string | undefined {
  if (!type) return undefined;
  if (Array.isArray(type)) {
    return type.find((item) => item !== 'null') ?? (type[0] as string | undefined);
  }
  return typeof type === 'string' ? type : undefined;
}

function fieldType(tool: Tool, field: string): string | undefined {
  const schema = propertySchema(tool, field);
  return normalizeType(schema?.type) ?? schema?.format ?? undefined;
}

function fieldDescription(tool: Tool, field: string): string | undefined {
  const schema = propertySchema(tool, field);
  return schema?.description;
}

function isRequired(tool: Tool, field: string): boolean {
  const schema = tool.input_schema as Record<string, any> | undefined;
  const required = schema?.required as string[] | undefined;
  return required?.includes(field) ?? false;
}

function getInputType(tool: Tool, field: string): 'text' | 'number' | 'boolean' | 'select' | 'json' {
  const schema = propertySchema(tool, field);
  if (!schema) return 'text';
  
  // Check for enum
  if (schema.enum && Array.isArray(schema.enum)) {
    return 'select';
  }
  
  const type = normalizeType(schema.type);
  
  switch (type) {
    case 'number':
    case 'integer':
      return 'number';
    case 'boolean':
      return 'boolean';
    case 'array':
    case 'object':
      return 'json';
    default:
      return 'text';
  }
}

function getEnumOptions(tool: Tool, field: string): string[] {
  const schema = propertySchema(tool, field);
  return schema?.enum ?? [];
}

function getPlaceholder(tool: Tool, field: string): string {
  const schema = propertySchema(tool, field);
  const type = normalizeType(schema?.type);
  
  if (schema?.default !== undefined) {
    return `默认: ${JSON.stringify(schema.default)}`;
  }
  
  switch (type) {
    case 'number':
    case 'integer':
      return '输入数字';
    case 'array':
      return '["item1", "item2"]';
    case 'object':
      return '{"key": "value"}';
    default:
      return `输入 ${field}`;
  }
}

function getFieldMin(tool: Tool, field: string): number | undefined {
  const schema = propertySchema(tool, field);
  return schema?.minimum;
}

function getFieldMax(tool: Tool, field: string): number | undefined {
  const schema = propertySchema(tool, field);
  return schema?.maximum;
}

function getFieldStep(tool: Tool, field: string): number | undefined {
  const schema = propertySchema(tool, field);
  const type = normalizeType(schema?.type);
  return type === 'integer' ? 1 : undefined;
}

function schemaEntry(tool: Tool): Record<string, string> {
  if (!schemaArgs[tool.name]) {
    schemaArgs[tool.name] = {};
  }
  return schemaArgs[tool.name];
}

function booleanEntry(tool: Tool): Record<string, boolean> {
  if (!booleanArgs[tool.name]) {
    booleanArgs[tool.name] = {};
  }
  return booleanArgs[tool.name];
}

function coerceValue(raw: string, schema?: Record<string, any>): unknown {
  const trimmed = raw.trim();
  if (!trimmed) return undefined;
  const type = normalizeType(schema?.type);
  if (!type) return trimmed;
  switch (type) {
    case 'number':
    case 'integer':
      const numberValue = Number(trimmed);
      return Number.isFinite(numberValue) ? numberValue : trimmed;
    case 'boolean':
      if (['true', '1'].includes(trimmed.toLowerCase())) return true;
      if (['false', '0'].includes(trimmed.toLowerCase())) return false;
      return trimmed;
    case 'array':
    case 'object':
      try {
        return JSON.parse(trimmed);
      } catch {
        return trimmed;
      }
    default:
      return trimmed;
  }
}

function buildPayload(tool: Tool): Record<string, unknown> | undefined {
  const fieldList = fields(tool);
  if (fieldList.length === 0) {
    const raw = (jsonArgs[tool.name] ?? '').trim();
    if (!raw) {
      jsonErrors[tool.name] = null;
      return undefined;
    }
    try {
      const parsed = JSON.parse(raw);
      if (parsed && typeof parsed === 'object' && !Array.isArray(parsed)) {
        jsonErrors[tool.name] = null;
        return parsed as Record<string, unknown>;
      }
      jsonErrors[tool.name] = '请输入一个 JSON 对象';
      return undefined;
    } catch (err) {
      jsonErrors[tool.name] = err instanceof Error ? err.message : '无法解析 JSON';
      return undefined;
    }
  }
  
  const stringValues = schemaEntry(tool);
  const boolValues = booleanEntry(tool);
  const payload: Record<string, unknown> = {};
  
  fieldList.forEach((field) => {
    const inputType = getInputType(tool, field);
    const schema = propertySchema(tool, field);
    
    if (inputType === 'boolean') {
      // Boolean values
      if (boolValues[field] !== undefined) {
        payload[field] = boolValues[field];
      }
    } else {
      // String-based values
      const entry = stringValues[field] ?? '';
      const converted = coerceValue(entry, schema);
      if (converted !== undefined && converted !== '') {
        payload[field] = converted;
      }
    }
  });
  
  return Object.keys(payload).length === 0 ? undefined : payload;
}

function handleCall(tool: Tool) {
  const args = buildPayload(tool);
  if (jsonErrors[tool.name]) return;
  emits('call', { name: tool.name, args });
}
</script>

<style scoped>
.tool-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
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
  background: rgba(139, 92, 246, 0.1);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 16px;
  color: var(--primary-color);
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

.tool-card {
  background: var(--bg-card);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-lg);
  overflow: hidden;
  transition: all var(--transition-normal);
}

.tool-card:hover {
  border-color: var(--border-light);
  box-shadow: var(--shadow-sm);
}

.tool-card.expanded {
  border-color: rgba(139, 92, 246, 0.3);
  box-shadow: var(--shadow-glow);
}

.tool-header {
  padding: 16px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  cursor: pointer;
  transition: background var(--transition-fast);
}

.tool-header:hover {
  background: rgba(255, 255, 255, 0.02);
}

.tool-info {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  flex: 1;
  min-width: 0;
}

.tool-icon {
  width: 36px;
  height: 36px;
  background: var(--primary-gradient);
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  flex-shrink: 0;
}

.tool-meta {
  flex: 1;
  min-width: 0;
}

.tool-meta h3 {
  font-size: 0.95rem;
  font-weight: 600;
  margin: 0 0 4px;
  color: var(--text-primary);
}

.tool-desc-preview {
  font-size: 0.82rem;
  color: var(--text-muted);
  margin: 0;
  line-height: 1.4;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 400px;
}

.markdown-description {
  margin-top: 12px;
  font-size: 0.88rem;
  line-height: 1.6;
  color: var(--text-secondary);
  border-bottom: 1px dashed var(--border-subtle);
  padding-bottom: 16px;
}

.markdown-description :deep(p) { margin: 8px 0; }
.markdown-description :deep(code) { 
  background: rgba(255, 255, 255, 0.08);
  padding: 2px 5px;
  border-radius: 4px;
  font-family: var(--font-mono);
  font-size: 0.85em;
}
.markdown-description :deep(pre) {
  background: rgba(0, 0, 0, 0.3);
  padding: 12px;
  border-radius: 8px;
  overflow: auto;
  margin: 12px 0;
}
.markdown-description :deep(ul), .markdown-description :deep(ol) {
  padding-left: 20px;
  margin: 8px 0;
}
.markdown-description :deep(li) { margin: 4px 0; }
.markdown-description :deep(strong) { color: var(--text-primary); }

.tool-actions {
  display: flex;
  align-items: center;
  gap: 10px;
}

.fields-count {
  font-size: 0.75rem;
  padding: 4px 10px;
  background: rgba(139, 92, 246, 0.15);
  color: var(--primary-color);
  border-radius: var(--radius-full);
  font-weight: 500;
}

.expand-btn {
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

.expand-btn:hover {
  background: rgba(255, 255, 255, 0.1);
  color: var(--text-primary);
}

.expand-btn.rotated {
  transform: rotate(180deg);
}

.tool-body {
  padding: 0 16px 16px;
  border-top: 1px solid var(--border-subtle);
  animation: slideUp var(--transition-normal) ease-out;
}

.input-label {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 0.8rem;
  font-weight: 500;
  color: var(--text-muted);
  margin: 16px 0 10px;
}

.textarea-wrapper {
  position: relative;
}

.json-textarea {
  width: 100%;
  padding: 12px;
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border-light);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-family: 'JetBrains Mono', monospace;
  font-size: 0.85rem;
  resize: vertical;
  min-height: 100px;
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
}

.json-textarea.small {
  min-height: 70px;
  font-size: 0.8rem;
}

.json-textarea:focus {
  outline: none;
  border-color: var(--border-focus);
  box-shadow: 0 0 0 3px rgba(139, 92, 246, 0.15);
}

.json-textarea::placeholder {
  color: var(--text-dim);
}

.json-error {
  display: flex;
  align-items: center;
  gap: 6px;
  margin: 8px 0 0;
  font-size: 0.8rem;
  color: var(--error);
}

.fields-grid {
  display: grid;
  gap: 16px;
}

.schema-field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.field-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.field-info {
  display: flex;
  align-items: center;
  gap: 8px;
}

.field-name {
  font-size: 0.85rem;
  font-weight: 500;
  color: var(--text-secondary);
}

.required-badge {
  font-size: 0.65rem;
  padding: 2px 6px;
  background: rgba(239, 68, 68, 0.15);
  color: var(--error);
  border-radius: var(--radius-full);
  font-weight: 600;
}

.field-type {
  font-size: 0.7rem;
  padding: 2px 8px;
  background: rgba(6, 182, 212, 0.15);
  color: var(--accent-color);
  border-radius: var(--radius-full);
  font-family: 'JetBrains Mono', monospace;
}

.field-desc {
  font-size: 0.78rem;
  color: var(--text-dim);
  margin: 0 0 4px;
  line-height: 1.4;
}

.field-input {
  width: 100%;
  padding: 10px 12px;
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border-light);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-size: 0.9rem;
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
}

.field-input:focus {
  outline: none;
  border-color: var(--border-focus);
  box-shadow: 0 0 0 3px rgba(139, 92, 246, 0.15);
}

.field-input::placeholder {
  color: var(--text-dim);
}

/* Number input specific styling */
.field-input[type="number"] {
  -moz-appearance: textfield;
}

.field-input[type="number"]::-webkit-outer-spin-button,
.field-input[type="number"]::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

/* Select dropdown */
.field-select {
  width: 100%;
  padding: 10px 12px;
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border-light);
  border-radius: var(--radius-md);
  color: var(--text-primary);
  font-size: 0.9rem;
  cursor: pointer;
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24' fill='none' stroke='%239ca3af' stroke-width='2'%3E%3Cpolyline points='6 9 12 15 18 9'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 12px center;
  padding-right: 36px;
}

.field-select:focus {
  outline: none;
  border-color: var(--border-focus);
  box-shadow: 0 0 0 3px rgba(139, 92, 246, 0.15);
}

.field-select option {
  background: var(--bg-secondary);
  color: var(--text-primary);
}

/* Toggle switch for boolean */
.toggle-wrapper {
  display: flex;
  align-items: center;
  gap: 12px;
}

.toggle {
  position: relative;
  display: inline-block;
  width: 48px;
  height: 26px;
}

.toggle input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid var(--border-light);
  border-radius: 26px;
  transition: all var(--transition-fast);
}

.toggle-slider:before {
  position: absolute;
  content: "";
  height: 20px;
  width: 20px;
  left: 2px;
  bottom: 2px;
  background: var(--text-muted);
  border-radius: 50%;
  transition: all var(--transition-fast);
}

.toggle input:checked + .toggle-slider {
  background: rgba(139, 92, 246, 0.3);
  border-color: var(--primary-color);
}

.toggle input:checked + .toggle-slider:before {
  transform: translateX(22px);
  background: var(--primary-color);
}

.toggle input:focus + .toggle-slider {
  box-shadow: 0 0 0 3px rgba(139, 92, 246, 0.15);
}

.toggle-label {
  font-size: 0.85rem;
  color: var(--text-secondary);
}

.run-button {
  width: 100%;
  margin-top: 16px;
  padding: 12px 20px;
  background: var(--primary-gradient);
  border: none;
  border-radius: var(--radius-md);
  color: white;
  font-size: 0.9rem;
  font-weight: 600;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  transition: all var(--transition-fast);
  box-shadow: 0 4px 12px rgba(139, 92, 246, 0.25);
}

.run-button:hover {
  transform: translateY(-1px);
  box-shadow: 0 6px 16px rgba(139, 92, 246, 0.35);
}

.run-button:active {
  transform: translateY(0);
}

@keyframes slideUp {
  from { 
    opacity: 0; 
    transform: translateY(-10px); 
  }
  to { 
    opacity: 1; 
    transform: translateY(0); 
  }
}
</style>
