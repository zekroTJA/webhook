use anyhow::Result;
use figment::{
    providers::{Format, Json, Toml, Yaml},
    Figment,
};
use serde::de::DeserializeOwned;
use std::{ops::Deref, path::Path};

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
