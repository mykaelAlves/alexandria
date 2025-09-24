mod queries;

use crate::models::database;
use crate::models::intern;

pub mod motivo {
	use crate::util::Paginated;

	use super::*;

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

		let motivos: Vec<database::Motivo> =
			sqlx::query_as(queries::motivo::LIST)
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
}

#[cfg(test)]
mod tests {
	use super::*;
	use sqlx::PgPool;

	#[sqlx::test(migrations = "./migrations")]
	async fn test_get_all_motivos(pool: PgPool) -> sqlx::Result<()> {
		let mut tx = pool.begin().await?;

		let before = motivo::get_all(&mut *tx).await?.len();

		sqlx::query(queries::motivo::INSERT)
			.bind("nome")
			.bind(1i16)
			.bind(true)
			.bind(3i16)
			.execute(&mut *tx)
			.await?;

		let after = motivo::get_all(&mut *tx).await?.len();

		assert_eq!(before + 1, after, "expected one more motivo after insert");

		Ok(())
	}

	#[sqlx::test(migrations = "./migrations")]
	async fn test_list_motivos(pool: PgPool) -> sqlx::Result<()> {
		let mut tx = pool.begin().await?;

		for i in 0..15 {
			let nome = format!("nome{}", i);
			sqlx::query(queries::motivo::INSERT)
				.bind(&nome)
				.bind(i as i16)
				.bind(true)
				.bind(i as i16)
				.execute(&mut *tx)
				.await?;
		}

		let paginated = motivo::list(&pool.clone(), 2, 5).await?;

		assert_eq!(paginated.items.len(), 5, "expected 5 motivos on page 2");
		assert_eq!(paginated.total, 15, "expected 15 total motivos");
		assert_eq!(paginated.page, 2, "expected page 2");
		assert_eq!(paginated.per_page, 5, "expected 5 per page");

		Ok(())
	}
}
