use sqlx::postgres;

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
            sqlx::query_as("SELECT * FROM motivo")
                .fetch_all(executor)
                .await?;

        Ok(motivos.iter().map(|m| m.to_intern()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_all_motivos_doesnt_panic() {
        //let m = motivo::get_all().await;
    }
}
