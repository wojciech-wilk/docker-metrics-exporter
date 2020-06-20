use actix_web::{App, HttpServer};

mod metrics;

#[macro_use]
extern crate log;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Starting docker-metrics-exporter 0.1...");

    HttpServer::new(|| App::new().service(metrics::index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
