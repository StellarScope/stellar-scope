# Contributing to StellarScope

Thank you for your interest in contributing to StellarScope! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

Be respectful, inclusive, and professional in all interactions. We're committed to providing a welcoming environment for all contributors.

## Getting Started

### Prerequisites

- Rust 1.75+
- Node.js 18+
- Docker & Docker Compose
- PostgreSQL 16+ (or use Docker)
- Git

### Development Setup

1. Fork the repository and clone your fork:
   ```bash
   git clone https://github.com/your-username/stellarscope.git
   cd stellarscope
   ```

2. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/original-repo/stellarscope.git
   ```

3. Set up environment:
   ```bash
   cp .env.example .env
   ```

4. Start PostgreSQL:
   ```bash
   docker run -d \
     --name stellarscope-postgres \
     -e POSTGRES_USER=stellarscope \
     -e POSTGRES_PASSWORD=password \
     -e POSTGRES_DB=stellarscope \
     -p 5432:5432 \
     postgres:16-alpine
   ```

5. Build and run services:
   ```bash
   cargo build
   cargo run -p api
   ```

6. In another terminal, set up frontend:
   ```bash
   cd apps/web
   pnpm install
   pnpm dev
   ```

## Making Changes

### Branch Naming

Use descriptive branch names:
- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation updates
- `refactor/description` - Code refactoring

Example: `feature/add-transaction-filtering`

### Commit Messages

Write clear, concise commit messages:
- Use imperative mood ("Add feature" not "Added feature")
- Keep first line under 50 characters
- Add detailed description if needed

Example:
```
Add transaction filtering by date range

- Implement date range query in API handler
- Add validation for date parameters
- Update transaction endpoint documentation
```

### Code Style

#### Rust

- Follow Rust conventions and idioms
- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes with no warnings
- Add doc comments for public APIs

```bash
cargo fmt
cargo clippy
```

#### TypeScript/JavaScript

- Follow ESLint configuration in `apps/web/.eslintrc.json`
- Use TypeScript for type safety
- Format with Prettier (configured in Next.js)

```bash
cd apps/web
pnpm lint
```

### Testing

#### Rust Tests

```bash
# Run all tests
cargo test

# Run tests for specific package
cargo test -p api

# Run with output
cargo test -- --nocapture
```

#### Frontend Tests

```bash
cd apps/web
pnpm test
```

Write tests for new functionality:
- Unit tests in the same file or `tests/` module
- Integration tests in `tests/` directory
- Aim for meaningful test coverage

### Documentation

- Update README.md if adding new features
- Add doc comments to public functions and types
- Update DEVELOPMENT.md if changing development workflow
- Include examples for complex functionality

## Submitting Changes

### Pull Request Process

1. Keep your branch up to date:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. Push to your fork:
   ```bash
   git push origin your-branch-name
   ```

3. Create a Pull Request on GitHub with:
   - Clear title describing the change
   - Description of what changed and why
   - Reference to related issues (e.g., "Fixes #123")
   - Screenshots for UI changes

4. Ensure all checks pass:
   - Tests pass
   - Code formatting is correct
   - No linting errors
   - Documentation is updated

### PR Guidelines

- Keep PRs focused on a single concern
- Avoid large refactorings mixed with feature changes
- Respond to review feedback promptly
- Keep commits clean and logical

## Project Structure

Familiarize yourself with the project layout:

```
stellarscope/
├── apps/web/              # Next.js frontend
├── services/
│   ├── api/               # REST API (Axum)
│   ├── indexer/           # Blockchain indexer
│   └── workers/           # Background jobs
├── crates/
│   ├── types/             # Shared types
│   ├── db/                # Database layer
│   └── decoder/           # XDR decoding
└── infrastructure/        # Docker & scripts
```

## Common Contribution Types

### Adding an API Endpoint

1. Define handler in `services/api/src/handlers/mod.rs`
2. Add route in `services/api/src/main.rs`
3. Add tests for the endpoint
4. Update API documentation

### Adding a Database Model

1. Define struct in `crates/types/src/lib.rs`
2. Add queries in `crates/db/src/lib.rs`
3. Create database migration
4. Add tests

### Adding a Frontend Page

1. Create file in `apps/web/src/app/` following Next.js conventions
2. Add navigation links if needed
3. Add tests for the page
4. Update documentation

### Fixing a Bug

1. Create a test that reproduces the bug
2. Fix the bug
3. Verify the test passes
4. Add regression test if needed

## Reporting Issues

### Bug Reports

Include:
- Clear description of the bug
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment details (OS, Rust version, etc.)
- Error messages or logs

### Feature Requests

Include:
- Clear description of the feature
- Use case and motivation
- Proposed implementation (if applicable)
- Examples or mockups

## Questions?

- Check existing issues and discussions
- Review DEVELOPMENT.md for setup help
- Ask in pull request comments
- Open a discussion for general questions

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Recognition

Contributors will be recognized in the project. Thank you for helping make StellarScope better!
