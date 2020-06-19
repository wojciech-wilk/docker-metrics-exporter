#[macro_use]
extern crate log;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Starting docker-metrics-exporter 0.1...");
}
