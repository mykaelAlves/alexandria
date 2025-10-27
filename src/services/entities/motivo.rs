use crate::models::database;
use crate::models::database::MotivoId;
use crate::models::intern;
use crate::services::queries;
use crate::util::{Paginated, capitalize_words};

fn from_db_vec(motivos_db: Vec<database::Motivo>) -> Vec<intern::Motivo> {
	motivos_db.into_iter().map(|m| m.into()).collect()
}

#[must_use]
pub async fn count_all<'a, E>(executor: E) -> Result<i64, sqlx::Error>
where
	E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
	let result: (i64,) = sqlx::query_as(queries::motivo::COUNT_ALL)
		.fetch_one(executor)
		.await?;

	Ok(result.0)
}

#[must_use]
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

#[must_use]
pub async fn get_n<'a, E>(
	executor: E,
	n: usize,
) -> Result<Vec<intern::Motivo>, sqlx::Error>
where
	E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
	let motivos: Vec<database::Motivo> = sqlx::query_as(queries::motivo::GET_N)
		.bind(n as i64)
		.fetch_all(executor)
		.await?;

	Ok(from_db_vec(motivos))
}

#[must_use]
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

#[must_use]
pub async fn get_by_nome<'a, E>(
	executor: E,
	nome: &str,
) -> Result<Option<intern::Motivo>, sqlx::Error>
where
	E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
	let motivo: Option<database::Motivo> =
		sqlx::query_as(queries::motivo::GET_BY_NOME)
			.bind(capitalize_words(nome))
			.fetch_optional(executor)
			.await?;

	Ok(motivo.map(|m| m.into()))
}

#[must_use]
pub async fn get_by_id<'a, E>(
	executor: E,
	id: MotivoId,
) -> Result<Option<intern::Motivo>, sqlx::Error>
where
	E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
	let motivo: Option<database::Motivo> =
		sqlx::query_as(queries::motivo::GET_BY_ID)
			.bind(id)
			.fetch_optional(executor)
			.await?;

	Ok(motivo.map(|m| m.into()))
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

pub async fn delete_by_id<'a, E>(
	executor: E,
	id: MotivoId,
) -> Result<String, sqlx::Error>
where
	E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
	let result: (String,) = sqlx::query_as(queries::motivo::DELETE_BY_ID)
		.bind(id)
		.fetch_one(executor)
		.await?;

	Ok(result.0)
}

pub async fn delete_by_nome<'a, E>(
	executor: E,
	nome: &str,
) -> Result<(), sqlx::Error>
where
	E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
	sqlx::query(queries::motivo::DELETE_BY_NOME)
		.bind(nome)
		.execute(executor)
		.await?;

	Ok(())
}

pub async fn update_by_id<'a, E>(
	executor: E,
	id: MotivoId,
	motivo: intern::Motivo,
) -> Result<(), sqlx::Error>
where
	E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
	sqlx::query(queries::motivo::UPDATE_BY_ID)
		.bind(&motivo.nome)
		.bind(motivo.artigo)
		.bind(motivo.paragrafo_unico)
		.bind(motivo.inciso)
		.bind(id)
		.execute(executor)
		.await?;

	Ok(())
}

pub async fn update_by_nome<'a, E>(
	executor: E,
	nome: &str,
	motivo: intern::Motivo,
) -> Result<(), sqlx::Error>
where
	E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
	sqlx::query(queries::motivo::UPDATE_BY_NOME)
		.bind(&motivo.nome)
		.bind(motivo.artigo)
		.bind(motivo.paragrafo_unico)
		.bind(motivo.inciso)
		.bind(nome)
		.execute(executor)
		.await?;

	Ok(())
}

#[cfg(test)]
mod tests {
	use sqlx::PgPool;

	use super::*;

	#[sqlx::test(migrations = "./migrations")]
	async fn test_count_all_empty(pool: PgPool) -> sqlx::Result<()> {
		let mut conn = pool.acquire().await?;

		let count = count_all(&mut *conn).await?;
		assert_eq!(count, 0);
		Ok(())
	}

