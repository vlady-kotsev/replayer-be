use axum::{Json, extract::State};

use crate::{
    AppResult,
    app::AppState,
    handler::{CreateGameRequest, GameResponse},
    model::CreateGameModel,
};

pub async fn get_all_games(State(state): State<AppState>) -> AppResult<Json<Vec<GameResponse>>> {
    let game_models = state.game_service.get_all_games().await?;

    let game_responses: Vec<GameResponse> = game_models.into_iter().map(|m| m.into()).collect();

    Ok(Json(game_responses))
}

pub async fn create_game(
    State(state): State<AppState>,
    Json(game_request): Json<CreateGameRequest>,
) -> AppResult<()> {
    let create_game_model: CreateGameModel = game_request.into();
    state.game_service.create_game(create_game_model).await
}
