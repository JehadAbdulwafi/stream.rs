use anyhow::{Ok, Result};
use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(root))
        .layer(TraceLayer::new_for_http());

    print!("hello world!");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await?;
    info!("listening on: http://localhost:8000");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "hello world!"
}
