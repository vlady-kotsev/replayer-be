use crate::{
    config::load_config, db::create_connection_pool, middleware::SignatureRecoverLayer,
    routes::hehe_router,
};
use anyhow::{Result, anyhow};
use axum::Router;
use solana_keypair::Keypair;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;
mod app_state;

pub use app_state::AppState;
pub struct App;

impl App {
    pub async fn run() -> Result<App> {
        tracing_subscriber::fmt::try_init()
            .map_err(|_| anyhow!("Failed to start tracing subscriber"))?;

        let app_config = Arc::new(load_config().await?);
        let pool = create_connection_pool().await?;
        info!("Conencted to db");

        let keypair = Keypair::new_from_array(app_config.solana.keypair_bytes[..32].try_into()?);

        let app_state = AppState {
            db: pool,
            config: app_config.clone(),
            app_keypair: Arc::new(keypair),
        };

        let app = Router::new()
            .merge(hehe_router())
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    // .layer(SignatureRecoverLayer::new(app_state.clone())),
            )
            .with_state(app_state);
        let app_config = app_config.clone();
        let listener =
            TcpListener::bind(format!("{}:{}", app_config.app.host, app_config.app.port)).await?;
        info!(port =%app_config.app.port, "Server started... ");
        axum::serve(listener, app).await?;
        Ok(App)
    }
}
