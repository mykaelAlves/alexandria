use axum::{
	Json,
	extract::{Query, State},
	response::{Html, IntoResponse},
};
use hyper::StatusCode;
use serde::Deserialize;

#[cfg(debug_assertions)]
use crate::log::debug;
use crate::models::database::MotivoId;
use crate::{app::GlobalState, models::intern, util::Pagination};
use crate::{log::err, services::entities::motivo, util::Paginated};

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

#[derive(Debug, Deserialize)]
pub struct GetMotivoParams {
	id: Option<i32>,
	nome: Option<String>,
}

/// Handler GET /motivo/get?id={} ou GET /motivo/get?nome={}
pub async fn get(
	state: State<GlobalState>,
	Query(params): Query<GetMotivoParams>,
) -> impl IntoResponse {
	#[cfg(debug_assertions)]
	debug("Requisição para /motivo/get");

	let resultado = if let Some(id) = params.id {
		motivo::get_by_id(&state.0.pg_pool, MotivoId(id)).await
	} else if let Some(nome) = params.nome {
		motivo::get_by_nome(&state.0.pg_pool, &nome).await
	} else {
		return (StatusCode::BAD_REQUEST, Json(None));
	};

	match resultado {
		Ok(motivo) => (StatusCode::OK, Json(motivo)),
		Err(e) => {
			err(&format!("Erro ao obter motivo: {}", e), Box::new(e));
			(StatusCode::INTERNAL_SERVER_ERROR, Json(None))
		}
	}
}
