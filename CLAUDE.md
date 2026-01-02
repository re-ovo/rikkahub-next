# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

RikkaHub is a cross-platform AI chat client with:
- **Desktop client**: Rust + GPUI (GPU-accelerated UI framework from Zed)
- **Server**: Go (HTTP API)

## Project Structure

```
rikkahub/
├── desktop/           # Rust desktop client
│   ├── Cargo.toml
│   └── crates/
│       ├── core/      # Shared models
│       └── desktop/   # GPUI application
└── server/            # Go backend (TODO)
```
