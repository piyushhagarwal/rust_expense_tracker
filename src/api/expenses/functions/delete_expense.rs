use sqlx::PgPool;

pub async fn delete_expense_function(pool: &PgPool, user_id: i32, expense_id: i32) -> sqlx::Result<()>{
    // Start a database transaction
    let tx = pool.begin().await?;

    //To get previous amount of the expense
    let previous_expense_amount: f64 = sqlx::query_scalar("SELECT amount FROM expenses WHERE id = $1")
        .bind(expense_id)
        .fetch_one(pool)
        .await?;

    // Insert the new expense into the expenses table
    sqlx::query("DELETE FROM expenses WHERE user_id = $1 AND id = $2")
        .bind(user_id)
        .bind(expense_id)
        .execute(pool)
        .await?;

    // Update the user's balance in the users table
    sqlx::query!(
        "UPDATE users SET balance = balance - $1 WHERE id = $2",
        previous_expense_amount,
        user_id
    )
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
    async fn test_delete_expense(){
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("Provide the database connection url");
        let pool = establish_connection(&database_url).await.expect("Pool is not created properly");
        delete_expense_function(&pool, 1, 2).await.unwrap();
    }
}