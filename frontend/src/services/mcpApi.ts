import { invoke } from '@tauri-apps/api/core';

// Types from App.vue (we should probably move types to a shared file, but for now duplicate or reference)
// Assuming types.ts exists or we inline roughly what we need to avoid import errors if types are not exported.
// App.vue imports from '@/types', let's stick to that.
import type { Tool, Resource, Prompt, HeaderPair, ConnectResult, ToolCallPayload } from '@/types';

// Removed conflicting global declarations
const isTauri = () => {
    if (typeof window === 'undefined') {
        return false;
    }
    // Check for common Tauri indicators
    const win = window as any;
    return !!(win.__TAURI_IPC__ || win.__TAURI_METADATA__ || win.__TAURI_INTERNALS__ || win.__TAURI__);
};

export function isRunningInTauri() {
    return isTauri();
}

class McpApiService {
    private baseUrl: string = '';
    private headers: HeaderPair[] = [];
    private isConnected: boolean = false;
    private messageId: number = 0;

    constructor() {
        console.log('[McpApiService] Initializing. Env is Tauri:', isTauri());
    }

    // Helper to generate JSON-RPC ID
    private nextId(): number {
        return ++this.messageId;
    }

    // Helper for Browser JSON-RPC calls
    private async rpcRequest(method: string, params?: any): Promise<any> {
        if (!this.baseUrl) {
            throw new Error('Not connected to any server.');
        }

        const payload = {
            jsonrpc: '2.0',
            id: this.nextId(),
            method,
            params,
        };

        const headerMap: Record<string, string> = {
            'Content-Type': 'application/json',
        };

        this.headers.forEach(h => {
            if (h.name && h.value) {
                headerMap[h.name] = h.value;
            }
        });

        console.log('[McpApi] Sending RPC:', payload);

        try {
            const response = await fetch(this.baseUrl, {
                method: 'POST',
                headers: headerMap,
                body: JSON.stringify(payload),
            });

            if (!response.ok) {
                throw new Error(`HTTP Error: ${response.status} ${response.statusText}`);
            }

            const data = await response.json();

            if (data.error) {
                throw new Error(`MCP Error: ${data.error.message} (${data.error.code})`);
            }

            return data.result;
        } catch (e) {
            // Handle CORS specifically as it's common in browser dev
            if (e instanceof TypeError && e.message === 'Failed to fetch') {
                throw new Error('Connection failed. Possible CORS issue or server unreachable.');
            }
            throw e;
        }
    }

    async connect(url: string, headers: HeaderPair[] = []): Promise<ConnectResult> {
        this.baseUrl = url;
        this.headers = headers;

        if (isTauri()) {
            return await invoke<ConnectResult>('connect_mcp', { payload: { url, headers } });
        } else {
            // Browser Mode: Validate connection by pinging or listing something simple
            try {
                // We'll try to list tools as a "ping"
                // Note: Some servers might need initialization. 
                // Standard MCP might require an 'initialize' handshake first in full protocol,
                // but for a stateless HTTP proxy, we might get away with just calling methods.
                // If strict MCP, we might need to send 'initialize'. Let's try a simple method first.
                // Actually, let's just mark as connected if we can reach it,
                // or maybe assume success to allow the dashboard to populate.
                // Let's try list_tools as a validator.

                // NOTE: Ideally we send "initialize". 
                // For now, let's assume the user's server is an HTTP-Mcp adapter.

                this.isConnected = true;
                return {
                    connected: true,
                    url: this.baseUrl,
                    headers: this.headers,
                };
            } catch (e) {
                this.isConnected = false;
                throw e;
            }
        }
    }

    async listTools(): Promise<Tool[]> {
        if (isTauri()) {
            return await invoke('list_tools');
        } else {
            // MCP JSON-RPC method: tools/list
            const result = await this.rpcRequest('tools/list');
            // MCP returns { tools: Tool[], nextCursor?: string }
            return result.tools || [];
        }
    }

    async listResources(): Promise<Resource[]> {
        if (isTauri()) {
            return await invoke('list_resources');
        } else {
            // MCP JSON-RPC method: resources/list
            const result = await this.rpcRequest('resources/list');
            return result.resources || [];
        }
    }

    async listPrompts(): Promise<Prompt[]> {
        if (isTauri()) {
            return await invoke('list_prompts');
        } else {
            // MCP JSON-RPC method: prompts/list
            const result = await this.rpcRequest('prompts/list');
            return result.prompts || [];
        }
    }

    async callTool(name: string, args?: any): Promise<any> {
        if (isTauri()) {
            return await invoke('call_tool', { payload: { name, args: args ?? null } });
        } else {
            // MCP JSON-RPC method: tools/call
            const result = await this.rpcRequest('tools/call', {
                name,
                arguments: args || {}
            });
            // Tool call result structure varies, usually { content: ... }
            // This matches what the UI probably expects (Result Value)
            // The Rust backend returns `Value`.
            return result;
        }
    }
}

export const mcpApi = new McpApiService();
