#!/usr/bin/env bash

set -euo pipefail

echo "🚀 MCP Inspector 快速启动"
echo "=========================="

if ! command -v npm &>/dev/null; then
  echo "❌ npm 未安装（请先安装 Node.js）"
  exit 1
fi

if ! command -v cargo &>/dev/null; then
  echo "❌ cargo 未安装"
  exit 1
fi

cd frontend
echo "📦 安装前端依赖..."
npm install

echo "⚙️ 启动开发模式（Vite + Tauri）"
npm run tauri
