# StellarScope

Production-grade blockchain explorer for the Stellar network.

## Project Structure

```
stellarscope/
├── apps/
│   └── web/                 # Next.js frontend application
├── services/
│   ├── api/                 # Axum REST API service
│   ├── indexer/             # Blockchain data indexer
│   └── workers/             # Background job workers
├── crates/
│   ├── decoder/             # XDR decoding library
│   ├── db/                  # Database layer (SQLx)
│   └── types/               # Shared type definitions
├── infrastructure/
│   ├── docker/              # Docker configurations
│   └── scripts/             # Setup and utility scripts
├── Cargo.toml               # Rust workspace configuration
├── package.json             # Node.js workspace configuration
└── .env.example             # Environment variables template
```

## Tech Stack

- **Backend**: Rust with Axum framework
- **Async Runtime**: Tokio
- **Database**: PostgreSQL with SQLx
- **Frontend**: Next.js with TypeScript and Tailwind CSS
- **Package Manager**: pnpm (frontend), Cargo (Rust)
- **Containerization**: Docker & Docker Compose

## Getting Started

### Prerequisites

- Rust 1.75+
- Node.js 18+
- Docker & Docker Compose
- PostgreSQL 16+ (or use Docker)

### Setup

1. Clone the repository
2. Copy environment variables:
   ```bash
   cp .env.example .env
   ```

3. Start services with Docker Compose:
   ```bash
   docker-compose -f infrastructure/docker/docker-compose.yml up
   ```

4. Or build and run locally:
   ```bash
   # Build Rust services
   cargo build --release
   
   # Install frontend dependencies
   pnpm install
   
   # Run frontend
   pnpm dev
   ```

## Services

### API Service (Port 3000)
REST API for querying blockchain data.

**Endpoints:**
- `GET /health` - Health check
- `GET /tx/:hash` - Get transaction details
- `GET /address/:id` - Get address information

### Indexer Service
Ingests and indexes blockchain data from Stellar network.

**TODO**: Configure data source and indexing parameters

### Workers Service
Background job processing for async tasks.

**TODO**: Define job types and handlers

## Frontend

Next.js application with pages for:
- Homepage with network overview
- Transaction details (`/tx/[hash]`)
- Address details (`/address/[id]`)

## Development

### Build Rust Services
```bash
cargo build
```

### Run Tests
```bash
cargo test
```

### Format Code
```bash
cargo fmt
```

### Lint
```bash
cargo clippy
```

### Frontend Development
```bash
cd apps/web
pnpm dev
```

## Database

PostgreSQL is used for data persistence.

**TODO**: Add migration scripts and schema definitions

## Docker

### Build Images
```bash
docker-compose -f infrastructure/docker/docker-compose.yml build
```

### Start Services
```bash
docker-compose -f infrastructure/docker/docker-compose.yml up -d
```

### View Logs
```bash
docker-compose -f infrastructure/docker/docker-compose.yml logs -f
```

### Stop Services
```bash
docker-compose -f infrastructure/docker/docker-compose.yml down
```

## Configuration

Environment variables are defined in `.env`. See `.env.example` for available options.

## TODO

- [ ] Implement XDR decoding logic
- [ ] Setup database migrations
- [ ] Implement transaction queries
- [ ] Implement address queries
- [ ] Setup blockchain data source connection
- [ ] Implement indexing pipeline
- [ ] Implement background job system
- [ ] Add API authentication
- [ ] Add frontend API integration
- [ ] Add comprehensive error handling
- [ ] Add monitoring and logging
- [ ] Add integration tests
- [ ] Add performance optimizations

## License

MIT
