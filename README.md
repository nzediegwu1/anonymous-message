# Anonymous Message API
An anonymous messaging backend service written in Rust

## Technologies:
- Rust Programming Language
- SQLx
- Postgres
- Axum
- Tokio
- Tower
- Serde
- Thiserror

## Create/drop the database at DATABASE_URL
```
sqlx database create
sqlx database drop
```

## Migrations
If you would like to create reversible migrations with corresponding "up" and "down" scripts, you use the -r flag when creating the first migration:

```
$ sqlx migrate add -r <name>
Creating migrations/20211001154420_<name>.up.sql
Creating migrations/20211001154420_<name>.down.sql
```
After that, you can edit the migration files and run:
```
$ sqlx migrate run
Applied migrations/20211001154420 <name> (32.517835ms)
```
And reverts work as well:

```
$ sqlx migrate revert
Applied 20211001154420/revert <name>
```
*Note*: All the subsequent migrations will be reversible as well.


## Run in dev mode
```
cargo watch -x run
```
## Add a package
```
cargo add <package_name>
```

## Remove a package
```
cargo rm <package_name>