use axum::http::StatusCode;
use axum::{
	Json,
	extract::{Query, State},
	response::{Html, IntoResponse},
};
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
		<h1>Motivo API</h1>
		<h2>Endpoints</h2>
		<ul>
			<li><strong>GET /motivo/list?page={}&per_page={}</strong> - Listar motivos com paginação</li>
			<li><strong>POST /motivo/create</strong> - Criar novo motivo (JSON body)</li>
			<li><strong>GET /motivo/get?id={}</strong> - Obter motivo por ID</li>
			<li><strong>GET /motivo/get?nome={}</strong> - Obter motivo por nome</li>
			<li><strong>PUT /motivo/update?id={}</strong> - Atualizar motivo por ID (JSON body)</li>
			<li><strong>DELETE /motivo/delete?id={}</strong> - Deletar motivo por ID</li>
			<li><strong>DELETE /motivo/delete?nome={}</strong> - Deletar motivo por nome</li>
			<li><strong>GET /motivo/count</strong> - Contar todos os motivos</li>
		</ul>
		<h2>Exemplos</h2>
		<p><code>GET /motivo/list?page=1&per_page=10</code></p>
		<p><code>POST /motivo/create</code> com JSON: <code>{"nome": "Motivo exemplo"}</code></p>
		<p><code>GET /motivo/get?id=1</code></p>
		<p><code>GET /motivo/get?nome=Motivo+exemplo</code></p>
		<p><code>PUT /motivo/update?id=1</code> com JSON: <code>{"nome": "Motivo atualizado"}</code></p>
		<p><code>DELETE /motivo/delete?id=1</code></p>
		<p><code>DELETE /motivo/delete?nome=Motivo+exemplo</code></p>
		<p><code>GET /motivo/count</code></p>
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
			err(&format!("Erro ao criar motivo"), Box::new(e));
			return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
		}
	};

	(StatusCode::CREATED, Json(Some(resultado)))
}

#[derive(Debug, Deserialize)]
pub struct MotivoParams {
	id: Option<i32>,
	nome: Option<String>,
}

type GetMotivoParams = MotivoParams;

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
			err(&format!("Erro ao obter motivo"), Box::new(e));
			(StatusCode::INTERNAL_SERVER_ERROR, Json(None))
		}
	}
}

type UpdateMotivoParams = MotivoParams;

/// Handler PUT /motivo/update?id={} ou PUT /motivo/update?nome={}
pub async fn update(
	state: State<GlobalState>,
	Query(params): Query<UpdateMotivoParams>,
) -> impl IntoResponse {
	// Handler implementation goes here
	(StatusCode::NOT_IMPLEMENTED, Json(None::<()>))
}

type DeleteMotivoParams = MotivoParams;

/// Handler DELETE /motivo/delete?id={} ou DELETE /motivo/delete?nome={}
pub async fn delete(
	state: State<GlobalState>,
	Query(params): Query<DeleteMotivoParams>,
) -> impl IntoResponse {
	#[cfg(debug_assertions)]
	debug("Requisição para /motivo/delete");

	match params {
		DeleteMotivoParams {
			id: Some(id),
			nome: None,
		} => match motivo::delete_by_id(&state.0.pg_pool, MotivoId(id)).await {
			Ok(message) => (StatusCode::OK, Json(Some(message))),
			Err(e) => {
				err(
					&format!("Erro ao deletar motivo por ID"),
					Box::new(e),
				);
				(StatusCode::INTERNAL_SERVER_ERROR, Json(None))
			}
		},
		DeleteMotivoParams {
			nome: Some(nome),
			id: None,
		} => match motivo::delete_by_nome(&state.0.pg_pool, &nome).await {
			Ok(_) => (
				StatusCode::OK,
				Json(Some(format!("Motivo '{}' deletado com sucesso.", nome))),
			),
			Err(e) => {
				err(
					&format!("Erro ao deletar motivo por nome"),
					Box::new(e),
				);
				(StatusCode::INTERNAL_SERVER_ERROR, Json(None))
			}
		},
		_ => (StatusCode::BAD_REQUEST, Json(None)),
	}
}

pub async fn count(state: State<GlobalState>) -> impl IntoResponse {
	#[cfg(debug_assertions)]
	debug("Requisição para /motivo/count");

	let resultado = match motivo::count_all(&state.0.pg_pool).await {
		Ok(count) => count,
		Err(e) => {
			err(&format!("Erro ao contar motivos: {}", e), Box::new(e));
			return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
		}
	};

	(StatusCode::OK, Json(Some(resultado)))
}
