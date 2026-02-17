use crate::{
    config::load_config,
    db::create_connection_pool,
    errors::AppResult,
    routes::{game_router, key_router},
    service::{GameService, SignerService},
};
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
    pub async fn run() -> AppResult<App> {
        tracing_subscriber::fmt::try_init()?;

        let app_config = load_config().await?;
        let pool = create_connection_pool(&app_config.app.database_url).await?;
        info!("Conencted to db");

        let keypair = Keypair::new_from_array(app_config.solana.keypair_bytes[..32].try_into()?);

        let game_service = GameService::new(pool);

        let signer_service = SignerService::new(Arc::new(keypair), 2);

        let app_state = AppState {
            signer_service,
            game_service,
        };

        let app = Router::new()
            .merge(game_router())
            .merge(key_router())
            .layer(
                ServiceBuilder::new().layer(TraceLayer::new_for_http()), //.layer(RetryLayer::new(policy)),
            )
            .with_state(app_state);

        let listener =
            TcpListener::bind(format!("{}:{}", app_config.app.host, app_config.app.port)).await?;
        info!(port =%app_config.app.port, "Server started... ");

        axum::serve(listener, app).await?;

        Ok(App)
    }
}
