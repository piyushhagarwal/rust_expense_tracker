use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    // Define the DATABASE_URL
    let database_url = "postgres://piyush:piyush@localhost:5432/rust";

    // Create a connection pool to the PostgreSQL database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Database connected");
}