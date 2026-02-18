use std::sync::Arc;

use axum::{Router, routing::post};
use solana_keypair::Address;

use crate::{
    app::AppServices,
    client::SolanaClient,
    handler::get_game_key,
    middleware::{RecoverSignatureLayer, ValidateNftLayer},
};

pub fn key_router(client: Arc<SolanaClient>, program_id: Address) -> Router<AppServices> {
    Router::new()
        .route("/keys", post(get_game_key))
        .route_layer(ValidateNftLayer::new(client, program_id))
        .route_layer(RecoverSignatureLayer)
}
