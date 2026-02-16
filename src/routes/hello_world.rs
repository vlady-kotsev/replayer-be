use axum::{Router, routing::get};

use crate::{app::AppState, handler::do_hehe};

pub fn hehe_router() -> Router<AppState> {
    Router::new()
        .route("/hehe", get(do_hehe)) //.post(handlers::create_user))
        .route("/hue", get(do_hehe))
}
