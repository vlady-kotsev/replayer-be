use axum::http::{Response, StatusCode};

pub async fn get_key() -> Result<Response<String>, StatusCode> {
    Ok(Response::new("Response".to_string()))
}
