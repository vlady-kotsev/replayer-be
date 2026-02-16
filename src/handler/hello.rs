use axum::{
    extract::State,
    http::{Response, StatusCode},
};
use tracing::info;

use crate::app::AppState;

pub async fn do_hehe(State(state): State<AppState>) -> Result<Response<String>, StatusCode> {
    info!("We are hehe");
    Ok(Response::new(state.config.app.database_url.clone()))
}
