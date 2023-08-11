use sqlx::postgres::PgPoolOptions;

pub async fn establish_connection() -> sqlx::Result<sqlx::Pool<sqlx::Postgres>> {
    let database_url = "postgres://piyush:piyush@localhost:5432/rust";

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}