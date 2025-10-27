use axum::{Json, extract::State, response::IntoResponse};

use crate::{app::GlobalState, models::intern};

pub async fn root() -> impl IntoResponse {
	// Handler implementation goes here
}

pub async fn create(
	state: State<GlobalState>,
	Json(motivo): Json<intern::Reclamacao>,
) -> impl IntoResponse {
	// Handler implementation goes here
}
