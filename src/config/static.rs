use super::ParseFromFile;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct StaticConfig {
    pub server: Option<Server>,
}

#[derive(Deserialize)]
pub struct Server {
    pub address: Option<String>,
}

impl ParseFromFile for StaticConfig {}
