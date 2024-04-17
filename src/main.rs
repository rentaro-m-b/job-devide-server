extern crate dotenv;

use actix_web::{App, HttpServer};
use job_devide_server::routes::config;
use dotenv::dotenv;

// server, routing
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .configure(config)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
