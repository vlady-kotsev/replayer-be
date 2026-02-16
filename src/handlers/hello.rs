use axum::http::{Response, StatusCode};
use tracing::info;

pub async fn do_hehe() -> Result<Response<String>, StatusCode> {
    info!("We are hehe");
    Ok(Response::new("hehe".to_string()))
}
