use crate::models::database;
use crate::models::intern;
use crate::services::queries;
use crate::util::Paginated;

fn from_db_vec(motivos_db: Vec<database::Motivo>) -> Vec<intern::Motivo> {
	motivos_db.into_iter().map(|m| m.into()).collect()
}

pub async fn get_all<'a, E>(
	executor: E,
) -> Result<Vec<intern::Motivo>, sqlx::Error>
where
	E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
	let motivos: Vec<database::Motivo> =
		sqlx::query_as(queries::motivo::GET_ALL)
			.fetch_all(executor)
			.await?;

	Ok(from_db_vec(motivos))
}

pub async fn list<'a, E>(
	executor: E,
	page: usize,
	per_page: usize,
) -> Result<Paginated<intern::Motivo>, sqlx::Error>
where
	E: sqlx::Executor<'a, Database = sqlx::Postgres> + Clone,
{
	let offset = page.saturating_sub(1) * per_page;

	let count_result: (i64,) = sqlx::query_as(queries::motivo::COUNT_ALL)
		.fetch_one(executor.clone())
		.await?;
	let total = count_result.0;

	let motivos: Vec<database::Motivo> = sqlx::query_as(queries::motivo::LIST)
		.bind(per_page as i64)
		.bind(offset as i64)
		.fetch_all(executor)
		.await?;

	Ok(Paginated {
		items: from_db_vec(motivos),
		total,
		page,
		per_page,
	})
}

pub async fn insert<'a, E>(
	executor: E,
	motivo: intern::Motivo,
) -> Result<intern::Motivo, sqlx::Error>
where
	E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
	let motivo: database::Motivo = sqlx::query_as(queries::motivo::INSERT)
		.bind(&motivo.nome)
		.bind(motivo.artigo)
		.bind(motivo.paragrafo_unico)
		.bind(motivo.inciso)
		.fetch_one(executor)
		.await?;

	Ok(motivo.into())
}

// pub async fn remove_by_id<'a, E>(
// 	executor: E,
// 	id: i32,
// )
