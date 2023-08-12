use rust_expense_tracker::{database, run};

#[tokio::main]
async fn main() {
    // Define the DATABASE_URL
    let database_url = "postgres://piyush:piyush@localhost:5432/rust";

    match database::connection::establish_connection(database_url).await {
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