## Source Code for RESTful API in Sync & Async Rust article

This is the companion code repository for the article [RESTful API in Sync & Async Rust](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md) on my [Rust blog](https://github.com/pretzelhammer/rust-blog).

### Servers

Each server implementation is in its own directory.

Server #1: Diesel + Rocket
- Directory: [diesel-rocket](./diesel-rocket)
- Nickname: DR
- Connection pool: r2d2
- SQL executor: Diesel
- HTTP routing: Rocket
- Compiled with: Rust v1.53 (Nightly)

Server #2: sqlx + actix-web
- Directory: [sqlx-actix-web](./sqlx-actix-web)
- Nickname: SA
- Connection pool: sqlx
- SQL executor: sqlx
- HTTP Routing: actix-web
- Compiled with: Rust v1.53 (Nightly)

Server #3: pg-promise + express.js (single-process)
- Directory: [pgp-express](./pgp-express)
- Nickname: PES
- Connection pool: pg-promise
- SQL executor: pg-promise
- HTTP Routing: express.js
- Interpreted with: node.js v16.0.0
- Mode: single-process

Server #4: pg-promise + express.js (multi-process)
- Directory: [pgp-express-multi](./pgp-express-multi)
- Nickname: PEM
- Connection pool: pg-promise
- SQL executor: pg-promise
- HTTP Routing: express.js
- Interpreted with: node.js v16.0.0
- Mode: multi-process

### Benchmarks

Source code for the benchmarks, and instructions on how to execute them, can be found in the [benchmarks](./benchmarks) directory.
