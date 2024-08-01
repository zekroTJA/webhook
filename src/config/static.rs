use super::ParseFromFile;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct StaticConfig {
    pub server: Option<Server>,
    pub logging: Option<Logging>,
}

#[derive(Deserialize)]
pub struct Server {
    pub address: Option<String>,
}

#[derive(Deserialize)]
pub struct Logging {
    pub level: Option<String>,
}

impl ParseFromFile for StaticConfig {}
