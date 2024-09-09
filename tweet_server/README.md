

## Migrations
<!-- https://crates.io/crates/sqlx-cli -->
### Create a .env file

```shell
export DATABASE_URL=postgres://localhost:5432/hello_rocket
```

```shell
SELECT pg_terminate_backend(pid)
FROM pg_stat_activity
WHERE datname = 'hello_rocket';
```

### Run sqlx-cli

```shell
brew install sqlx-cli
source .env

sqlx database create
sqlx database drop

# Creates a new file in `migrations/<timestamp>-<name>.sql`.
# Add your database schema changes to this new file.
sqlx migrate add -r <name>

sqlx migrate run

sqlx migrate info --source ../relative/migrations

sqlx migrate revert
```