	#[sqlx::test(migrations = "./migrations")]
	async fn test_count_all_twenty(pool: PgPool) -> sqlx::Result<()> {
		for i in 1..=20 {
			let motivo = intern::Motivo {
				nome: format!("Motivo {}", i),
				artigo: i as i16,
				paragrafo_unico: i % 2 == 0,
				inciso: if i % 3 == 0 { Some(i as i16) } else { None },
			};
			insert(&pool, motivo).await?;
		}
		let mut conn = pool.acquire().await?;
		let count = count_all(&mut *conn).await?;
		assert_eq!(count, 20);
		Ok(())
	}

	#[sqlx::test(migrations = "./migrations")]
	async fn test_get_n_empty(pool: PgPool) -> sqlx::Result<()> {
		let mut conn = pool.acquire().await?;

		let motivo = get_by_id(&mut *conn, MotivoId(1)).await?;
		assert!(motivo.is_none());
		Ok(())
	}

	#[sqlx::test(migrations = "./migrations")]
	async fn test_get_by_id_empty(pool: PgPool) -> sqlx::Result<()> {
		let mut conn = pool.acquire().await?;

		let motivo = get_by_id(&mut *conn, MotivoId(1)).await?;
		assert!(motivo.is_none());
		Ok(())
	}

	#[sqlx::test(migrations = "./migrations")]
	async fn test_get_all_empty(pool: PgPool) -> sqlx::Result<()> {
		let mut conn = pool.acquire().await?;

		let motivos = get_all(&mut *conn).await?;
		assert!(motivos.is_empty());
		Ok(())
	}

	#[sqlx::test(migrations = "./migrations")]
	async fn test_get_all_twenty(pool: PgPool) -> sqlx::Result<()> {
		for i in 1..=20 {
			let motivo = intern::Motivo {
				nome: format!("Motivo {}", i),
				artigo: i as i16,
				paragrafo_unico: i % 2 == 0,
				inciso: if i % 3 == 0 { Some(i as i16) } else { None },
			};
			insert(&pool, motivo).await?;
		}

		let motivos = get_all(&pool).await?;
		assert_eq!(motivos.len(), 20);
		Ok(())
	}

	#[sqlx::test(migrations = "./migrations")]
	async fn test_insert_and_get_by_nome(pool: PgPool) -> sqlx::Result<()> {
		let motivo = intern::Motivo {
			nome: "Teste".to_string(),
			artigo: 5,
			paragrafo_unico: true,
			inciso: Some(2),
		};
		insert(&pool, motivo.clone()).await?;

		let mut conn = pool.acquire().await?;
		let fetched_motivo = get_by_nome(&mut *conn, "Teste").await?;
		assert!(fetched_motivo.is_some());
		let fetched_motivo = fetched_motivo.unwrap();
		assert_eq!(fetched_motivo.nome, motivo.nome);
		assert_eq!(fetched_motivo.artigo, motivo.artigo);
		assert_eq!(fetched_motivo.paragrafo_unico, motivo.paragrafo_unico);
		assert_eq!(fetched_motivo.inciso, motivo.inciso);
		Ok(())
	}

	#[sqlx::test(migrations = "./migrations")]
	async fn test_update_by_id(pool: PgPool) -> sqlx::Result<()> {
		let motivo = intern::Motivo {
			nome: "Original".to_string(),
			artigo: 1,
			paragrafo_unico: false,
			inciso: None,
		};
		insert(&pool, motivo).await?;

		let updated_motivo = intern::Motivo {
			nome: "Atualizado".to_string(),
			artigo: 2,
			paragrafo_unico: true,
			inciso: Some(1),
		};
		update_by_id(&pool, MotivoId(1), updated_motivo.clone()).await?;

		let mut conn = pool.acquire().await?;
		let fetched_motivo = get_by_id(&mut *conn, MotivoId(1)).await?;
		assert!(fetched_motivo.is_some());
		let fetched_motivo = fetched_motivo.unwrap();
		assert_eq!(fetched_motivo.nome, updated_motivo.nome);
		assert_eq!(fetched_motivo.artigo, updated_motivo.artigo);
		assert_eq!(
			fetched_motivo.paragrafo_unico,
			updated_motivo.paragrafo_unico
		);
		assert_eq!(fetched_motivo.inciso, updated_motivo.inciso);
		Ok(())
	}
}
