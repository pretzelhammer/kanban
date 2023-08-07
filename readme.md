## Source Code for RESTful API in Sync & Async Rust article

This is the companion code repository for the article [RESTful API in Sync & Async Rust](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md) on my [Rust blog](https://github.com/pretzelhammer/rust-blog).

### Servers

Each server implementation is in its own directory.


| Server                                                         | Nickname | Connection pool | SQL executor | HTTP routing |    Compiled with     | Interpreted with |      Mode      |
|:---------------------------------------------------------------|:--------:|:---------------:|:------------:|:------------:|:--------------------:|:----------------:|:--------------:|
| [Diesel + Rocket](./diesel-rocket)                             |    DR    |      r2d2       |    Diesel    |    Rocket    | Rust v1.53 (Nightly) |                  |                |
| [sqlx + actix-web](./sqlx-actix-web)                           |    SA    |      sqlx       |     sqlx     |  actix-web   | Rust v1.53 (Nightly) |                  |                |
| [pg-promise + express.js (single-process)](./pgp-express)      |   PES    |   pg-promise    |  pg-promise  |  express.js  |                      | node.js v16.0.0  | single-process |
| [pg-promise + express.js (multi-process)](./pgp-express-multi) |   PEM    |   pg-promise    |  pg-promise  |  express.js  |                      | node.js v16.0.0  | multi-process  |

### Benchmarks

Source code for the benchmarks, and instructions on how to execute them, can be found in the [benchmarks](./benchmarks) directory.
