mod handlers;
use axum::{Router, routing::{post, patch, delete, get}};
use handlers::{add_balance,get_expenses,get_single_expense,add_expense,delete_expense,update_expense};

pub fn expenses_routes() -> Router {
    Router::new()
        .route("/:user_id", get(get_expenses))
        .route("/:user_id/:expense_id",get(get_single_expense))
        .route("/:user_id/addbalance", post(add_balance))
        .route("/:user_id/addexpense", post(add_expense))
        .route("/:user_id/:expense_id", delete(delete_expense))
        .route("/:user_id/:expense_id", patch(update_expense))
        // TODO: Add more routes as needed
}