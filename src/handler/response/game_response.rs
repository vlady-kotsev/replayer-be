use serde::Serialize;

use crate::model::GameModel;

#[derive(Serialize)]
pub struct GameResponse {
    pub name: String,
    pub developer: String,
}

impl From<GameModel> for GameResponse {
    fn from(model: GameModel) -> Self {
        GameResponse {
            name: model.name,
            developer: model.developer.to_string(),
        }
    }
}
