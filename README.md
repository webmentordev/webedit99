# Under development
```
- First time setup
cargo install sqlx-cli
sqlx database create

- Manage migrations
sqlx migrate add <table-name>
sqlx migrate run
sqlx migrate revert
sqlx database reset

- Info command
sqlx migrate info
```