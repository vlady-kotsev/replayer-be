use crate::service::{GameService, KeyService, SignerService};

#[derive(Clone)]
pub struct AppServices {
    pub game: GameService,
    pub key: KeyService,
    pub signer: SignerService,
}
