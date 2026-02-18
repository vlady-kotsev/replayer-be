use crate::{
    AppResult,
    app::AppServices,
    handler::{GetKeyRequest, KeyResponse},
};
use axum::{Json, extract::State};

pub async fn get_game_key(
    State(services): State<AppServices>,
    Json(key_request): Json<GetKeyRequest>,
) -> AppResult<Json<KeyResponse>> {
    let get_key_model = key_request.into();
    let signature_model = services.signer.sign_message(&get_key_model);
    let key_model = services.key.get_game_key(get_key_model).await?;

    Ok(Json((key_model, signature_model).into()))
}
