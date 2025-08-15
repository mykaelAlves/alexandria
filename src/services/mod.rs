use sqlx::postgres;

use crate::models::database;
use crate::models::intern;

pub mod motivo {
    use super::*;

    pub async fn get_all(pool: postgres::PgPool) -> Vec<intern::Motivo> {
        let motivos: Vec<database::Motivo> =
            sqlx::query_as("SELECT * FROM motivo")
                .fetch_all(&pool)
                .await
                .unwrap();

        motivos.iter().map(|m| m.to_intern()).collect()
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
