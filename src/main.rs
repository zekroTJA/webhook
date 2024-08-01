mod auth;
mod config;
mod model;

use auth::{check_auth, AuthenticationToken};
use axum::{
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::any,
    Json, Router,
};
use clap::Parser;
use config::{DynamicConfig, Loader, ParseFromFile, StaticConfig};
use model::CommandResult;
use std::{path::PathBuf, process::Command, str::FromStr, sync::Arc};
use tower_http::trace::TraceLayer;
use tracing::{debug, info};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the static config file
    #[arg(short, long, default_value = "config.yaml")]
    static_conf: PathBuf,

    /// Path to the dynamic config file
    #[arg(short, long, default_value = "dynamic.yaml")]
    dynamic_conf: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config = StaticConfig::from_file(cli.static_conf)?;

    let log_level = config
        .logging
        .and_then(|l| l.level)
        .map(|l| tracing::Level::from_str(&l))
        .transpose()?
        .unwrap_or(tracing::Level::INFO);
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_writer(std::io::stdout)
        .init();

    let server_address = config
        .server
        .as_ref()
        .and_then(|v| v.address.as_ref())
        .cloned()
        .unwrap_or_else(|| "0.0.0.0:8080".to_string());

    let loader: Loader<DynamicConfig> = Loader::new(cli.dynamic_conf)?;

    debug!("Loaded hooks:\n{}", loader.get_cached());

    let loader = Arc::new(loader);

    let app = Router::new()
        .route("/*path", any(handler))
        .with_state(loader)
        .layer(TraceLayer::new_for_http());

    info!("Server listening on {server_address} ...");

    let listener = tokio::net::TcpListener::bind(server_address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

enum ResponseError {
    NotFound,
    BadRequest(String),
    Unauthorized,
    InternalServerError(String),
    MethodNotAllowed,
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        match self {
            Self::NotFound => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap(),
            Self::InternalServerError(msg) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(msg))
                .unwrap(),
            Self::BadRequest(msg) => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(msg))
                .unwrap(),
            Self::Unauthorized => Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::empty())
                .unwrap(),
            Self::MethodNotAllowed => Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body(Body::empty())
                .unwrap(),
        }
    }
}

async fn handler(
    Path(path): Path<String>,
    method: Method,
    State(config): State<Arc<Loader<DynamicConfig>>>,
    header: HeaderMap,
) -> Result<Json<CommandResult>, ResponseError> {
    let cfg = config
        .get()
        .map_err(|e| ResponseError::InternalServerError(e.to_string()))?;

    let Some(hook) = cfg.hooks.get(path.trim_end_matches('/')) else {
        return Err(ResponseError::NotFound);
    };

    if let Some(ref hook_method) = hook.method {
        if hook_method != method.as_str() {
            return Err(ResponseError::MethodNotAllowed);
        }
    }

    if let Some(ref auth_keys) = hook.auth {
        let auth_value: Option<AuthenticationToken> = header
            .get("authorization")
            .map(|v| v.try_into())
            .transpose()
            .map_err(|e: anyhow::Error| ResponseError::BadRequest(e.to_string()))?;

        match auth_value {
            None => {
                if !auth_keys.is_empty() {
                    return Err(ResponseError::Unauthorized);
                }
            }
            Some(auth_value) => {
                check_auth(auth_keys, &cfg, auth_value)?;
            }
        }
    }

    let mut cmd = Command::new(&hook.command);

    if let Some(ref args) = hook.args {
        cmd.args(args);
    }

    if let Some(ref env) = hook.env {
        cmd.envs(env);
    }

    let output = cmd
        .output()
        .map_err(|e| ResponseError::InternalServerError(e.to_string()))?;

    let res = output
        .try_into()
        .map_err(|e: anyhow::Error| ResponseError::InternalServerError(e.to_string()))?;

    Ok(Json(res))
}
