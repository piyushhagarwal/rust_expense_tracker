mod handlers;
use axum::{Router, routing::post};
use handlers::{signup,login};

pub fn users_routes() -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        // TODO: Add more routes as needed
}