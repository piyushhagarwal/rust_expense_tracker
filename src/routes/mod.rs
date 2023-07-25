use axum::{Router, routing::{get, post, delete, patch}};
mod expense;

pub fn create_routes() -> Router<>{
    Router::new()
        .route("/createuser", post(expense::create_user))
        .route("/:user_id/addbalance", post(expense::add_balance))
        .route("/:user_id", get(expense::get_user))
        .route("/:user_id/addexpense", post(expense::add_expense))
        .route("/:user_id/deleteexpense", delete(expense::delete_expense))
        .route("/:user_id/updateexpense", patch(expense::update_expense))
}

