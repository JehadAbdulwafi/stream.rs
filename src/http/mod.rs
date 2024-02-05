use anyhow::Context;
use axum::{Extension, Router};
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::State;
pub mod routes;
pub mod handlers;

pub fn app(app_state: State) -> Router {
    Router::new()
        .nest("/api/users", routes::auth_routes::router())
        .nest("/api/streams", routes::stream_routes::router())
        .layer(Extension(app_state))
        .layer(TraceLayer::new_for_http())
}

pub async fn serve(app_state: State) -> Result<(), anyhow::Error> {
    let listener = tokio::net::TcpListener::bind("192.168.1.10:8000").await?;

    info!("listening on: http://localhost:8000");

    let _ = axum::serve(listener, app(app_state)).await.context("failed to serve API");

    Ok(())
}
