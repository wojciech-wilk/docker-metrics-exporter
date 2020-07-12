use actix_web::get;
use actix_web::web::Data;

use crate::docker::client::{DockerClient, DockerClientError};
use crate::metrics::metric::{Label, Metric};
use crate::settings::build_info::BUILD_INFO;
use crate::settings::Settings;

#[get("/metrics")]
pub async fn metrics(settings: Data<Settings>) -> Result<String, actix_web::Error> {
    let client = DockerClient::new(&settings.docker);

    let docker_version = get_docker_version(&client).await?;

    Ok(
        vec![docker_version, get_info()]
            .iter()
            .map(|metric| metric.into_prometheus_string())
            .collect::<Vec<String>>()
            .join("\n")
    )
}

fn get_info() -> Metric {
    Metric {
        name: "version_info".to_string(),
        value: "1".to_string(),
        labels: vec![
            Label { name: "commit".to_string(), value: BUILD_INFO.commit_hash.to_string() },
            Label { name: "version".to_string(), value: BUILD_INFO.version.to_string() }
        ],
    }
}

async fn get_docker_version(client: &DockerClient) -> Result<Metric, DockerClientError> {
    let version = client.get_version().await?;

    Ok(Metric {
        name: "docker_version".to_string(),
        value: version.version,
        labels: vec![],
    })
}
