use crate::settings::Settings;
use std::{process};

mod app;
mod docker;
mod metrics;
mod settings;

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
            let app: app::App = app::App::new(settings);
            app.start().await
        }
        Err(error) => {
            error!("Configuration error: {:?}", error);
            process::exit(1);
        }
    }
}
