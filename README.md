# AX

## start

### create database container

```sh
docker-compose -f compose.yml up -d

cp .env.example .env

chmod 777 ./scripts/init_db.sh

./scripts/init_db.sh

cargo run --bin tweet_server
```

