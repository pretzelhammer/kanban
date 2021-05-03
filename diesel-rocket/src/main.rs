// src/main.rs

// required for rocket macros to work
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;

mod db;
mod logger;
mod models;
mod routes;
mod schema;

type StdErr = Box<dyn std::error::Error>;

#[rocket::get("/")]
fn hello_world() -> &'static str {
    "Hello, world!"
}

fn main() -> Result<(), StdErr> {
    dotenv::dotenv()?;
    #[cfg(debug_assertions)]
    logger::init()?;

    let db = db::Db::connect()?;

    rocket::ignite()
        .manage(db)
        .mount("/", rocket::routes![hello_world])
        .mount("/api", routes::api())
        .launch();

    Ok(())
}
