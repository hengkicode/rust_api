use actix_web::{App, HttpServer, web};
use crate::config::{init_config, init_db};
use crate::routes::api::configure_routes;

mod config;
mod controllers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_config();
    let pool = init_db().await;

    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
