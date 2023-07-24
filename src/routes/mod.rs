use axum::{Router, routing::{get, post}};
mod expense;

pub fn create_routes() -> Router<>{
    Router::new()
        .route("/createuser", post(expense::create_user))
        .route("/addbalance", post(expense::add_balance))
}

