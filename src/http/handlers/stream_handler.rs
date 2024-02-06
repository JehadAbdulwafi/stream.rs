use anyhow::anyhow;
use axum::{
    extract::Json,
    extract::{Query, Request},
    http::StatusCode,
    response::IntoResponse,
    Extension,
};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use crate::{error::AppError, State};

#[derive(Debug, Deserialize, Serialize)]
pub struct PublishEvent {
    action: String,
    client_id: String,
    ip: String,
    vhost: String,
    app: String,
    stream: String,
    param: String,
    server_id: String,
    stream_url: String,
    stream_id: String,
}

struct Stream {
    id: Uuid,
    islive: Option<bool>,
}


pub async fn on_publish(
    app_state: Extension<State>,
    Json(payload): Json<PublishEvent>,
) -> Result<impl IntoResponse, AppError> {
    let response = serde_json::json!({
        "code": 0
    });

    let token = payload
        .param
        .splitn(2, '&')
        .next()
        .map(|s| s.trim_start_matches("?token="))
        .unwrap_or_default()
        .to_string();

    info!("token: {}", token);

    let stream: Stream = sqlx::query_as!(
        Stream,
        "SELECT id, isLive FROM streams WHERE app = $1 AND stream_name = $2",
        payload.app,
        payload.stream,
    )
    .fetch_one(&app_state.pool)
    .await
    .map_err(|err| anyhow!("Failed to fetch user: {}", err))?;

    info!("on_publish => Received request: {:?} \n", payload);

    match stream.islive {
        Some(value) => {
            if value == true {
                return Ok((StatusCode::CONFLICT, "Stream already published").into_response());
            } else {
                let _ = sqlx::query!("UPDATE streams SET isLive = true WHERE id = $1", stream.id)
                    .execute(&app_state.pool)
                    .await
                    .map_err(|err| anyhow!("Failed to update stream status: {}", err))?;
                return Ok((StatusCode::OK, response.to_string()).into_response());
            }
        }
        None => return Ok((StatusCode::CONFLICT, "Stream already published").into_response()),
    }
}

pub async fn on_unpublish(
    app_state: Extension<State>,
    Json(payload): Json<PublishEvent>,
) -> Result<impl IntoResponse, AppError> {
    let response = serde_json::json!({
        "code": 0
    });

    let token = payload
        .param
        .splitn(2, '&')
        .next()
        .map(|s| s.trim_start_matches("token="))
        .unwrap_or_default()
        .to_string();

    info!("token: {}", token);

    let stream: Stream = sqlx::query_as!(
        Stream,
        "SELECT id, isLive FROM streams WHERE app = $1 AND stream_name = $2",
        payload.app,
        payload.stream,
    )
    .fetch_one(&app_state.pool)
    .await
    .map_err(|err| anyhow!("Failed to fetch user: {}", err))?;

    info!("on_publish => Received request: {:?} \n", payload);

    match stream.islive {
        Some(value) => {
            if value == false {
                return Ok((StatusCode::CONFLICT, "Stream already unpublished").into_response());
            } else {
                let _ = sqlx::query!("UPDATE streams SET isLive = false WHERE id = $1", stream.id)
                    .execute(&app_state.pool)
                    .await
                    .map_err(|err| anyhow!("Failed to update stream status: {}", err))?;
                return Ok((StatusCode::OK, response.to_string()).into_response());
            }
        }
        None => return Ok((StatusCode::CONFLICT, "Stream already published").into_response()),
    }
}

pub async fn on_play(request: Request) -> String {
    // TODO: increse number of viewers
    let response = serde_json::json!({
        "code": 0
    });

    info!("on_play => Received request: {:?} \n", request);

    response.to_string()
}
pub async fn on_stop(request: Request) -> String {
    // TODO: decrese number of viewers
    let response = serde_json::json!({
        "code": 0
    });

    info!("on_stop => Received body: {:?} \n", request.body());

    response.to_string()
}

