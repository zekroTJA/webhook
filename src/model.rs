use std::process::{ExitStatus, Output};

use serde::Serialize;

#[derive(Serialize, Default)]
pub struct CommandResult {
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub status_code: i32,
}

impl TryFrom<Output> for CommandResult {
    type Error = anyhow::Error;

    fn try_from(value: Output) -> Result<Self, Self::Error> {
        let stdout = Some(String::from_utf8(value.stdout)?);
        let stderr = Some(String::from_utf8(value.stderr)?);
        let status_code = value.status.code().unwrap_or_default();

        Ok(Self {
            stdout,
            stderr,
            status_code,
        })
    }
}

impl From<ExitStatus> for CommandResult {
    fn from(value: ExitStatus) -> Self {
        Self {
            status_code: value.code().unwrap_or_default(),
            ..Default::default()
        }
    }
}
