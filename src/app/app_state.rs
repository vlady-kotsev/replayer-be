use crate::service::{GameService, SignerService};

#[derive(Clone)]
pub struct AppState {
    pub game_service: GameService,
    pub signer_service: SignerService,
}
