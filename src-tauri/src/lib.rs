// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod session;

use self::session::{McpSession, PromptView, ResourceView, ToolView};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tauri::{command, State};
use tokio::sync::Mutex;

const DEFAULT_URL: &str = "http://localhost:8003/mcp";

pub struct AppState {
    session: Mutex<Option<Arc<McpSession>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            session: Mutex::new(None),
        }
    }
}

#[derive(Default, Deserialize)]
pub struct ConnectPayload {
    url: Option<String>,
    headers: Option<Vec<HeaderPair>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HeaderPair {
    pub name: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct ConnectResult {
    pub connected: bool,
    pub url: String,
    pub headers: Vec<HeaderPair>,
}

#[derive(Deserialize)]
pub struct CallToolPayload {
    pub name: String,
    pub args: Option<Value>,
}

pub fn run() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            connect_mcp,
            list_tools,
            list_resources,
            list_prompts,
            call_tool
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[command]
async fn connect_mcp(
    state: State<'_, AppState>,
    payload: Option<ConnectPayload>,
) -> Result<ConnectResult, String> {
    println!(
        "Received connect_mcp command with payload: {:?}",
        payload.is_some()
    );
    let payload = payload.unwrap_or_default();

    let url = match payload.url {
        Some(ref u) if !u.trim().is_empty() => u.trim().to_string(),
        _ => {
            println!("No URL provided in payload, using default: {}", DEFAULT_URL);
            DEFAULT_URL.to_string()
        }
    };
    let headers = payload
        .headers
        .unwrap_or_default()
        .into_iter()
        .map(|pair| HeaderPair {
            name: pair.name.trim().to_string(),
            value: pair.value.trim().to_string(),
        })
        .filter(|pair| !pair.name.is_empty() && !pair.value.is_empty())
        .collect::<Vec<_>>();
    let header_tuples = headers
        .iter()
        .map(|pair| (pair.name.clone(), pair.value.clone()))
        .collect::<Vec<_>>();
    let session_url = url.clone();

    println!("Connecting to {} with headers: {:?}", url, headers);

    let session = match McpSession::connect(session_url.clone(), header_tuples).await {
        Ok(session) => session,
        Err(err) => {
            let header_summary = if headers.is_empty() {
                "无".to_string()
            } else {
                headers
                    .iter()
                    .map(|header| format!("{}: {}", header.name, header.value))
                    .collect::<Vec<_>>()
                    .join(" | ")
            };
            println!(
                "Failed to connect to {} (headers: {}): {}",
                session_url, header_summary, err
            );
            return Err(err);
        }
    };
    let mut guard = state.session.lock().await;
    *guard = Some(Arc::new(session));
    Ok(ConnectResult {
        connected: true,
        url,
        headers,
    })
}

#[command]
async fn list_tools(state: State<'_, AppState>) -> Result<Vec<ToolView>, String> {
    let session = {
        let guard = state.session.lock().await;
        guard
            .as_ref()
            .cloned()
            .ok_or_else(|| "Not connected to MCP server".to_string())?
    };
    session.list_tools().await
}

#[command]
async fn list_resources(state: State<'_, AppState>) -> Result<Vec<ResourceView>, String> {
    let session = {
        let guard = state.session.lock().await;
        guard
            .as_ref()
            .cloned()
            .ok_or_else(|| "Not connected to MCP server".to_string())?
    };
    session.list_resources().await
}

#[command]
async fn list_prompts(state: State<'_, AppState>) -> Result<Vec<PromptView>, String> {
    let session = {
        let guard = state.session.lock().await;
        guard
            .as_ref()
            .cloned()
            .ok_or_else(|| "Not connected to MCP server".to_string())?
    };
    session.list_prompts().await
}

#[command]
async fn call_tool(state: State<'_, AppState>, payload: CallToolPayload) -> Result<Value, String> {
    let args = match payload.args {
        Some(Value::Object(map)) => Some(map),
        Some(_) => return Err("Tool arguments must be a JSON object".into()),
        None => None,
    };
    let session = {
        let guard = state.session.lock().await;
        guard
            .as_ref()
            .cloned()
            .ok_or_else(|| "Not connected to MCP server".to_string())?
    };
    session.call_tool(payload.name, args).await
}
