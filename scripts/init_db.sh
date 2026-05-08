#!/usr/bin/env bash
set -euo pipefail

if ! command -v sqlx &>/dev/null; then
    echo "Error: sqlx is not installed." >&2
    exit 1
fi

DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=ax}"
DB_PORT="${POSTGRES_PORT:=5432}"

if [[ -z "${SKIP_DOCKER:-}" ]]; then
    docker rm -f postgres_ax 2>/dev/null || true
    docker run \
        --name postgres_ax \
        -e POSTGRES_USER="${DB_USER}" \
        -e POSTGRES_PASSWORD="${DB_PASSWORD}" \
        -e POSTGRES_DB="${DB_NAME}" \
        -p "${DB_PORT}":5432 \
        -d postgres \
        postgres -N 1000
    CONTAINER_NAME="postgres_ax"
else
    CONTAINER_NAME="${POSTGRES_CONTAINER:=postgres_ax}"
fi

until docker exec "${CONTAINER_NAME}" pg_isready -U "${DB_USER}" -d "${DB_NAME}" &>/dev/null; do
    echo "Postgres is still unavailable - sleeping" >&2
    sleep 1
done

echo "Postgres is up and running on port ${DB_PORT}!" >&2

cd "$(dirname "$0")/.."

export DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}"
sqlx database create
sqlx migrate run

echo "Postgres has been migrated, ready to go!" >&2

docker rm -f ax-redis 2>/dev/null || true
docker run --name ax-redis -p 6379:6379 -d redis:8
