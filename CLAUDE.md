# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

RikkaHub 是一个跨平台高性能 AI 聊天客户端，采用 Rust monorepo 架构。项目处于早期开发阶段。

## 常用命令

```bash
# 构建桌面客户端（默认）
cargo build

# 构建服务端
cargo build -p server

# 运行桌面客户端
cargo run

# 运行服务端
cargo run -p server

# 检查代码
cargo check --workspace

# 运行测试
cargo test --workspace

# 格式化代码
cargo fmt --all

# Lint 检查
cargo clippy --workspace
```

## 架构

### Workspace 结构

- `crates/shared` - 共享代码库，包含聊天消息模型（Message, ChatRequest, ChatResponse）、错误类型定义
- `crates/desktop` - 桌面客户端，使用 GPUI 框架构建原生界面，输出二进制 `rikkahub`
- `crates/server` - 后端服务，使用 Axum + SQLx + PostgreSQL，输出二进制 `rikkahub-server`

### 技术栈

- **桌面端**: GPUI (Zed 编辑器的 GUI 框架) + gpui-component
- **服务端**: Axum (Web 框架) + SQLx (数据库) + PostgreSQL
- **通用**: Tokio (异步运行时), Serde (序列化), Tracing (日志)

### 服务端模块

服务端采用分层架构：
- `config/` - 环境配置（从 .env 读取）
- `database/` - 数据库连接池和迁移
- `models/` - 数据库实体（User, Group, GroupPermission, Setting 等）
- `handlers/` - HTTP 请求处理器
- `routes/` - 路由定义
- `services/` - 业务逻辑层
- `middleware/` - 中间件
- `utils/` - 工具函数

### 数据库

- 使用 PostgreSQL
- 迁移文件位于 `crates/server/migrations/`
- 主要表：users, groups, user_groups, group_permissions, settings
- 启动时自动运行迁移

## 开发环境配置

服务端需要配置环境变量，参考 `crates/server/.env.example`：

```bash
SERVER_PORT=3000
SERVER_HOST=0.0.0.0
DATABASE_URL=postgres://postgres:password@localhost:5432/rikkahub?sslmode=disable
DATABASE_MAX_CONNECTIONS=10
```

## 注意事项

- 项目使用 Rust 2024 edition
- 默认构建目标是 desktop crate
- 桌面客户端日志级别通过 `RUST_LOG` 环境变量控制
