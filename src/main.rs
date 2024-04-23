extern crate dotenv;

use actix_web::{App, HttpServer};
use job_devide_server::routes::config;
use dotenv::dotenv;
use std::env;

// server, routing
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let http_server_workers_string = &env::var("HTTP_SERVER_WORKERS").unwrap_or("5".to_string());
    let http_server_workers: usize = http_server_workers_string.parse().unwrap();

    HttpServer::new(|| {
        App::new()
            .configure(config)
    })
    .workers(http_server_workers)
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
