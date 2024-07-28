use super::ParseFromFile;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct DynamicConfig {
    pub auth: HashMap<String, Auth>,
    pub hooks: HashMap<String, Hook>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Auth {
    Basic(BasicAuth),
    Bearer(BearerAuth),
}

#[derive(Deserialize)]
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct BearerAuth {
    pub token: String,
}

#[derive(Deserialize)]
pub struct Hook {
    pub command: String,
    pub method: Option<String>,
    pub auth: Option<Vec<String>>,
    pub args: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
}

impl ParseFromFile for DynamicConfig {}
