use crate::{
    client::SolanaClient,
    config::{Config, load_config},
    db::create_connection_pool,
    errors::{AppError, AppResult},
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
pub struct App {
    pub config: Config,
    pub state: AppState,
}

impl App {
    pub async fn init() -> AppResult<App> {
        tracing_subscriber::fmt::try_init().map_err(|e| AppError::internal(e.to_string()))?;

        let app_config = load_config().await?;
        let pool = create_connection_pool(&app_config.app.database_url).await?;
        info!("Conencted to db");

        let keypair = Keypair::new_from_array(
            app_config.solana.keypair_bytes[..32]
                .try_into()
                .map_err(|_| AppError::internal("Error loading keypair"))?,
        );

        let game_service = GameService::new(pool);

        let signer_service = SignerService::new(Arc::new(keypair), 2);

        let app_state = AppState {
            signer_service,
            game_service,
        };

        Ok(App {
            config: app_config,
            state: app_state,
        })
    }

    pub async fn run() -> AppResult<()> {
        let app = App::init().await?;

        let client = Arc::new(SolanaClient::new(app.config.solana.rpc_url));

        let router = Router::new()
            .merge(game_router())
            .merge(key_router(client, app.config.solana.program_id))
            .layer(
                ServiceBuilder::new().layer(TraceLayer::new_for_http()), //.layer(RetryLayer::new(policy)),
            )
            .with_state(app.state);

        let listener =
            TcpListener::bind(format!("{}:{}", app.config.app.host, app.config.app.port))
                .await
                .map_err(|e| AppError::internal(e.to_string()))?;

        info!(port =%app.config.app.port, "Server started... ");

        axum::serve(listener, router)
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        Ok(())
    }
}
