use crate::{error::AppError, State, utils::generate_random_string};
use anyhow::{anyhow, Result};
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    Extension,
};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Deserialize, Serialize)]
pub struct SingupEvent {
    name: String,
    email: String,
    password: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginEvent {
    email: String,
    password: String,
}

pub async fn sign_up(
    app_state: Extension<State>,
    Json(payload): Json<SingupEvent>,
) -> Result<impl IntoResponse, AppError> {
    info!("signup => Received request: {:?} \n", payload);

    if payload.name.is_empty() || payload.email.is_empty() || payload.password.is_empty() {
        return Ok((StatusCode::BAD_REQUEST, "Invalid payload").into_response());
    }

    let existing_user = sqlx::query!(
        "SELECT id FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&app_state.pool)
    .await
    .map_err(|err| {
        anyhow!("Failed to fetch user: {}", err)
    })?; 

    if existing_user.is_some() {
        return Ok((StatusCode::CONFLICT, "User already exists").into_response());
    }

    let user = sqlx::query!(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id",
        payload.name,
        payload.email,
        payload.password,
    )
    .fetch_one(&app_state.pool)
    .await
    .map_err(|err| {
        anyhow!("Failed to create a new user: {}", err)
    })?;


    info!("New user created with ID: {:?}", user.id);

    let stream_name = generate_random_string(12);

    let stream = sqlx::query!(
        "INSERT INTO streams (app, stream_name, user_id, url) VALUES ($1, $2, $3, $4) RETURNING id",
        "live",
        stream_name,
        user.id,
        "rtmp://localhost:1935/live/".to_owned() + &stream_name,
    )
    .fetch_one(&app_state.pool)
    .await
    .map_err(|err| {
        anyhow!("Failed to create a new stream: {}", err)
    })?;

    info!("New stream created with ID: {:?}", stream.id);

    Ok((StatusCode::OK, "signup successful").into_response())
}

pub async fn login(app_state: Extension<State>, Json(payload): Json<LoginEvent>) -> Result<impl IntoResponse, AppError> {
    info!("login => Received request: {:?} \n", payload);

    if payload.email.is_empty() || payload.password.is_empty() {
        return Ok((StatusCode::BAD_REQUEST, "Invalid payload").into_response());
    }

    let user = sqlx::query!(
        "SELECT id FROM users WHERE email = $1 AND password = $2",
        payload.email,
        payload.password,
    )
    .fetch_optional(&app_state.pool)
    .await
    .map_err(|err| {
        anyhow!("Failed to fetch user: {}", err)
    })?;

    if user.is_none() {
        return Ok((StatusCode::UNAUTHORIZED, "Invalid credentials").into_response());
    }

    Ok((StatusCode::OK, "login successful").into_response())
}


