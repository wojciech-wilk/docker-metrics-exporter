use actix_web::HttpServer;

use crate::metrics;
use crate::settings::Settings;

const VERSION: &str = "0.1.0";

pub struct App {
    settings: Settings,
}

impl App {
    pub fn new(settings: Settings) -> Self {
        App {
            settings
        }
    }

    pub async fn start(self) -> std::io::Result<()> {
        info!("Starting docker-metrics-exporter {}...", VERSION);

        let server = HttpServer::new(|| actix_web::App::new().service(metrics::service::metrics))
            .workers(2)
            .bind(format!("{}:{}", self.settings.http.address, self.settings.http.port))?
            .run();

        let res = futures::future::try_join(server, self.start_callback()).await;
        match res {
            Ok((r, _)) => Ok(r),
            Err(e) => Err(e)
        }

    }

    async fn start_callback(&self) -> std::io::Result<()> {
        info!("Started docker-metrics-exporter");
        Ok(())
    }
}
