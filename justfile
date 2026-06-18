# Project Ax Automation Recipes

# Default recipe: list available recipes
default:
	@just --list

# Initialize the PostgreSQL database and run migrations
init-db:
	sqlx database create
	sqlx migrate run

# Compile check the backend server
check:
	cargo check --manifest-path tweet_server/Cargo.toml

# Run the unit and integration tests using cargo-nextest
test:
	cargo nextest run --manifest-path tweet_server/Cargo.toml

# Run the backend server locally
run:
	cargo run --manifest-path tweet_server/Cargo.toml

# Build the mdBook documentation
doc-build:
	mdbook build docs

# Serve the mdBook documentation locally
doc-serve:
	mdbook serve docs

# Install frontend dependencies (v1.1)
fe-install:
	cd frontend/v1.1 && bun install

# Start frontend development server (v1.1)
fe-dev:
	cd frontend/v1.1 && bun run dev

# Run frontend typescript typecheck (v1.1)
fe-check:
	cd frontend/v1.1 && bun run typecheck

# Build frontend production bundle (v1.1)
fe-build:
	cd frontend/v1.1 && bun run build

# Install model server python dependencies
rec-install:
	pip install -r model_server/requirements.txt

# Run the model server locally
rec-run:
	uvicorn model_server.main:app --host 127.0.0.1 --port 8001 --reload

