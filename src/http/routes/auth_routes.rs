use crate::http::handlers::auth_handler::{login, sign_up};
use axum::routing::post;
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/signup", post(sign_up))
        .route("/login", post(login))
}
