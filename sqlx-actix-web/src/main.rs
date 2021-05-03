// src/main.rs
mod db;
mod logger;
mod models;
mod routes;

type StdErr = Box<dyn std::error::Error>;

#[actix_web::get("/")]
async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> Result<(), StdErr> {
    dotenv::dotenv().ok();
    #[cfg(debug_assertions)]
    logger::init()?;

    let db = db::Db::connect().await?;

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .data(db.clone())
            .service(hello_world)
            .service(routes::api())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await?;

    Ok(())
}
