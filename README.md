---
title: ax system
---

## Apps

- `ax-server`

TODO...

## Requirements

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

## Dev

```sh
git clone git@github.com:desonglll/ax.git && cd ax
cargo install
cd backend/query
export DATABASE_URL=postgres://localhost:5432/hello_rocket
diesel setup && diesel database reset
cd ../..
cargo run
```