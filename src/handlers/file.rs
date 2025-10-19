use axum::{
	extract::{Multipart, Query, State},
	http::StatusCode,
	response::{Html, IntoResponse},
};

#[cfg(debug_assertions)]
use crate::log::debug;
use crate::{
	app::GlobalState, models::intern::Protocolo, services::entities::reclamacao,
};

pub async fn upload(
	state: State<GlobalState>,
	file: Multipart,
	protocolo: Query<String>,
) -> impl IntoResponse {
	#[cfg(debug_assertions)]
	debug("Requisição para /file/upload");

	let protocolo: Protocolo = match protocolo.as_str().try_into() {
		Ok(p) => p,
		Err(e) => {
			return (
				StatusCode::BAD_REQUEST,
				format!("Formato de protocolo inválido: {}", e),
			);
		}
	};

	let dir = match reclamacao::get_diretorio_by_reclamacao_protocolo(
		&state.0.pg_pool,
		protocolo,
	)
	.await
	{
		Ok(Some(d)) => d,
		Ok(None) => {
			return (
				StatusCode::NOT_FOUND,
				"Reclamação não encontrada para o protocolo informado.".into(),
			);
		}
		Err(e) => {
			return (
				StatusCode::INTERNAL_SERVER_ERROR,
				format!("Erro ao obter diretório da reclamação: {}", e),
			);
		}
	};

	if !dir.modificavel {
		return (
			StatusCode::FORBIDDEN,
			"O diretório associado à reclamação não permite modificações."
				.into(),
		);
	}

	// TODO

	(
		StatusCode::OK,
		"Upload de arquivo efetuado com sucesso!".into(),
	)
}
