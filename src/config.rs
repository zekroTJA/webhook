use anyhow::Result;
use figment::{
    providers::{Format, Json, Toml, Yaml},
    Figment,
};
use serde::{de::DeserializeOwned, Deserialize};
use std::{collections::HashMap, ops::Deref, path::Path};

#[derive(Deserialize)]
pub struct Config {
    pub server: Option<Server>,
    pub auth: HashMap<String, Auth>,
    pub hooks: HashMap<String, Hook>,
}

#[derive(Deserialize)]
pub struct Server {
    pub address: Option<String>,
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

impl ParseFromFile for Config {}

pub trait ParseFromFile {
    fn from_file<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
        Self: DeserializeOwned,
    {
        let ext = path.as_ref().extension().unwrap_or_default();

        let mut figment = Figment::new();

        figment = match ext.to_string_lossy().deref() {
            "yml" | "yaml" => figment.merge(Yaml::file(path)),
            "toml" => figment.merge(Toml::file(path)),
            "json" => figment.merge(Json::file(path)),
            _ => anyhow::bail!("invalid config file type"),
        };

        Ok(figment.extract()?)
    }
}
