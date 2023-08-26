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

// use rust_expense_tracker::{database::connection::establish_connection, api::expenses::functions::add_expense::add_expense_function};

// #[tokio::main]
// async fn main() {
//     dotenv().ok();

//     // Number of concurrent tasks you want to spawn
//     let num_tasks = 50;

//     // Create a vector to hold the task handles
//     let mut handles = vec![];

//     for i in 0..num_tasks {
//         // Clone the DATABASE_URL for each task
//         let database_url = env::var("DATABASE_URL").expect("Provide the database connection url");

//         // Spawn a new Tokio task
//         let handle = tokio::spawn(async move {
//             println!("{i}");
//             test_add_expense(&database_url).await;
//         });

//         handles.push(handle);
//     }

//     // Wait for all tasks to finish
//     for handle in handles {
//         handle.await.expect("Task panicked");
//     }
// }

// async fn test_add_expense(database_url: &str) {
//     let pool = establish_connection(database_url).await.expect("Pool is not created properly");
//     // Perform the operation using the local pool
//     add_expense_function(&pool, 1, 200.0, "transportation".to_string(), "Travel".to_string()).await.unwrap();
// }
