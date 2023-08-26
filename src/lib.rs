pub mod database;
pub mod api;

use api::{users::users_routes, expenses::expenses_routes};

pub async fn run(){
    
    let app = axum::Router::new()
        .nest("/api/users", users_routes())
        .nest("/api/expenses", expenses_routes());

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
