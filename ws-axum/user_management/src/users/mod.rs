use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    id: u64,
    first: String,
    last: String,
}

pub type UsersResponse = Vec<User>;

pub async fn index() -> (StatusCode, Json<UsersResponse>) {
    (StatusCode::OK, Json(dummy()))
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
