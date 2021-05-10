## sqlx + actix-web

This is the full source code for the sqlx + actix-web server implemented in the article [Kanban Backend in Rust](#) on my [Rust blog](https://github.com/pretzelhammer/rust-blog).

Useful `just` commands:

```bash
just start-dev # spin up DB container & run migrations
just psql # access running DB container
cargo run # run server
just testapi # run a bunch of api requests
```