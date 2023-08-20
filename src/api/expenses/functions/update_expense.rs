use sqlx::PgPool;

pub async fn update_expense_function(pool: &PgPool, user_id: i32, expense_id: i32, amount: f64, expense_name: String, expense_category: String) -> sqlx::Result<()>{
    // Start a database transaction
    let tx = pool.begin().await?;

    //To get the balance of the user
    let balance: f64 = sqlx::query_scalar("SELECT balance FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    //To get previous amount of the expense
    let previous_expense_amount: f64 = sqlx::query_scalar("SELECT amount FROM expenses WHERE id = $1")
        .bind(expense_id)
        .fetch_one(pool)
        .await?;

    let updated_balance = balance - previous_expense_amount + amount;

    // Update the expense
    sqlx::query(
        "UPDATE expenses SET amount = $1,expense_name = $2,expense_category = $3 WHERE id = $4")
        .bind(amount)
        .bind(expense_name)
        .bind(expense_category)
        .bind(expense_id)
        .execute(pool)
        .await?;

    // Update the user's balance in the users table
    sqlx::query("UPDATE users SET balance = $1 WHERE id = $2")
        .bind(updated_balance)
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
    async fn test_update_expense(){
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("Provide the database connection url");
        let pool = establish_connection(&database_url).await.expect("Pool is not created properly");
        update_expense_function(&pool,1, 2, 100.0,"transportation".to_string(),"Travel".to_string()).await.unwrap();
    }
}