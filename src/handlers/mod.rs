use axum::{http::StatusCode, response::{Html, IntoResponse}};

use crate::log::debug;

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
