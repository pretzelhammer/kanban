## Diesel + Rocket

This is the full source code for the Diesel + Rocket server implemented in the article [Kanban Backend in Rust](#) on my [Rust blog](https://github.com/pretzelhammer/rust-blog).

Useful `just` commands:

```bash
just set-nightly # use nightly Rust compiler, needed for Rocket
just start-dev # spin up DB container & run migrations
just setup-diesel # for first time project setup
just psql # access running DB container
cargo run # run server
just testapi # run a bunch of api requests
```