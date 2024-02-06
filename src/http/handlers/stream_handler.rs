use anyhow::anyhow;
use axum::{extract::Json, http::StatusCode, response::IntoResponse, Extension};
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
    is_alive: Option<bool>,
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
        "SELECT id, is_alive FROM streams WHERE app = $1 AND stream_name = $2",
        payload.app,
        payload.stream,
    )
    .fetch_one(&app_state.pool)
    .await
    .map_err(|err| anyhow!("Failed to find stream: {}", err))?;

    info!("on_publish => Received request: {:?} \n", payload);

    match stream.is_alive {
        Some(value) => {
            if value == true {
                return Ok((StatusCode::CONFLICT, "Stream already published").into_response());
            } else {
                let _ = sqlx::query!(
                    "UPDATE streams SET is_alive = true WHERE id = $1",
                    stream.id
                )
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
        "SELECT id, is_alive FROM streams WHERE app = $1 AND stream_name = $2",
        payload.app,
        payload.stream,
    )
    .fetch_one(&app_state.pool)
    .await
    .map_err(|err| anyhow!("Failed to find stream: {}", err))?;

    info!("on_unpublish => Received request: {:?} \n", payload);

    match stream.is_alive {
        Some(value) => {
            if value == false {
                return Ok((StatusCode::CONFLICT, "Stream already unpublished").into_response());
            } else {
                let _ = sqlx::query!(
                    "UPDATE streams SET is_alive = true, viewers = 0 WHERE id = $1",
                    stream.id
                )
                .execute(&app_state.pool)
                .await
                .map_err(|err| anyhow!("Failed to update stream status: {}", err))?;
                return Ok((StatusCode::OK, response.to_string()).into_response());
            }
        }
        None => return Ok((StatusCode::CONFLICT, "Stream already published").into_response()),
    }
}

pub async fn on_play(
    app_state: Extension<State>,
    Json(payload): Json<PublishEvent>,
) -> Result<impl IntoResponse, AppError> {
    info!("on_play => Received request: {:?} \n", payload);
    let response = serde_json::json!({
        "code": 0
    });

    let stream: Stream = sqlx::query_as!(
        Stream,
        "SELECT id, is_alive FROM streams WHERE app = $1 AND stream_name = $2",
        payload.app,
        payload.stream,
    )
    .fetch_one(&app_state.pool)
    .await
    .map_err(|err| anyhow!("Failed to find stream: {}", err))?;

    let _ = sqlx::query!(
        "UPDATE streams SET viewers = viewers + 1, total_viewers = total_viewers + 1 WHERE id = $1",
        stream.id
    )
    .execute(&app_state.pool)
    .await
    .map_err(|err| anyhow!("Failed to update stream status: {}", err))?;


    Ok((StatusCode::OK, response.to_string()).into_response())
}
pub async fn on_stop(
    app_state: Extension<State>,
    Json(payload): Json<PublishEvent>,
) -> Result<impl IntoResponse, AppError> {
    let response = serde_json::json!({
        "code": 0
    });

    let stream: Stream = sqlx::query_as!(
        Stream,
        "SELECT id, is_alive FROM streams WHERE app = $1 AND stream_name = $2",
        payload.app,
        payload.stream,
    )
    .fetch_one(&app_state.pool)
    .await
    .map_err(|err| anyhow!("Failed to find stream: {}", err))?;

    let _ = sqlx::query!(
        "UPDATE streams SET viewers = viewers - 1 WHERE id = $1",
        stream.id
    )
    .execute(&app_state.pool)
    .await
    .map_err(|err| anyhow!("Failed to update stream status: {}", err))?;

    info!("on_stop => Received request: {:?} \n", payload);

    Ok((StatusCode::OK, response.to_string()).into_response())
}
