use crate::models::{database, intern};

pub async fn get_diretorio_by_reclamacao_protocolo<'a, E>(
	executor: E,
	protocolo: intern::Protocolo,
) -> Result<Option<intern::Diretorio>, sqlx::Error>
where
	E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
	let dir: Option<database::Diretorio> =
		sqlx::query_as(crate::services::queries::diretorio::GET_BY_PROTOCOLO)
			.bind(String::from(&protocolo))
			.fetch_optional(executor)
			.await?;

	Ok(dir.map(|d| d.into()))
}
