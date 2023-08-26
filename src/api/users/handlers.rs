use axum::Json;
use serde::{Serialize, Deserialize};
use std::env;
use dotenv::dotenv;

use crate::database::connection::establish_connection;

#[derive(Serialize, Deserialize)]
pub struct UserData{
    username: String,
    password: String
}

pub async fn signup(user_data: Json<UserData>) -> Json<UserData>{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Provide the database connection url");
    let pool = establish_connection(&database_url).await.expect("Pool is not created properly");

    // Start a database transaction
    let tx = pool.begin().await.expect("Error in starting a database trasaction");

    // Insert the new expense into the expenses table
    let created_user = sqlx::query("INSERT INTO users (username,password) VALUES ($1, $2) returning *")
        .bind(user_data.username.to_string())
        .bind(user_data.password.to_string())
        .fetch_all(&pool)
        .await
        .expect("Error in inserting a user");

    Json(UserData { username: user_data.username.to_string(), password: user_data.password.to_string() })
}

pub async fn login(){
    
}