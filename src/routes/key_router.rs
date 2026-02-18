use std::sync::Arc;

use axum::{Router, routing::post};
use solana_keypair::Address;

use crate::{app::AppState, client::SolanaClient, handler::get_key, middleware::{RecoverSignatureLayer, ValidateNftLayer}};

pub fn key_router(client: Arc<SolanaClient>, program_id: Address) -> Router<AppState> {
    Router::new().route("/key", post(get_key))
    .route_layer(ValidateNftLayer::new(client, program_id))
    .route_layer(RecoverSignatureLayer)
}
