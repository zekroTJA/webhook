use crate::{
    config::{Auth, Config},
    ResponseError,
};
use axum::http::HeaderValue;
use base64::{prelude::BASE64_STANDARD, Engine};

pub enum AuthenticationToken {
    Basic { username: String, password: String },
    Bearer(String),
}

impl TryFrom<&HeaderValue> for AuthenticationToken {
    type Error = anyhow::Error;

    fn try_from(value: &HeaderValue) -> Result<Self, Self::Error> {
        if value.is_empty() {
            anyhow::bail!("empty header value")
        }

        let value = value.to_str()?;
        let mut split = value.split_whitespace();

        let prefix = split.next().expect("first element");

        let token = match prefix.to_lowercase().as_str() {
            "bearer" => {
                let Some(v) = split.next() else {
                    anyhow::bail!("empty bearer value")
                };
                Self::Bearer(v.to_string())
            }
            "basic" => {
                let Some(v) = split.next() else {
                    anyhow::bail!("empty basic value")
                };
                let (username, password) = basic_decode(v)?;
                Self::Basic { username, password }
            }
            _ => anyhow::bail!("invalid auth header value"),
        };

        Ok(token)
    }
}

fn basic_decode(v: &str) -> anyhow::Result<(String, String)> {
    let decoded = String::from_utf8(BASE64_STANDARD.decode(v)?)?;

    let (username, password) = decoded
        .split_once(':')
        .ok_or_else(|| anyhow::anyhow!("invalid basic token format"))?;

    Ok((username.to_string(), password.to_string()))
}

pub fn check_auth(
    auth_keys: &[String],
    config: &Config,
    token: AuthenticationToken,
) -> Result<(), ResponseError> {
    for auth_key in auth_keys {
        let Some(auth) = config.auth.get(auth_key) else {
            return Err(ResponseError::InternalServerError(format!(
                "misconfigured hook auth: '{auth_key}' does not exist"
            )));
        };

        match auth {
            Auth::Basic(basic_auth) => {
                if let AuthenticationToken::Basic {
                    ref username,
                    ref password,
                } = token
                {
                    if &basic_auth.username == username && &basic_auth.password == password {
                        return Ok(());
                    }
                };
            }
            Auth::Bearer(beaere_auth) => {
                if let AuthenticationToken::Bearer(ref token) = token {
                    if &beaere_auth.token == token {
                        return Ok(());
                    }
                };
            }
        }
    }

    Err(ResponseError::Unauthorized)
}
