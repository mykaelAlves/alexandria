use axum::{
	http::StatusCode,
	response::{Html, IntoResponse},
};
use serde::Deserialize;

use crate::log::debug;
use crate::util::Pagination;

/// Handler ANY /
pub async fn root() -> impl IntoResponse {
	#[cfg(debug_assertions)]
	debug("Requisição para /");

	let body = Html(
		r#"
		<h1>Alexandria</h1>
		"#,
	);

	(StatusCode::OK, body)
}

pub mod motivo {
	use axum::{
		Json,
		extract::{Query, State},
	};

	use crate::{log::err, services, util::Paginated};
	use crate::{models::intern, run::GlobalState};

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

		let motivos = match services::motivo::list(
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

		let resultado =
			match services::motivo::insert(&state.0.pg_pool, motivo).await {
				Ok(motivo) => motivo,
				Err(e) => {
					err(&format!("Erro ao criar motivo: {}", e), Box::new(e));
					return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
				}
			};

		(StatusCode::CREATED, Json(Some(resultado)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::run::GlobalState;
	use axum::{
		Router,
		body::Body,
		http::{self, Request, StatusCode},
		routing::{get, post},
	};
	use core::time;
	use dotenvy::dotenv;
	use serde_json::json;
	use sqlx::{
		PgPool, Postgres, migrate::MigrateDatabase, postgres::PgPoolOptions,
	};
	use std::{sync::Arc, time::SystemTime};
	use tower::ServiceExt;

	use crate::models::intern::Motivo;
	use crate::util::Paginated;

	async fn setup_test_app() -> (Router, PgPool) {
		let server = crate::run::ServerApp::new().await.unwrap();

		let app = Router::new()
			.route("/", get(root))
			.route("/motivo", get(motivo::root))
			.route("/motivo/list", get(motivo::list))
			.route("/motivo/create", post(motivo::create))
			.with_state(server.state().clone());

		let db_url = dotenv()
			.ok()
			.and_then(|_| std::env::var("DATABASE_URL").ok())
			.unwrap();
		let test_db_url = format!(
			"{}{}{}",
			db_url,
			"_test",
			SystemTime::now()
				.duration_since(SystemTime::UNIX_EPOCH)
				.unwrap()
				.as_secs()
		);

		if Postgres::database_exists(&test_db_url)
			.await
			.unwrap_or(false)
		{
			Postgres::drop_database(&test_db_url)
				.await
				.unwrap();
		}
		Postgres::create_database(&test_db_url)
			.await
			.unwrap();

		let pg_pool = PgPoolOptions::new()
			.max_connections(5)
			.connect(&test_db_url)
			.await
			.expect("Falha ao conectar ao banco de dados de teste.");

		sqlx::migrate!("./migrations")
			.run(&pg_pool)
			.await
			.expect("Falha ao executar migrations no banco de teste.");

		(app, pg_pool)
	}

	async fn cleanup_test_db(pool: &PgPool) {
		sqlx::query("DROP SCHEMA public CASCADE; CREATE SCHEMA public;")
			.execute(pool)
			.await
			.unwrap();
	}

	async fn response_body_to_json<T: for<'de> serde::Deserialize<'de>>(
		response: axum::response::Response,
	) -> T {
		let body_bytes = axum::body::to_bytes(response.into_body(), 15912)
			.await
			.unwrap();
		serde_json::from_slice(&body_bytes).unwrap()
	}

	#[tokio::test]
	async fn test_root_handler() {}

	#[tokio::test]
	async fn test_motivo_root_handler() {}

	#[tokio::test]
	async fn test_motivo_create() {
		let (app, _pool) = setup_test_app().await;

		let motivo_payload = json!({
			"nome": "Art. 157 - Roubo",
			"artigo": 157,
			"paragrafo_unico": false,
			"inciso": 2
		});

		let request = Request::builder()
			.method(http::Method::POST)
			.uri("/motivo/create")
			.header(http::header::CONTENT_TYPE, "application/json")
			.body(Body::from(motivo_payload.to_string()))
			.unwrap();

		let response = app.oneshot(request).await.unwrap();

		assert_eq!(response.status(), StatusCode::CREATED);

		let created_motivo: Option<Motivo> =
			response_body_to_json(response).await;

		let motivo =
			created_motivo.expect("O corpo da resposta não deveria ser nulo.");

		assert_eq!(motivo.nome, "Art. 157 - Roubo");
		assert_eq!(motivo.artigo, 157);
		assert_eq!(motivo.paragrafo_unico, false);
		assert_eq!(motivo.inciso, Some(2));

		cleanup_test_db(&_pool).await;
	}

	#[tokio::test]
	async fn test_motivo_list() {
		let (app, pool) = setup_test_app().await;

		sqlx::query(
			"INSERT INTO motivo (nome, artigo, paragrafo_unico, inciso) \
			 VALUES ($1, $2, $3, $4), ($5, $6, $7, $8), ($9, $10, $11, $12)",
		)
		.bind("Art. 121 - Homicídio")
		.bind(121)
		.bind(false)
		.bind(Option::<i16>::None)
		.bind("Art. 155 - Furto")
		.bind(155)
		.bind(true)
		.bind(Option::<i16>::None)
		.bind("Art. 157 - Roubo")
		.bind(157)
		.bind(false)
		.bind(2)
		.execute(&pool)
		.await
		.unwrap();

		let response = app
			.oneshot(
				Request::builder()
					.uri("/motivo/list?page=1&per_page=2")
					.body(Body::empty())
					.unwrap(),
			)
			.await
			.unwrap();

		assert_eq!(response.status(), StatusCode::OK);

		let paginated_result: Paginated<Motivo> =
			response_body_to_json(response).await;

		assert_eq!(paginated_result.total, 3);
		assert_eq!(paginated_result.page, 1);
		assert_eq!(paginated_result.per_page, 2);
		assert_eq!(paginated_result.items.len(), 2);

		assert_eq!(paginated_result.items[0].nome, "Art. 121 - Homicídio");
		assert_eq!(paginated_result.items[0].artigo, 121);
		assert_eq!(paginated_result.items[1].nome, "Art. 155 - Furto");
		assert_eq!(paginated_result.items[1].paragrafo_unico, true);
	}
}
