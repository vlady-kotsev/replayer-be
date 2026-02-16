use anyhow::{Result, anyhow};
use axum::Router;
use routes::hehe_router;
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::db::create_connection_pool;

mod config;
mod db;
mod handlers;
mod models;
mod routes;

pub struct App;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
}

impl App {
    pub async fn run() -> Result<App> {
        tracing_subscriber::fmt::try_init()
            .map_err(|_| anyhow!("Failed to start tracing subscriber"))?;

        let pool = create_connection_pool().await?;
        info!("Conencted to db");
        let app_state = AppState { db: pool };

        let app = Router::new()
            .merge(hehe_router())
            .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
            .with_state(app_state);

        let port = 3000;
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
        info!(port =%port, "Server started... ");
        axum::serve(listener, app).await?;
        Ok(App)
    }
}
