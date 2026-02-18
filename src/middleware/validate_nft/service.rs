use crate::{client::SolanaClient, errors::AppError, middleware::ValidateNftBody};
use axum::{
    body::{Body, to_bytes},
    http::{self, Request},
    response::{IntoResponse, Response},
};
use mpl_core::accounts::BaseAssetV1;
use solana_keypair::Address;

use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tower::Service;

const GAME_KEY_ASSET_SEED: &[u8] = b"game_key";

#[derive(Clone)]
pub struct ValidateNftService<S> {
    pub inner: S,
    pub client: Arc<SolanaClient>,
    pub program_id: Address,
}

impl<S> Service<http::Request<Body>> for ValidateNftService<S>
where
    S: Service<http::Request<Body>, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send,
{
    type Response = Response<Body>;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: http::Request<Body>) -> Self::Future {
        let mut inner = self.inner.clone();
        let client = self.client.clone();
        let program_id = self.program_id;

        Box::pin(async move {
            let (parts, body) = req.into_parts();

            let bytes = match to_bytes(body, usize::MAX).await {
                Ok(b) => b,
                Err(e) => {
                    return Ok(AppError::bad_request(e.to_string()).into_response());
                }
            };

            let parsed_body: ValidateNftBody = match serde_json::from_slice(&bytes) {
                Ok(pb) => pb,
                Err(e) => {
                    return Ok(AppError::bad_request(e.to_string()).into_response());
                }
            };

            let (game_nft_address, _game_nft_bump) = Address::find_program_address(
                &[
                    GAME_KEY_ASSET_SEED,
                    parsed_body.developer.as_ref(),
                    parsed_body.game_name.as_bytes(),
                    parsed_body.player.as_ref(),
                ],
                &program_id,
            );
            let Some(result) = client.check_account_exists(&game_nft_address).await else {
                return Ok(AppError::not_found("NFT PDA not found").into_response());
            };

            let nft_asset = match BaseAssetV1::from_bytes(&result) {
                Ok(asset) => asset,
                Err(e) => return Ok(AppError::internal(e.to_string()).into_response()),
            };

            if nft_asset.owner.to_bytes().ne(parsed_body.player.as_array()) {
                return Ok(AppError::forbidden("Player doesn't own the game").into_response());
            }

            let req = Request::from_parts(parts, Body::from(bytes));
            let response = inner.call(req).await?;
            Ok(response)
        })
    }
}
