use axum::{routing::get, Router};
use models::Database;

mod models;

#[tokio::main]
async fn main() {
    let db = Database::default();

    let app = Router::new().route("/", get(root)).with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// curl -X GET http://localhost:3000
async fn root() -> &'static str {
    "Hello, World!"
}
