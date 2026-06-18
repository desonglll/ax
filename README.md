# The Ax Project

This is the official distribution of the Ax Project, a secure micro-blogging and media web application server.

## Overview

The Ax Project provides a secure web server written in the Rust programming language utilizing the Actix-web framework. Data is persisted in a PostgreSQL database and session states are cached via Redis. Project documentation is managed through mdBook.

## Prerequisites

To compile, build, and test this software, the following tools must be installed:
- Cargo and the Rust compiler toolchain (edition 2021).
- The `just` command runner.
- The `sqlx-cli` tool (compiled with Postgres support).
- The `cargo-nextest` test runner.
- The `mdbook` compiler (for documentation generation).

You may install the development tools with:
```bash
cargo install just
cargo install sqlx-cli --no-default-features --features native-tls,postgres
cargo install --locked cargo-nextest
cargo install mdbook
```

---

## Installation & Setup

1. **Start the Infrastructure Containers**:
   Deploy PostgreSQL and Redis instances using Docker Compose:
   ```bash
   docker compose -f compose.yml up -d
   ```

2. **Configure the Environment**:
   Copy the example environment configuration:
   ```bash
   cp .env.example .env
   ```

3. **Initialize the Database**:
   Create the database schema and execute trigger migrations:
   ```bash
   just init-db
   ```

4. **Launch the Server**:
   Start the backend application:
   ```bash
   just run
   ```

---

## Operational Commands

The project root contains a `justfile` providing the following automation recipes:
- `just init-db`: Creates the database and runs migrations.
- `just check`: Compiles the server codebase to check for errors.
- `just test`: Runs all unit and integration tests.
- `just run`: Launches the backend HTTP server.
- `just start`: Launches all services (backend, frontend, recommendation) concurrently.
- `just doc-build`: Builds the documentation book.
- `just doc-serve`: Launches a local server to view documentation.

---

## Documentation

Full manuals, API specifications, database trigger details, and development guidelines are available in the `docs` folder. To build and view the manual locally:
```bash
just doc-build
just doc-serve
```
Then navigate to `http://localhost:3000` in your web browser.
