use axum::{
    body::{Body, to_bytes},
    http::{self, Request, Response, StatusCode},
};
use serde::{Deserialize, Deserializer, de};
use solana_keypair::{Signature, Signer};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tracing::info;

use crate::app::AppState;

pub fn deserialize_signature<'de, D>(deserializer: D) -> Result<Signature, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let signature: Signature = s
        .parse()
        .map_err(|_| de::Error::custom("Cant parse signature"))?;

    Ok(signature)
}

#[derive(Deserialize)]
pub struct SignatureRecoverBody {
    #[serde(deserialize_with = "deserialize_signature")]
    pub signature: Signature,
    pub message: String,
}

#[derive(Clone)]
pub struct SignatureRecoverLayer {
    state: AppState,
}

impl SignatureRecoverLayer {
    pub fn new(state: AppState) -> SignatureRecoverLayer {
        Self { state }
    }
}

impl<S> Layer<S> for SignatureRecoverLayer {
    type Service = SignatureRecoverService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SignatureRecoverService {
            inner,
            state: self.state.clone(),
        }
    }
}

#[derive(Clone)]
pub struct SignatureRecoverService<S> {
    inner: S,
    state: AppState,
}

impl<S> Service<http::Request<Body>> for SignatureRecoverService<S>
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
        let state = self.state.clone();

        Box::pin(async move {
            let (parts, body) = req.into_parts();

            let bytes = match to_bytes(body, usize::MAX).await {
                Ok(b) => b,
                Err(_) => {
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Failed to read request body"))
                        .unwrap());
                }
            };

            let parsed_body: SignatureRecoverBody = match serde_json::from_slice(&bytes) {
                Ok(b) => b,
                Err(_) => {
                    return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Invalid or missing signature in request body"))
                        .unwrap());
                }
            };

            info!("{}", parsed_body.signature.to_string());

            let is_verified = parsed_body.signature.verify(
                state.app_keypair.pubkey().to_bytes().as_ref(),
                parsed_body.message.as_ref(),
            );

            info!("Verifier {}", is_verified);
            let req = Request::from_parts(parts, Body::from(bytes));
            let response = inner.call(req).await?;
            Ok(response)
        })
    }
}
