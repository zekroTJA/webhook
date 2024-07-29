use super::ParseFromFile;
use core::fmt;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone)]
pub struct DynamicConfig {
    pub auth: Option<HashMap<String, Auth>>,
    pub hooks: HashMap<String, Hook>,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum Auth {
    Basic(BasicAuth),
    Bearer(BearerAuth),
}

#[derive(Deserialize, Clone)]
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Clone)]
pub struct BearerAuth {
    pub token: String,
}

#[derive(Deserialize, Clone)]
pub struct Hook {
    pub command: String,
    pub method: Option<String>,
    pub auth: Option<Vec<String>>,
    pub args: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
}

impl ParseFromFile for DynamicConfig {}

impl fmt::Display for DynamicConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (path, hook) in &self.hooks {
            if let Some(ref method) = hook.method {
                write!(f, "-> {} ", method.to_uppercase())?
            } else {
                write!(f, "-> ANY ")?
            }
            write!(f, "/{} : {}", path, hook.command)?;
            if let Some(ref args) = hook.args {
                for arg in args {
                    write!(f, " {arg}")?;
                }
            }

            write!(f, "\n   Auth: ")?;

            if let Some(ref auth) = hook.auth {
                if auth.is_empty() {
                    write!(f, "NONE")?;
                } else {
                    write!(f, "{}", auth.join(", "))?;
                }
            } else {
                write!(f, "NONE")?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
