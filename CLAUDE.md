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

### Settings System

**Model** (`models.Setting`):
- **Key**: String primary key (e.g., `auth.allow_register`)
- **Value**: JSONB field storing any JSON-serializable value
- **Type**: Enum (`bool`, `string`, `int`, `json`)
- **Description**: Optional human-readable description

**Service** (`services.Settings`):
- **Global singleton**: Initialized once via `InitSettings(db)`
- **Cache**: In-memory map with 5-minute TTL and auto-refresh
- **Thread-safe**: Uses `sync.RWMutex` for concurrent access
- **Generic methods**: `GetBool()`, `GetString()`, `GetInt()`, `Set()`
- **Typed setters**: `SetBool()`, `SetString()`, `SetInt()`

**Usage Example**:
```go
// Get global instance
settings := services.GetSettings()

// Using convenience accessors
if settings.Auth().AllowRegister() {
    // handle registration
}

// Direct access
maxMessages := settings.GetInt("chat.max_context_messages", 50)

// Update setting (updates both cache and DB)
err := settings.SetBool("auth.allow_register", false)
```

### Default User Groups
- `guest`: Limited permissions (`conversation.create`, `conversation.read`, `model.gpt-3.5.use`)
- `user` (default): Standard permissions (`conversation.*`, `model.*.use`)
- `admin`: Full access (`*`)
