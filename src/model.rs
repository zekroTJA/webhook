use std::process::{ExitStatus, Output};

use serde::Serialize;

#[derive(Serialize, Default)]
pub struct CommandResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<String>,
    pub status_code: Option<i32>,
}

impl TryFrom<Output> for CommandResult {
    type Error = anyhow::Error;

    fn try_from(value: Output) -> Result<Self, Self::Error> {
        let stdout = Some(String::from_utf8(value.stdout)?);
        let stderr = Some(String::from_utf8(value.stderr)?);
        let status_code = value.status.code();

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
            status_code: value.code(),
            ..Default::default()
        }
    }
}
