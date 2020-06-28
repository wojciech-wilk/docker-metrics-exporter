use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Info {
    #[serde(rename = "Containers")]
    pub containers: u32,

    #[serde(rename = "Images")]
    pub images: u32,

    #[serde(rename = "ServerVersion")]
    pub server_version: String,
}

#[derive(Deserialize, Debug)]
pub struct Version {
    #[serde(rename = "Version")]
    pub version: String,

    #[serde(rename = "BuildTime")]
    pub build_time: String,
}

