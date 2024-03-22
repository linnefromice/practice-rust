use axum::{
    routing::{get, post},
    Router,
};
use models::Database;

mod models;
mod users;

#[tokio::main]
async fn main() {
    let db = Database::default();

    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(users::controller::index))
        .route("/users/:id", get(users::controller::get))
        .route("/users", post(users::controller::create))
        .route("/users/update", post(users::controller::update))
        .route("/users/delete", post(users::controller::delete))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// curl -X GET http://localhost:3000
async fn root() -> &'static str {
    "Hello, World!"
}
