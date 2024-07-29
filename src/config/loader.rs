use super::ParseFromFile;
use anyhow::Result;
use serde::de::DeserializeOwned;
use std::{path::PathBuf, sync::RwLock, time::SystemTime};
use tracing::debug;

struct State<T> {
    value: T,
    last_modified: SystemTime,
}

pub struct Loader<T> {
    state: RwLock<State<T>>,
    path: PathBuf,
}

impl<T> Loader<T>
where
    T: ParseFromFile,
    T: DeserializeOwned,
    T: Clone,
{
    pub fn new<P: Into<PathBuf>>(path: P) -> Result<Self> {
        let path: PathBuf = path.into();
        let value = T::from_file(&path)?;
        let last_modified = path.metadata()?.modified()?;

        let state = RwLock::new(State {
            value,
            last_modified,
        });

        Ok(Self { path, state })
    }

    pub fn get(&self) -> Result<T> {
        let last_modified = self.path.metadata()?.modified()?;

        {
            let state = self.state.read().expect("mtx read unlock");
            if last_modified <= state.last_modified {
                debug!("Returning dynamic config from cached state");
                return Ok(state.value.clone());
            }
        }

        debug!("Reloading dynamic config from file");
        let mut state = self.state.write().expect("mtx write unlock");

        state.value = T::from_file(&self.path)?;
        state.last_modified = last_modified;

        Ok(state.value.clone())
    }

    pub fn get_cached(&self) -> T {
        let state = self.state.read().expect("mtx read unlock");
        state.value.clone()
    }
}
