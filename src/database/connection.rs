use sqlx::postgres::PgPoolOptions;

pub async fn establish_connection(database_url: &str) -> sqlx::Result<sqlx::Pool<sqlx::Postgres>> {

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_establish_connection_database(){
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("Provide the database connection url");
        assert!(establish_connection(&database_url).await.is_ok());
    }

}

