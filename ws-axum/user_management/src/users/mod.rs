use axum::{extract::Path, http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    id: u64,
    first: String,
    last: String,
}

pub type UsersResponse = Vec<User>;
pub type UserResponse = User;

#[derive(serde::Deserialize)]
pub struct UserCreateRequestParam {
    first: String,
    last: String,
}

// ex: curl -X GET http://localhost:3000/users
pub async fn index() -> (StatusCode, Json<UsersResponse>) {
    (StatusCode::OK, Json(dummy()))
}

// ex: curl -X GET http://localhost:3000/users/1
pub async fn get(Path(id): Path<u64>) -> (StatusCode, Json<Option<UserResponse>>) {
    let user = dummy().into_iter().find(|user| user.id == id);

    match user {
        Some(user) => (StatusCode::OK, Json(Some(user))),
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}

// ex: curl -X POST -H "Content-Type: application/json" -d '{"first":"Alice","last":"Roberts"}' http://localhost:3000/users
pub async fn create(
    Json(payload): Json<UserCreateRequestParam>,
) -> (StatusCode, Json<UserResponse>) {
    let user = User {
        id: dummy().len() as u64 + 1,
        first: payload.first,
        last: payload.last,
    };

    (StatusCode::CREATED, Json(user))
}

fn dummy() -> Vec<User> {
    vec![
        User {
            id: 1,
            first: "John".to_string(),
            last: "Smith".to_string(),
        },
        User {
            id: 2,
            first: "Jane".to_string(),
            last: "Doe".to_string(),
        },
        User {
            id: 3,
            first: "Emily".to_string(),
            last: "Jones".to_string(),
        },
        User {
            id: 4,
            first: "Michael".to_string(),
            last: "Johnson".to_string(),
        },
        User {
            id: 5,
            first: "Emma".to_string(),
            last: "Brown".to_string(),
        },
    ]
}
