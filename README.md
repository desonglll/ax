---
title: ax system
---

## !!! This README is not updated for sqlx.
## !!! This README is not updated for distribute system.

## Apps

An Chat And Post System based on Rust and Postgres.

![AX](./design/DALL·E%202024-08-13%2017.25.06%20-%20A%20cute-style%20illustration%20featuring%20Rust%20as%20the%20dominant%20element.%20The%20Rust%20programming%20language%20logo,%20a%20friendly,%20cartoonish,%20and%20slightly%20simplified%20.webp) 

The previous project was [Crab Rocket](https://github.com/desonglll/crab_rocket)


- [!!! This README is not updated for sqlx.](#-this-readme-is-not-updated-for-sqlx)
- [!!! This README is not updated for distribute system.](#-this-readme-is-not-updated-for-distribute-system)
- [Apps](#apps)
- [🧩 Project Dependencies](#-project-dependencies)
- [Demo](#demo)
- [⚙️ Requirements](#️-requirements)
  - [Rust](#rust)
  - [Postgres](#postgres)
  - [Redis](#redis)
  - [diesel](#diesel)
- [🎃 Quick Start](#-quick-start)
- [Migrations](#migrations)
  - [Create a .env file](#create-a-env-file)
  - [Run sqlx-cli](#run-sqlx-cli)
  - [Run server](#run-server)
- [🔧 Compile Release Version](#-compile-release-version)
  - [Installation](#installation)
  - [🚀 Running the Binary](#-running-the-binary)
  - [Reset Database](#reset-database)
  - [Run](#run)
- [Design](#design)
  - [Technical](#technical)


## 🧩 Project Dependencies

- Rust
- Postgresql16
- Redis
- Actix-Web

## Demo

![Post Page](./design/Demo.png) 

## ⚙️ Requirements
### Rust

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Postgres

```sh
brew install postgresql@16
```

### Redis

```sh
brew install redis
```

### diesel

```sh
cargo install diesel_cli --no-default-features --features postgres
```

## 🎃 Quick Start

```sh
git clone git@github.com:desonglll/ax.git && cd ax
cargo install
cd backend/query
export DATABASE_URL=postgres://localhost:5432/hello_rocket
diesel setup && diesel database reset
cd ../..
cargo run
```
<!-- ## 🥰 Development -->
## Migrations
<!-- https://crates.io/crates/sqlx-cli -->
### Create a .env file

```shell
export DATABASE_URL=postgres://localhost:5432/hello_rocket
```

### Run sqlx-cli

```shell
brew install sqlx-cli
source .env

cd tweet_server

sqlx database create
sqlx database drop

# Creates a new file in `migrations/<timestamp>-<name>.sql`.
# Add your database schema changes to this new file.
sqlx migrate add -r <name>

sqlx migrate run

sqlx migrate info --source ../relative/migrations

sqlx migrate revert
```

### Run server
```shell
# Run the server
cd ax
cargo run --bin tweet_service
```

<!-- ### Database Migration

```shell
# Install Diesel CLI
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/download/v2.2.1/diesel_cli-installer.sh | sh

cd ./backend/query

# Setup Diesel
diesel setup

# Redo and run migrations
# diesel migration redo
diesel migration run

# Run the server
cargo run
```

!!! Run `diesel database reset` before run `cargo test` . -->

## 🔧 Compile Release Version

```shell
cargo build --release
```

### Installation

```shell
cargo install --path .
```

### 🚀 Running the Binary

Set the environment variable:

```shell
export DATABASE_URL=postgres://@localhost/hello_rocket
```

Alternatively, update the `.env` file in the project root.

### Reset Database

```shell
diesel database reset
```

### Run

```shell
ax
```
## Design

### Technical

![Technical](./design/Technic.png) 