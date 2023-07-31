pub mod filesystem;

pub mod routes;

pub mod user;

pub async fn run(){
    // build our application with a single route
    let app = routes::create_routes();

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

