# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

RikkaHub is a cross-platform AI chat client built with Rust and GPUI (GPU-accelerated UI framework from Zed). It uses Rust Edition 2024.

## Build Commands

```bash
# Build all crates
cargo build

# Run desktop client (macOS)
cargo run -p desktop

# Run standalone server
cargo run -p server
# With custom host/port: HOST=0.0.0.0 PORT=8080 cargo run -p server

# Run tests
cargo test --workspace

# Run single crate tests
cargo test -p server

# Format and lint
cargo fmt
cargo clippy --all-targets --all-features
```

## Architecture

Three-crate workspace with client-server architecture:

```
core     → Domain models (Message, ChatRequest/Response, Conversation) and error types
  ↓
server   → Axum HTTP server (handlers, state, middleware) → Docker image

desktop  → GPUI desktop client with HTTP client (connects to server)
```

**Key architectural decisions:**
- Server is standalone, designed for Docker deployment
- Desktop connects to server via HTTP (no embedded API)
- Server can freely use any middleware (Redis, database pools, etc.)
- HTTP endpoints: `/api/chat/send`, `/api/models`, `/api/conversations`
- Tower middleware stack: CORS, gzip compression, tracing
- Structured logging via `tracing` with env-filter (`RUST_LOG=debug`)

## Adding Features

**New API endpoint:**
1. Add request/response types in `crates/core/src/models.rs`
2. Add handler in `crates/server/src/handlers.rs`
3. Register route in `crates/server/src/main.rs`
4. Add client method in `crates/desktop/src/client.rs`

**New error type:**
Add variant to `CoreError` in `crates/core/src/error.rs`

## Current Status

The project is in early development. API handlers return mock data (see TODO comments in `handlers.rs`). Desktop UI shows placeholder content. No database or authentication yet.
