use axum::{
	Json,
	extract::{Query, State},
};

use crate::{log::err, services::entities::motivo, util::Paginated};
use crate::{models::intern, app::GlobalState};

use super::*;

/// Handler ANY /motivo
pub async fn root() -> impl IntoResponse {
	#[cfg(debug_assertions)]
	debug("Requisição para /motivo");

	let body = Html(
		r#"
        <h1>Motivo</h1>
        "#,
	);

	(StatusCode::OK, body)
}

/// Handler GET /motivo/list?page={}&per_page={}
pub async fn list(
	state: State<GlobalState>,
	pagination: Query<Pagination>,
) -> impl IntoResponse {
	#[cfg(debug_assertions)]
	debug("Requisição para /motivo/list");

	let motivos = match motivo::list(
		&state.0.pg_pool,
		pagination.0.page,
		pagination.0.per_page,
	)
	.await
	{
		Ok(motivos) => motivos,
		Err(e) => {
			err(&format!("Erro ao listar motivos: {}", e), Box::new(e));

			return (
				StatusCode::INTERNAL_SERVER_ERROR,
				Json(Paginated {
					items: vec![],
					total: 0,
					page: pagination.0.page,
					per_page: pagination.0.per_page,
				}),
			);
		}
	};

	(StatusCode::OK, Json(motivos))
}

/// Handler POST /motivo/create
pub async fn create(
	state: State<GlobalState>,
	Json(motivo): Json<intern::Motivo>,
) -> impl IntoResponse {
	#[cfg(debug_assertions)]
	debug("Requisição para /motivo/create");

	let resultado = match motivo::insert(&state.0.pg_pool, motivo).await {
		Ok(motivo) => motivo,
		Err(e) => {
			err(&format!("Erro ao criar motivo: {}", e), Box::new(e));
			return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
		}
	};

	(StatusCode::CREATED, Json(Some(resultado)))
}
