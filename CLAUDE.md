# CLAUDE.md

This file provides context for Claude Code when working with this repository.

## Project Overview

**zero2prod** is a newsletter subscription service built following the "Zero to Production in Rust" book. It's an async web application using Actix-web and PostgreSQL.

## Tech Stack

- **Web Framework:** Actix-web 4.12
- **Database:** PostgreSQL with SQLx (async, compile-time checked queries)
- **Async Runtime:** Tokio
- **Logging:** Tracing with Bunyan JSON formatter
- **Configuration:** YAML-based (config crate)
- **Validation:** validator crate + custom domain validation

## Project Structure

```
src/
├── bin/main.rs        # Application entry point
├── lib.rs             # Library root
├── configuration.rs   # Config loading from YAML
├── startup.rs         # HTTP server setup and routing
├── telemetry.rs       # Tracing/logging setup
├── domain/            # Business logic and validation
│   ├── new_subscriber.rs
│   ├── subscriber_email.rs
│   └── subscriber_name.rs
└── routes/            # HTTP handlers
    ├── health_check.rs
    └── subscriptions.rs

tests/api/             # Integration tests
configuration/         # Environment configs (base.yaml, local.yaml, production.yaml)
migrations/            # SQLx database migrations
scripts/               # Database setup scripts
```

## Common Commands

```bash
# Database setup (requires Docker or existing PostgreSQL)
.\scripts\init_db.ps1              # Windows
./scripts/init_db.sh               # Linux/Mac
SKIP_DOCKER=true ./scripts/init_db.sh  # Use existing DB

# Development
cargo build                        # Build the project
cargo run                          # Run server (port 8000)
cargo test                         # Run all tests

# Code quality
cargo fmt                          # Format code
cargo clippy                       # Lint
cargo clippy -- -D warnings        # Lint (strict)

# SQLx (offline mode for CI)
cargo sqlx prepare                 # Generate query metadata
```

## Configuration

Configuration loads from `configuration/` directory:
- `base.yaml` - Default settings
- `local.yaml` - Development overrides (default)
- `production.yaml` - Production overrides

Set `APP_ENVIRONMENT=production` to use production config.

### Database Connection

Default local settings:
- Host: 127.0.0.1:5432
- Database: newsletter
- User: app / secret (via init script) or postgres / password (base config)

## API Endpoints

- `GET /health_check` - Returns 200 OK
- `POST /subscriptions` - Subscribe with form data (`name`, `email`)

## Testing

Integration tests spawn isolated test servers with dedicated databases. Tests are in `tests/api/`.

Key test helpers are in `tests/api/helpers.rs` (spawns app, creates test DB).

## Code Conventions

- Domain validation in `src/domain/` (email format, name length/characters)
- Use `tracing` macros for logging (`tracing::info!`, `tracing::error!`)
- Errors return appropriate HTTP status codes (400 for validation, 500 for server errors)
- Property-based testing with quickcheck for domain validation
