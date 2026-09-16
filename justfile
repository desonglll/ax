# Project Ax — task runner. Run `just` to list recipes.

set dotenv-load

default:
    @just --list

# Start PostgreSQL (the only external service)
db:
    docker compose up -d

# Stop PostgreSQL
db-down:
    docker compose down

# Run the backend (migrations are applied automatically on startup)
run:
    cargo run --manifest-path tweet_server/Cargo.toml

# Type-check the backend without a database
check:
    SQLX_OFFLINE=true cargo check --manifest-path tweet_server/Cargo.toml --all-targets

# Lint the backend
clippy:
    SQLX_OFFLINE=true cargo clippy --manifest-path tweet_server/Cargo.toml --all-targets -- -D warnings

# Run backend unit tests (no database needed)
test:
    SQLX_OFFLINE=true cargo test --manifest-path tweet_server/Cargo.toml

# Regenerate the SQLx offline query cache after changing SQL (needs DATABASE_URL)
sqlx-prepare:
    cargo sqlx prepare --workspace -- --all-targets

# Create a new migration file
migrate-add name:
    sqlx migrate add {{name}}

# Install frontend dependencies
fe-install:
    cd frontend && bun install

# Frontend dev server (proxies /api to the backend port in .server-port)
fe-dev:
    cd frontend && bun run dev

# Frontend type-check
fe-check:
    cd frontend && bun run typecheck

# Frontend production build
fe-build:
    cd frontend && bun run build

# Everything CI runs: backend check + clippy + tests, frontend typecheck + build
ci: check clippy test fe-check fe-build

# Start backend (random port) and frontend dev server together
start:
    #!/usr/bin/env bash
    set -euo pipefail
    rm -f .server-port
    trap 'kill 0' INT TERM EXIT
    PORT=0 PORT_FILE=.server-port cargo run --manifest-path tweet_server/Cargo.toml &
    backend=$!
    for _ in $(seq 1 600); do
        test -s .server-port && break
        kill -0 "$backend" 2>/dev/null || { echo "backend exited before publishing its port" >&2; exit 1; }
        sleep 0.1
    done
    test -s .server-port || { echo "backend did not publish its port" >&2; exit 1; }
    echo "backend: http://127.0.0.1:$(cat .server-port)"
    (cd frontend && bun run dev) &
    wait

# Build the documentation book
doc-build:
    mdbook build docs

# Serve the documentation book locally
doc-serve:
    mdbook serve docs
