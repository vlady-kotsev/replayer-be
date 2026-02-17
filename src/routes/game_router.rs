use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    app::AppState,
    handler::{create_game, get_all_games},
    middleware::SignatureRecoverLayer,
};

pub fn game_router() -> Router<AppState> {
    Router::new()
        .route("/games", post(create_game))
        .route_layer(SignatureRecoverLayer)
        .route("/games", get(get_all_games)) //.post(handlers::create_user))
}
