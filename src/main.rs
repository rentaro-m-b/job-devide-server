use actix_web::{App, HttpServer};
use job_devide_server::routes::config;

// server, routing
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
