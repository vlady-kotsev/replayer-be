use crate::{errors::AppError, middleware::recover_signature::RecoverSignatureBody};
use axum::{
    body::{Body, to_bytes},
    http::{self, Request},
    response::{IntoResponse, Response},
};

use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tower::Service;

#[derive(Clone)]
pub struct RecoverSignatureService<S> {
    pub inner: S,
}

impl<S> Service<http::Request<Body>> for RecoverSignatureService<S>
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

        Box::pin(async move {
            let (parts, body) = req.into_parts();

            let bytes = match to_bytes(body, usize::MAX).await {
                Ok(b) => b,
                Err(e) => {
                    return Ok(AppError::bad_request(e.to_string()).into_response());
                }
            };

            let parsed_body: RecoverSignatureBody = match serde_json::from_slice(&bytes) {
                Ok(pb) => pb,
                Err(e) => {
                    return Ok(AppError::bad_request(e.to_string()).into_response());
                }
            };

            let is_verified = parsed_body
                .signature
                .verify(parsed_body.developer.as_ref(), parsed_body.message.as_ref());

            if !is_verified {
                return Ok(AppError::forbidden("Invalid signer").into_response());
            }

            let req = Request::from_parts(parts, Body::from(bytes));
            let response = inner.call(req).await?;
            Ok(response)
        })
    }
}
