# AX

## for developers

### dependencies

``` sh
cargo install just
# cargo install sqlx-cli
cargo install sqlx-cli --no-default-features --features native-tls,postgres
cargo install --locked cargo-nextest
cargo install mdbook
```

### test

``` sh
cargo nextest run
```

## start

### create database container

```sh
docker-compose -f compose.yml up -d

cp .env.example .env

chmod 777 ./scripts/init_db.sh

./scripts/init_db.sh

cargo run --bin tweet_server
```

