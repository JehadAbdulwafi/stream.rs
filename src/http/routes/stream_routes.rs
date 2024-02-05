use crate::http::handlers::stream_handler::{on_play, on_publish, on_stop, on_unpublish};
use axum::routing::post;
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/publish", post(on_publish))
        .route("/unpublish", post(on_unpublish))
        .route("/play", post(on_play))
        .route("/stop", post(on_stop))
}
