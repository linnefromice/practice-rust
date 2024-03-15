use axum::{http::StatusCode, routing::{get, post}, Json, Router};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

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

// curl -X POST http://localhost:3000/users -d '{"name":"Alice"}' -H 'Content-Type: application/json'
// {"id":42,"name":"Alice"}
async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 42,
        name: payload.name,
    };
    (
        StatusCode::CREATED,
        Json(user)
    )
}

#[derive(serde::Deserialize)]
struct CreateUser {
    name: String,
}

#[derive(serde::Serialize)]
struct User {
    id: u64,
    name: String,
}
