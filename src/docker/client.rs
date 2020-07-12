use std::fmt;
use std::path::Path;

use async_trait::async_trait;
use futures::TryStreamExt;
use hyper::{Body, Client, Response};
use hyper::client::HttpConnector;
use hyperlocal::{UnixClientExt, UnixConnector, Uri};
use serde::de::DeserializeOwned;

use crate::docker::types::{Info, Version};
use crate::settings::DockerClientSettings;

#[derive(Clone)]
pub struct DockerClient {
    client: Box<dyn Get>,
}

impl DockerClient {
    pub fn new(config: &DockerClientSettings) -> Self {
        let docker_url_split: Vec<&str> = config.url.split("://").collect();
        let schema = docker_url_split[0].to_lowercase();

        let client: Box<dyn Get>;
        if schema.as_str() == "unix" {
            client = Box::new(UnixClient::new(docker_url_split[1]));
        } else {
            client = Box::new(TcpClient::new(&config.url));
        }

        DockerClient {
            client,
        }
    }

    pub async fn get_info(&self) -> Result<Info, DockerClientError> {
        Ok(self.get("/info").await?)
    }

    pub async fn get_version(&self) -> Result<Version, DockerClientError> {
        Ok(self.get("/version").await?)
    }
}

#[async_trait]
trait Get: CloneGet {
    async fn get(&self, path: &str) -> Result<Response<Body>, DockerClientError>;
}

#[derive(Clone)]
struct TcpClient {
    client: Client<HttpConnector, Body>,
    base_url: String,
}

#[derive(Clone)]
struct UnixClient {
    client: Client<UnixConnector, Body>,
    socket_path: String,
}

impl UnixClient {
    pub fn new(socket_path: &str) -> Self {
        UnixClient {
            client: Client::unix(),
            socket_path: socket_path.to_string(),
        }
    }
}

impl TcpClient {
    pub fn new(base_url: &str) -> Self {
        TcpClient {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }
}

#[async_trait]
impl Get for UnixClient {
    async fn get(&self, path: &str) -> Result<Response<Body>, DockerClientError> {
        let uri = Uri::new(Path::new(&self.socket_path), path).into();

        Ok(self.client.get(uri).await?)
    }
}

#[async_trait]
impl Get for TcpClient {
    async fn get(&self, path: &str) -> Result<Response<Body>, DockerClientError> {
        let uri_string = format!("{}{}", &self.base_url, path);

        Ok(self.client.get(uri_string.parse().unwrap()).await?)
    }
}

impl DockerClient {
    async fn get<Type>(&self, path: &str) -> Result<Type, DockerClientError>
        where Type: DeserializeOwned {
        debug!("GET {}", &path);
        let result = self.client.get(path).await?;

        let bytes = result.into_body()
            .try_fold(Vec::default(), |mut buf, bytes| async {
                buf.extend(bytes);
                Ok(buf)
            })
            .await?;

        let body_string = String::from_utf8(bytes)?;
        trace!("GET {} response: '{}'", &path, &body_string);

        Ok(serde_json::from_str(body_string.as_str())?)
    }
}

trait CloneGet {
    fn clone_get(&self) -> Box<dyn Get>;
}

impl<T> CloneGet for T
    where T: Get + Clone + 'static {
    fn clone_get(&self) -> Box<dyn Get> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Get> {
    fn clone(&self) -> Self {
        self.clone_get()
    }
}

#[derive(Debug, Clone)]
pub struct DockerClientError;

impl std::error::Error for DockerClientError {}

impl fmt::Display for DockerClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Docker client error")
    }
}

impl From<hyper::error::Error> for DockerClientError {
    fn from(_: hyper::error::Error) -> Self {
        DockerClientError {}
    }
}

impl From<serde_json::error::Error> for DockerClientError {
    fn from(_: serde_json::error::Error) -> Self {
        DockerClientError {}
    }
}

impl From<std::string::FromUtf8Error> for DockerClientError {
    fn from(_: std::string::FromUtf8Error) -> Self {
        DockerClientError {}
    }
}

