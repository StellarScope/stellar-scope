# Development Guide

## Quick Start

### Local Development

1. **Setup environment:**
   ```bash
   cp .env.example .env
   ```

2. **Start PostgreSQL (Docker):**
   ```bash
   docker run -d \
     --name stellarscope-postgres \
     -e POSTGRES_USER=stellarscope \
     -e POSTGRES_PASSWORD=password \
     -e POSTGRES_DB=stellarscope \
     -p 5432:5432 \
     postgres:16-alpine
   ```

3. **Build Rust services:**
   ```bash
   cargo build
   ```

4. **Run API service:**
   ```bash
   cargo run -p api
   ```

5. **Run indexer service:**
   ```bash
   cargo run -p indexer
   ```

6. **Run workers service:**
   ```bash
   cargo run -p workers
   ```

7. **Setup frontend:**
   ```bash
   cd apps/web
   pnpm install
   pnpm dev
   ```

### Docker Compose

Start all services:
```bash
docker-compose -f infrastructure/docker/docker-compose.yml up
```

View logs:
```bash
docker-compose -f infrastructure/docker/docker-compose.yml logs -f
```

Stop services:
```bash
docker-compose -f infrastructure/docker/docker-compose.yml down
```

## Project Layout

### Rust Services

- **services/api** - REST API server (Axum)
- **services/indexer** - Blockchain data indexer
- **services/workers** - Background job processor

### Shared Crates

- **crates/types** - Shared type definitions
- **crates/db** - Database layer (SQLx)
- **crates/decoder** - XDR decoding utilities

### Frontend

- **apps/web** - Next.js application

## Common Tasks

### Add a new Rust dependency

Edit `Cargo.toml` in the workspace root under `[workspace.dependencies]`, then reference it in service `Cargo.toml` files.

### Add a new API endpoint

1. Add handler in `services/api/src/handlers/mod.rs`
2. Add route in `services/api/src/main.rs`

### Add a new database model

1. Define struct in `crates/types/src/lib.rs`
2. Add queries in `crates/db/src/lib.rs`

### Add a new frontend page

Create file in `apps/web/src/app/` following Next.js App Router conventions.

## Testing

```bash
# Run all tests
cargo test

# Run tests for specific package
cargo test -p api

# Run with output
cargo test -- --nocapture
```

## Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check types
cargo check
```

## Environment Variables

See `.env.example` for all available configuration options.

## Database

### Migrations

TODO: Add migration setup and scripts

### Connection Pool

Configured in `crates/db/src/lib.rs`

## Logging

Logging is configured via `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run -p api
```

## Performance

- Tokio for async runtime
- Connection pooling for database
- TODO: Add caching layer
- TODO: Add query optimization

## Troubleshooting

### Port already in use

Change port in `.env` or service configuration.

### Database connection failed

Ensure PostgreSQL is running and connection string is correct in `.env`.

### Build failures

```bash
# Clean build
cargo clean
cargo build
```

## Next Steps

See README.md TODO section for planned features and improvements.
