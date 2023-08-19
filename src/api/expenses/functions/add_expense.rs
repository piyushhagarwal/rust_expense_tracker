use sqlx::PgPool;

pub async fn add_expense_function(pool: &PgPool, user_id: i32, amount: f64, expense_name: String, expense_category: String) -> sqlx::Result<()>{
    // Start a database transaction
    let tx = pool.begin().await?;

    // Insert the new expense into the expenses table
    sqlx::query("INSERT INTO expenses (user_id, amount, expense_name, expense_category) VALUES ($1, $2, $3, $4)")
        .bind(user_id)
        .bind(amount)
        .bind(expense_name)
        .bind(expense_category)
        .execute(pool)
        .await?;

    // Update the user's balance in the users table
    sqlx::query!(
        "UPDATE users SET balance = balance + $1 WHERE id = $2",
        amount,
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
    async fn test_add_expense(){
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("Provide the database connection url");
        let pool = establish_connection(&database_url).await.expect("Pool is not created properly");
        add_expense_function(&pool, 1, 200.0,"transportation".to_string(),"Travel".to_string()).await.unwrap();
    }
}