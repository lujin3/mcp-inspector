export interface Tool {
  name: string;
  description?: string;
  title?: string;
  input_schema?: Record<string, any>;
}

export interface Resource {
  uri: string;
  name: string;
  title?: string;
  description?: string;
  mime_type?: string;
}

export interface Prompt {
  name: string;
  description?: string;
}

export interface HeaderPair {
  name: string;
  value: string;
}

export interface ConnectResult {
  connected: boolean;
  url: string;
  headers: HeaderPair[];
}

export interface ToolCallPayload {
  name: string;
  args?: Record<string, unknown>;
}
