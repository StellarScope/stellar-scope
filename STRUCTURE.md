# StellarScope Monorepo Structure

## Complete Directory Layout

```
stellarscope/
│
├── apps/
│   └── web/                          # Next.js Frontend Application
│       ├── src/
│       │   └── app/
│       │       ├── page.tsx           # Homepage
│       │       ├── layout.tsx         # Root layout
│       │       ├── globals.css        # Global styles
│       │       ├── tx/
│       │       │   └── [hash]/
│       │       │       └── page.tsx   # Transaction detail page
│       │       └── address/
│       │           └── [id]/
│       │               └── page.tsx   # Address detail page
│       ├── package.json              # Frontend dependencies
│       ├── tsconfig.json             # TypeScript config
│       ├── next.config.js            # Next.js config
│       ├── tailwind.config.ts        # Tailwind CSS config
│       ├── postcss.config.js         # PostCSS config
│       ├── .eslintrc.json            # ESLint config
│       └── .gitignore
│
├── services/                         # Rust Microservices
│   ├── api/                          # REST API Service (Axum)
│   │   ├── src/
│   │   │   ├── main.rs               # Server setup & routes
│   │   │   ├── handlers/
│   │   │   │   └── mod.rs            # Endpoint handlers
│   │   │   ├── routes/
│   │   │   │   └── mod.rs            # Route definitions
│   │   │   └── db/
│   │   │       └── mod.rs            # Database helpers
│   │   └── Cargo.toml
│   │
│   ├── indexer/                      # Blockchain Indexer Service
│   │   ├── src/
│   │   │   ├── main.rs               # Service entry point
│   │   │   ├── ingestion/
│   │   │   │   └── mod.rs            # Data ingestion
│   │   │   ├── decoding/
│   │   │   │   └── mod.rs            # XDR decoding
│   │   │   └── pipeline/
│   │   │       └── mod.rs            # Pipeline orchestration
│   │   └── Cargo.toml
│   │
│   └── workers/                      # Background Workers Service
│       ├── src/
│       │   ├── main.rs               # Service entry point
│       │   └── jobs/
│       │       └── mod.rs            # Job definitions
│       └── Cargo.toml
│
├── crates/                           # Shared Rust Libraries
│   ├── types/                        # Shared Type Definitions
│   │   ├── src/
│   │   │   └── lib.rs                # Transaction, Event, Address types
│   │   └── Cargo.toml
│   │
│   ├── db/                           # Database Layer (SQLx)
│   │   ├── src/
│   │   │   └── lib.rs                # Connection pool, models
│   │   └── Cargo.toml
│   │
│   └── decoder/                      # XDR Decoder Library
│       ├── src/
│       │   └── lib.rs                # decode_xdr(), parse_events()
│       └── Cargo.toml
│
├── infrastructure/                   # Deployment & Infrastructure
│   ├── docker/
│   │   ├── Dockerfile.api            # API service container
│   │   ├── Dockerfile.indexer        # Indexer service container
│   │   ├── docker-compose.yml        # Multi-service orchestration
│   │   └── .dockerignore
│   │
│   └── scripts/
│       └── setup.sh                  # Setup automation script
│
├── Cargo.toml                        # Rust workspace configuration
├── Cargo.lock                        # Rust dependency lock file
├── package.json                      # Node.js workspace configuration
├── .env.example                      # Environment variables template
├── .gitignore                        # Git ignore rules
│
├── README.md                         # Project overview
├── DEVELOPMENT.md                    # Development guide
├── STRUCTURE.md                      # This file
└── INITIALIZATION_CHECKLIST.md       # Initialization status
```

## File Statistics

### Rust Code
- **Services**: 3 binary crates (api, indexer, workers)
- **Libraries**: 3 library crates (types, db, decoder)
- **Source files**: 13 Rust files
- **Configuration files**: 6 Cargo.toml files

### Frontend Code
- **Framework**: Next.js with TypeScript
- **Pages**: 4 (home, tx/[hash], address/[id], layout)
- **Styling**: Tailwind CSS
- **Configuration files**: 6 (tsconfig, tailwind, postcss, eslint, next.config, package.json)

### Infrastructure
- **Docker**: 2 Dockerfiles + 1 docker-compose.yml
- **Scripts**: 1 setup script

### Documentation
- **README.md**: Project overview and setup instructions
- **DEVELOPMENT.md**: Development guide and quick start
- **STRUCTURE.md**: This file - directory structure
- **INITIALIZATION_CHECKLIST.md**: Initialization status and next steps

## Key Features

### Rust Workspace
- Unified dependency management via workspace
- Shared crates for types, database, and decoding
- Three independent services with clear responsibilities
- Tokio async runtime throughout
- SQLx for type-safe database queries

### Frontend
- Next.js App Router for modern routing
- TypeScript for type safety
- Tailwind CSS for styling
- Responsive design with placeholder content

### Docker
- Multi-stage builds for optimized images
- Docker Compose for local development
- PostgreSQL database service
- Environment-based configuration

### Configuration
- Workspace-level dependency management
- Shared environment variables
- Development and production ready
- Extensible architecture

## Module Responsibilities

### API Service
- REST endpoints for querying blockchain data
- Health checks
- Request/response handling
- Database integration

### Indexer Service
- Ingests blockchain data
- Decodes XDR transactions
- Processes events
- Stores data in database

### Workers Service
- Background job processing
- Async task execution
- Job queue management
- Retry logic

### Types Crate
- Shared data structures
- Serialization/deserialization
- Type safety across services

### DB Crate
- Connection pool management
- Database models
- Query builders
- Migration support

### Decoder Crate
- XDR decoding logic
- Event parsing
- Data transformation

## Development Workflow

1. **Setup**: Copy .env.example to .env
2. **Build**: `cargo build` for Rust, `pnpm install` for frontend
3. **Run**: Use Docker Compose or run services individually
4. **Develop**: Edit code in respective service directories
5. **Test**: `cargo test` for Rust, `pnpm test` for frontend
6. **Deploy**: Use Docker images from infrastructure/docker

## Compilation Status

✅ All Rust code compiles successfully
✅ All configuration files are valid
✅ All dependencies are resolved
✅ Ready for development

## Next Steps

1. Implement business logic in placeholder functions
2. Setup database migrations
3. Connect frontend to API
4. Add authentication
5. Add comprehensive error handling
6. Add monitoring and logging
7. Setup CI/CD pipeline
8. Deploy to production

---

For detailed development instructions, see DEVELOPMENT.md
For initialization status, see INITIALIZATION_CHECKLIST.md
