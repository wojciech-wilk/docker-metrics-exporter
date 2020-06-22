use actix_web::{App, HttpServer};
use crate::settings::Settings;
use std::process;

mod settings;
mod metrics;

#[macro_use]
extern crate log;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init();

    let settings = Settings::new();
    match settings {
        Ok(settings) => {
            println!("{:?}", settings);
            info!("Starting docker-metrics-exporter 0.1...");

            HttpServer::new(|| App::new().service(metrics::service::index))
                .bind(format!("{}:{}", settings.http.address, settings.http.port))?
                .run()
                .await
        }
        Err(error) => {
            error!("Configuration error: {:?}", error);
            process::exit(1);
        }
    }
}
