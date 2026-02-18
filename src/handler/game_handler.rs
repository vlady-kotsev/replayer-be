use axum::{Json, extract::State};

use crate::{
    AppResult,
    app::AppServices,
    handler::{CreateGameRequest, GameResponse},
    model::CreateGameModel,
};

pub async fn get_all_games(
    State(services): State<AppServices>,
) -> AppResult<Json<Vec<GameResponse>>> {
    let game_models = services.game.get_all_games().await?;

    let game_responses: Vec<GameResponse> = game_models.into_iter().map(Into::into).collect();

    Ok(Json(game_responses))
}

pub async fn create_game(
    State(services): State<AppServices>,
    Json(game_request): Json<CreateGameRequest>,
) -> AppResult<()> {
    let create_game_model: CreateGameModel = game_request.into();
    services.game.create_game(create_game_model).await
}
