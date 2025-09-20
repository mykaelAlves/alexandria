mod queries;

use crate::models::database;
use crate::models::intern;

pub mod motivo {
	use super::*;

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

		Ok(motivos.iter().map(|m| m.into()).collect())
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
}
