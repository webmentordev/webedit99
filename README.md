# Under development
```
- First time setup
cargo install sqlx-cli
cargo sqlx database create

- Manage migrations
cargo sqlx migrate add <table-name>
cargo sqlx migrate run
cargo sqlx migrate revert
sqlx database reset

- Info command
cargo sqlx migrate info
```