use sqlx::postgres;

mod queries;

use crate::models::database;
use crate::models::intern;

pub mod motivo {
	use super::*;

	pub async fn get_all<'a, E>(
		executor: E,
	) -> Result<Vec<intern::Motivo>, sqlx::Error>
	where
		E: postgres::PgExecutor<'a>,
	{
		let motivos: Vec<database::Motivo> =
			sqlx::query_as(queries::GET_ALL_MOTIVOS)
				.fetch_all(executor)
				.await?;

		Ok(motivos.iter().map(|m| m.into()).collect())
	}
}

#[cfg(test)]
mod tests {

}
