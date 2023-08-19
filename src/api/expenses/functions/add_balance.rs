use sqlx::PgPool;

pub async fn add_balance_function(pool: &PgPool, user_id: i32, amount: f64) -> sqlx::Result<()>{
    //Start a database transaction
    let tx = pool.begin().await?;

    let balance: f64 = sqlx::query_scalar("SELECT balance FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    // Calculate the new balance after adding the amount
    let new_balance = balance + amount;

    sqlx::query("UPDATE users SET balance = $1 WHERE id = $2")
        .bind(new_balance)
        .bind(user_id)
        .execute(pool)
        .await?;

    // Commit the transaction
    tx.commit().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use dotenv::dotenv;
    use crate::database::connection::establish_connection;

    #[tokio::test]
    async fn test_add_balance(){
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("Provide the database connection url");
        let pool = establish_connection(&database_url).await.expect("Pool is not created properly");
        add_balance_function(&pool, 1, 100.0).await.unwrap();
    }

}