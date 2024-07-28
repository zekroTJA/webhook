use serde::Deserialize;

use super::ParseFromFile;

#[derive(Deserialize)]
pub struct StaticConfig {
    pub server: Option<Server>,
}

#[derive(Deserialize)]
pub struct Server {
    pub address: Option<String>,
}

impl ParseFromFile for StaticConfig {}
