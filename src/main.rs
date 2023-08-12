use rust_expense_tracker::{database, run};
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() {

    dotenv().ok();
    // Define the DATABASE_URL
    let database_url = env::var("DATABASE_URL").unwrap();

    match database::connection::establish_connection(&database_url).await {
        Ok(_pool) => {
            println!("Database connected");
            run().await;
        }
        Err(err) => {
            eprintln!("Failed to connect to the database: {}", err);
            // Handle the error or exit the application
        }
    }
}