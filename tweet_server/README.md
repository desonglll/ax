

## Migrations

### Create a .env file

```shell
export DATABASE_URL=postgres://localhost:5432/hello_rocket
```

### Run sqlx-cli

```shell
brew install sqlx-cli
source .env

sqlx database create

# Creates a new file in `migrations/<timestamp>-<name>.sql`.
# Add your database schema changes to this new file.
sqlx migrate add -r <name>

sqlx migrate run

```