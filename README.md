# Ralph Desktop

一个可视化的 Ralph Loop 控制器桌面应用，使用 Tauri 2.x + Svelte 5 + TypeScript + Rust 构建。

## 功能特性

### MVP 功能

- **项目管理**：创建、删除、切换项目
- **Brainstorm 引擎**：通过苏格拉底式问答引导生成任务 prompt
- **Loop 执行引擎**：支持 pause/resume/stop 控制
- **日志查看器**：实时显示 CLI 输出，支持 ANSI 颜色
- **恢复机制**：检测中断任务并提供恢复选项
- **通知系统**：Toast 消息提示任务状态
- **队列管理**：支持并发项目队列
- **键盘快捷键**：Cmd+N/Cmd+,/Cmd+? 等快捷操作
- **主题支持**：浅色/深色/跟随系统

### 支持的 CLI

- Claude Code (`claude`)
- OpenAI Codex (`codex`)

## 技术栈

- **前端**：Svelte 5 + TypeScript + Tailwind CSS 4
- **后端**：Rust + Tauri 2.x
- **构建**：Vite + Cargo

## 项目结构

```
ralph-desktop-cc/
├── src/                    # Svelte 前端
│   ├── lib/
│   │   ├── components/     # UI 组件
│   │   ├── services/       # Tauri API 封装
│   │   ├── stores/         # Svelte stores
│   │   └── types/          # TypeScript 类型
│   └── routes/             # SvelteKit 路由
├── src-tauri/              # Rust 后端
│   └── src/
│       ├── adapters/       # CLI 适配器
│       ├── commands/       # Tauri 命令
│       ├── engine/         # Loop/Brainstorm 引擎
│       ├── security/       # 安全工具
│       └── storage/        # 数据持久化
└── docs/                   # 文档
```

## 开发

### 前置要求

- Node.js 18+
- pnpm
- Rust 1.70+
- Tauri CLI

### 安装依赖

```bash
pnpm install
```

### 开发模式

```bash
pnpm tauri dev
```

### 构建

```bash
pnpm tauri build
```

## 数据存储

应用数据存储在：
- macOS: `~/Library/Application Support/com.ralph.desktop/`
- Windows: `%APPDATA%/com.ralph.desktop/`
- Linux: `~/.config/com.ralph.desktop/`

## 许可证

MIT
