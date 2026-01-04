# MCP Inspector

基于 Tauri 2 + Vue 3 重新实现的 MCP (Model Context Protocol) 检查器，前端由 Vue 负责渲染工具/资源/提示与日志面板，Rust 后端透过 Tauri 命令调用 `rmcp` 客户端并与 MCP 服务器交互。

## 目录结构

- `frontend/`：Vue 3 + Vite 单页应用，渲染界面并通过 `@tauri-apps/api/tauri` 的 `invoke` 调用 Rust 命令（`connect_mcp`, `list_tools`, `call_tool`, 等）。  
- `src-tauri/`：Tauri 2 Rust 后端，包含所有 MCP 通道逻辑，利用 `rmcp` 和 `reqwest` 向服务器发起请求，并返回 JSON 结构给前端。  
- `Cargo.toml`：工作区配置，仅包含 `src-tauri` crate。

## 运行

### 准备

1. 安装 Node.js 与 npm/yarn；  
2. 安装 Rust toolchain（stable）和 Tauri CLI（`cargo install tauri-cli`）。  
3. 若在 Linux 上构建，确保系统提供 [WebKitGTK](https://webkitgtk.org/)，例如通过 `sudo apt install libwebkit2gtk-4.1-dev` 以满足 Tauri/Wry 的 C 依赖，否则 `cargo tauri build`/`cargo check` 会报缺少 `webkit2gtk-4.1`。

### 开发模式

```bash
cd frontend
npm install
npm run tauri
```

该命令会先启动 Vite 开发服务器（`localhost:5173`），再通过 Tauri 打开桌面窗口。修改 Vue 文件会自动热更新。

### 构建发布

```bash
cd frontend
npm install
npm run build
cd ../src-tauri
cargo tauri build
```

Vue 资产会被打包到 `frontend/dist`，Tauri 会将其嵌入最终应用。

## 后端命令

Rust 侧提供以下 `tauri::command` 接口供 Vue 调用：

- `connect_mcp(payload)`：连接 MCP 服务器，可以传入 `url`（默认 `http://localhost:8000/mcp`）与一组 `HeaderPair`，命令会在结果里返回已经发送的 headers 以方便前端记录。  
- `list_tools()` / `list_resources()` / `list_prompts()`：返回简化的工具/资源/提示结构供前端渲染。  
- `call_tool({ name, args })`：调用工具时传入 optional JSON 对象（UI 会根据 schema 或 raw JSON 自动拼装），命令只接受对象类型以避免非结构化 payload。

这些命令都共享一个 `McpSession`，它在全局状态里维护 `rmcp::service::RunningService` 实例，确保前端调用期间连接不会被重建。

## 前端交互

- 顶部连接面板允许指定服务器 URL、维护一组 header 并展示最后一次连接的 URL/headers 与连接按钮，操作结果会写入日志流。  
- 左侧工具卡片展示名称/描述，并根据工具的 `input_schema.properties` 自动生成字段输入；若未提供 schema，则支持自由 JSON 并会校验是否为对象。  
- 右侧面板依次展示 Resources、Prompts 与日志信息，日志也会记录调用工具/连接的返回值与错误。  
- 顶部状态栏显示连接状态，Vue 插件 `@tauri-apps/api` 的 `invoke` 用于向 Rust 发送命令。

## 技术栈

- Rust + Tauri：负责 MCP 通信与命令暴露。  
- Vue 3 + Vite：提供响应式界面与表单控件。  
- rmcp：官方 MCP Rust SDK。  
- reqwest + sse-stream：实现可容忍省略 `Content-Type` 的 SSE transport。

## 贡献

欢迎提交 Issue 或 Pull Request；如果需要扩展工具调用、添加提示模板、或替换 UI 样式都可以在 frontend 里进行迭代。
