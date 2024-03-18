use axum::{
    routing::{get, post},
    Router,
};

use crate::common::Database;

mod common;
mod users;

#[tokio::main]
async fn main() {
    let db = Database::default();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/users", get(users::index))
        .route("/users/:id", get(users::get))
        .route("/users", post(users::create))
        .route("/users/update", post(users::update))
        .route("/users/delete", post(users::delete))
        .with_state(db);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// curl -X GET http://localhost:3000
// Hello, World!
async fn root() -> &'static str {
    "Hello, World!"
}
