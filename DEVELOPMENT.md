# 开发说明

## 项目架构

### 模块划分

```
mcp-inspector
├── main.rs           - 应用入口,CLI 参数解析
├── mcp/             - MCP 客户端模块
│   ├── client.rs    - 核心客户端实现
│   └── ...          - 各功能模块
├── logging/         - 日志系统
│   ├── collector.rs - 日志收集
│   ├── formatter.rs - 日志格式化
│   └── storage.rs   - 日志存储
└── ui/              - 用户界面
    ├── app.rs       - TUI 主应用
    └── components/  - UI 组件
```

### 关键设计决策

1. **异步架构**: 使用 Tokio 异步运行时,支持高效的 I/O 操作
2. **模块化设计**: 清晰的模块划分,便于维护和扩展
3. **类型安全**: 充分利用 Rust 的类型系统,减少运行时错误
4. **错误处理**: 使用 `anyhow` 和 `Result` 类型,统一错误处理

## 扩展指南

### 添加新的 MCP 功能

1. 在 `src/mcp/client.rs` 中添加新方法
2. 在 UI 中添加对应的显示逻辑
3. 更新文档

### 添加新的 UI 组件

1. 在 `src/ui/components/` 中创建新文件
2. 在 `src/ui/app.rs` 中集成组件
3. 添加键盘快捷键支持

### 自定义日志格式

修改 `src/logging/formatter.rs` 中的 `format` 方法

## 性能优化建议

1. **日志缓冲**: 当前限制为 1000 条,可根据需要调整
2. **UI 刷新率**: 当前为 100ms,可根据性能需求调整
3. **发布构建**: 使用 `--release` 标志获得最佳性能

## 调试技巧

### 启用详细日志

```bash
RUST_LOG=trace ./target/debug/mcp-inspector --server ...
```

### 使用 Rust 调试器

```bash
rust-gdb ./target/debug/mcp-inspector
```

## 常见问题

### Q: 如何支持新的传输协议?

A: 在 `src/mcp/client.rs` 中的 `McpClient::new_from_command` 方法中添加新的传输类型支持。

### Q: 如何添加新的标签页?

A: 
1. 在 `Tab` 枚举中添加新变体
2. 更新 `titles()` 和转换方法
3. 在 `render()` 方法中添加渲染逻辑

### Q: 如何自定义 UI 主题?

A: 修改 `src/ui/app.rs` 中的 `Style` 定义,使用不同的颜色和样式。

## 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 代码风格

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 遵循 Rust 官方风格指南
