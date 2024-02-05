use axum::{extract::Request, extract::Json};
use serde::{Deserialize, Serialize};
use tracing::info;

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

pub async fn on_publish(Json(request): Json<PublishEvent>) -> String {
    let response = serde_json::json!({
        "code": 0
    });


    info!("on_publish => Received request: {:?} \n", request);

    response.to_string()
}

pub async fn on_unpublish(Json(request): Json<PublishEvent>) -> String {
    let response = serde_json::json!({
        "code": 0
    });

    // TODO: turn off stream

    info!("on_unpublish => Received request: {:?} \n", request);

    response.to_string()
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
