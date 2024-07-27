use std::process::Output;

use serde::Serialize;

#[derive(Serialize)]
pub struct CommandResult {
    pub stdout: String,
    pub stderr: String,
    pub status_code: i32,
}

impl TryFrom<Output> for CommandResult {
    type Error = anyhow::Error;

    fn try_from(value: Output) -> Result<Self, Self::Error> {
        let stdout = String::from_utf8(value.stdout)?;
        let stderr = String::from_utf8(value.stderr)?;
        let status_code = value.status.code().unwrap_or_default();

        Ok(Self {
            stdout,
            stderr,
            status_code,
        })
    }
}
