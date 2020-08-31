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
            .into_iter()
            .map(|metric| metric.into_prometheus_string())
            .collect::<Vec<String>>()
            .join("\n")
    )
}

fn get_info() -> Metric {
    let labels = Some(vec![
        Label::new("commit".to_string(), BUILD_INFO.commit_hash.to_string()),
        Label::new("version".to_string(), BUILD_INFO.version.to_string()),
    ]);

    Metric::new("version_info".to_string(), "1".to_string(), labels)
}

async fn get_docker_version(client: &DockerClient) -> Result<Metric, DockerClientError> {
    let version = client.get_version().await?;

    let labels = Some(vec![
        Label::new("version".to_string(), version.version),
    ]);

    Ok(Metric::new("docker_info".to_string(), "1".to_string(), labels))
}
