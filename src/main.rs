use actix_web::{web, App, HttpServer};
use crate::config::{init_config, init_db};
use crate::routes::api::configure_routes;
use log::info;
use std::env;

mod config;
mod controllers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set default log level jika tidak diset dari luar
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    dotenv::dotenv().ok();
    env_logger::init();

    init_config();
    let pool = init_db().await;

    info!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
