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
