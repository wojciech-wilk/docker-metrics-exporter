use actix_web::{HttpServer, middleware};
use actix_web::web::Data;

use crate::metrics;
use crate::settings::build_info::BUILD_INFO;
use crate::settings::Settings;

pub struct App {
    settings: Settings,
}

impl App {
    pub fn new(settings: Settings) -> Self {
        App {
            settings
        }
    }

    pub async fn start(&self) -> std::io::Result<()> {
        info!("Starting docker-metrics-exporter {}({})...", &BUILD_INFO.version, &BUILD_INFO.commit_hash);

        let settings_data = Data::new(self.settings.clone());
        let server = HttpServer::new(move || actix_web::App::new()
            .wrap(middleware::Logger::default())
            .service(metrics::service::metrics)
            .app_data(settings_data.clone()))
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
