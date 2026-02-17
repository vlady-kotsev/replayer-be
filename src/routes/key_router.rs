use axum::{Router, routing::post};

use crate::{app::AppState, handler::get_key};

pub fn key_router() -> Router<AppState> {
    Router::new().route("/key", post(get_key))
    // .route_layer(layer)
}
