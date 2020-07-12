use crate::docker::client::DockerClientError;

pub mod service;
mod metric;

impl actix_http::error::ResponseError for DockerClientError {}
