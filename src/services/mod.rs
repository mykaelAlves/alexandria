use sqlx::postgres;

use crate::models::database;
use crate::models::intern;
use crate::config::SERVER_APP;

async fn connect_db() -> postgres::PgPool {
    let database_url = SERVER_APP.get_database_url();
    let max_conn = SERVER_APP.get_max_connections();
    let pool = postgres::PgPoolOptions::new()
        .max_connections(max_conn.into())
        .connect(&database_url)
        .await
        .unwrap();

    pool
}

pub mod motivo {
    use super::*;

    pub async fn get_all() -> Vec<intern::Motivo> {
        let pool = connect_db().await;
        
        let motivos: Vec<database::Motivo> = sqlx::query_as("SELECT * FROM Motivo")
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
        let m = motivo::get_all().await;

        println!("{:?}", m);
    }
}
