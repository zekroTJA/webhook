use super::ParseFromFile;
use anyhow::Result;
use serde::de::DeserializeOwned;
use std::{path::PathBuf, time::SystemTime};

pub struct Loader<T> {
    value: T,
    path: PathBuf,
    last_modified: SystemTime,
}

impl<T> Loader<T>
where
    T: ParseFromFile,
    T: DeserializeOwned,
{
    pub fn new<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path: PathBuf = path.into();
        let value = T::from_file(&path)?;
        let last_modified = path.metadata()?.modified()?;
        Ok(Self {
            path,
            value,
            last_modified,
        })
    }

    pub fn get(&mut self) -> Result<&T> {
        let last_modified = self.path.metadata()?.modified()?;
        if last_modified <= self.last_modified {
            return Ok(&self.value);
        }

        self.value = T::from_file(&self.path)?;
        self.last_modified = last_modified;

        Ok(&self.value)
    }
}
