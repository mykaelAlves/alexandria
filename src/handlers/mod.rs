use axum::{http::StatusCode, response::IntoResponse};

use crate::log::debug;

pub async fn root() -> impl IntoResponse {
    #[cfg(debug_assertions)]
    debug("Requisição para /");

    StatusCode::FORBIDDEN
}
