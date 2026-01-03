# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

RikkaHub is a cross-platform AI chat client with:
- **Desktop client**: Rust + GPUI (GPU-accelerated UI framework from Zed)
- **Server**: Go (Fiber HTTP API)

## Project Structure

```
rikkahub/
├── desktop/           # Rust desktop client
│   ├── Cargo.toml
│   └── crates/
│       ├── core/      # Shared models
│       └── desktop/   # GPUI application
└── server/            # Go backend
    ├── cmd/server/    # Application entry point
    ├── internal/
    │   ├── config/    # Environment configuration
    │   ├── database/  # Database connection
    │   ├── handlers/  # HTTP request handlers
    │   ├── middleware/# Auth, i18n middleware
    │   ├── models/    # GORM models and migrations
    │   └── services/  # Business logic (auth, jwt, password, settings, i18n)
    ├── locales/       # i18n JSON files (en, zh)
    └── pkg/utils/     # Utility functions
```

## Server Development

### Tech Stack
- **Framework**: Fiber v2 (high-performance Go web framework)
- **ORM**: GORM v1.31
- **Auth**: JWT v5 with Argon2id password hashing
- **i18n**: go-i18n v2

### Commands

```bash
# Build server
cd server && go build -o server ./cmd/server

# Run server (requires .env file)
cd server && ./server

# Run tests
cd server && go test ./...

# Run single test
cd server && go test -run TestName ./internal/services/
```

### Configuration

Server requires `.env` file with:
```env
SERVER_PORT=3000
DATABASE_URL=postgres://user:pass@localhost:5432/rikkahub?sslmode=disable
JWT_SECRET=your-secret-key
JWT_ACCESS_EXPIRES_HOURS=24
JWT_REFRESH_EXPIRES_DAYS=7
```

### Architecture Patterns

- **Layered design**: models → services → handlers
- **Dependency injection**: Services injected into handlers
- **Settings cache**: 5-minute TTL with auto-refresh
- **Permission system**: Wildcard support (e.g., `model.*.use`, `*`)

### Default User Groups
- `guest`: Limited permissions (`conversation.create`, `conversation.read`, `model.gpt-3.5.use`)
- `user` (default): Standard permissions (`conversation.*`, `model.*.use`)
- `admin`: Full access (`*`)
