use axum::{Json, extract::State, http::StatusCode};

use crate::{
    app::AppState,
    handler::{CreateGameRequest, GameResponse},
    model::CreateGameModel,
};

pub async fn get_all_games(
    State(state): State<AppState>,
) -> Result<Json<Vec<GameResponse>>, StatusCode> {
    let game_models = state
        .game_service
        .get_all_games()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let game_responses: Vec<GameResponse> = game_models.into_iter().map(|m| m.into()).collect();

    Ok(Json(game_responses))
}

pub async fn create_game(
    State(state): State<AppState>,
    Json(game_request): Json<CreateGameRequest>,
) -> Result<(), StatusCode> {
    let create_game_model: CreateGameModel = game_request.into();
    state
        .game_service
        .create_game(create_game_model)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
