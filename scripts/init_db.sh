#!/usr/bin/env bash
set -euo pipefail

if ! command -v sqlx &>/dev/null; then
    echo "Error: sqlx is not installed." >&2
    exit 1
fi

sqlx database create
sqlx migrate run

echo "Postgres has been migrated, ready to go!" >&2
