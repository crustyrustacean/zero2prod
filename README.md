# zero2prod

A newsletter subscription service built following [Zero To Production In Rust](https://www.zero2prod.com/) by Luca Palmieri.

[![CI](https://github.com/crustyrustacean/zero2prod/actions/workflows/general.yml/badge.svg)](https://github.com/crustyrustacean/zero2prod/actions/workflows/general.yml)
[![Security Audit](https://github.com/crustyrustacean/zero2prod/actions/workflows/audit.yml/badge.svg)](https://github.com/crustyrustacean/zero2prod/actions/workflows/audit.yml)

## Book Progress

| Chapter | Title | Status |
|---------|-------|--------|
| 1 | Getting Started | :white_check_mark: Complete |
| 2 | Building An Email Newsletter | :white_check_mark: Complete |
| 3 | Sign Up A New Subscriber | :white_check_mark: Complete |
| 4 | Telemetry | :white_check_mark: Complete |
| 5 | Going Live (Deployment) | :white_check_mark: Complete |
| 6 | Reject Invalid Subscribers | :white_check_mark: Complete |
| 7 | Reject Invalid Subscribers (Part 2) | :hourglass: Pending |
| 8 | Error Handling | :hourglass: Pending |
| 9 | Naive Newsletter Delivery | :hourglass: Pending |
| 10 | Securing Our API | :hourglass: Pending |
| 11 | Fault-Tolerant Workflows | :hourglass: Pending |

## Tech Stack

- **Web Framework:** [Actix-web](https://actix.rs/) 4.12
- **Database:** PostgreSQL with [SQLx](https://github.com/launchbadge/sqlx) (async, compile-time verified queries)
- **Async Runtime:** [Tokio](https://tokio.rs/)
- **Observability:** [Tracing](https://github.com/tokio-rs/tracing) with Bunyan JSON formatting
- **Configuration:** YAML-based using [config](https://github.com/mehcode/config-rs)

## Prerequisites

- Rust (Edition 2024)
- PostgreSQL 14+ (or Docker)
- sqlx-cli:
  ```bash
  cargo install --version='~0.8' sqlx-cli --no-default-features --features rustls,postgres
  ```

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/crustyrustacean/zero2prod.git
cd zero2prod
```

### 2. Set up the database

**Windows (PowerShell):**
```powershell
.\scripts\init_db.ps1
```

**Linux/macOS:**
```bash
./scripts/init_db.sh
```

**Using an existing PostgreSQL instance:**
```bash
SKIP_DOCKER=true ./scripts/init_db.sh
```

### 3. Run the application

```bash
cargo run
```

The server starts at `http://127.0.0.1:8000`.

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health_check` | Health check (returns 200 OK) |
| POST | `/subscriptions` | Subscribe to newsletter |

### Example: Subscribe

```bash
curl -X POST http://127.0.0.1:8000/subscriptions \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "name=John%20Doe&email=john@example.com"
```

## Development

### Run tests

```bash
cargo test
```

### Code quality

```bash
cargo fmt          # Format code
cargo clippy       # Lint
```

### Prepare for offline builds (CI)

```bash
cargo sqlx prepare
```

## Configuration

Configuration files are in the `configuration/` directory:

| File | Purpose |
|------|---------|
| `base.yaml` | Default settings |
| `local.yaml` | Development overrides |
| `production.yaml` | Production overrides |

Set `APP_ENVIRONMENT=production` to use production configuration.

## Project Structure

```
src/
├── bin/main.rs        # Application entry point
├── configuration.rs   # Configuration loading
├── startup.rs         # Server setup and routing
├── telemetry.rs       # Tracing/logging setup
├── domain/            # Business logic and validation
└── routes/            # HTTP request handlers

tests/api/             # Integration tests
migrations/            # Database migrations
scripts/               # Setup scripts
```

## Docker

Build and run with Docker:

```bash
docker build -t zero2prod .
docker run -e APP_ENVIRONMENT=production -p 8000:8000 zero2prod
```

## License

MIT
