use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::common::Database;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    id: u64,
    first: String,
    last: String,
}

pub type UsersResponse = Vec<User>;
pub type UserResponse = User;

#[derive(Deserialize)]
pub struct UserCreateRequestParam {
    first: String,
    last: String,
}

// ex: curl -X GET http://localhost:3000/users
pub async fn index(State(state): State<Database>) -> (StatusCode, Json<UsersResponse>) {
    let data = state.clone().lock().unwrap().clone();
    (StatusCode::OK, Json(data))
}

// ex: curl -X GET http://localhost:3000/users/1
pub async fn get(
    Path(id): Path<u64>,
    State(state): State<Database>,
) -> (StatusCode, Json<Option<UserResponse>>) {
    let data = state.clone().lock().unwrap().clone();
    let user = data.iter().find(|user| user.id == id).cloned();

    match user {
        Some(user) => (StatusCode::OK, Json(Some(user))),
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}

// ex: curl -X POST -H "Content-Type: application/json" -d '{"first":"Alice","last":"Roberts"}' http://localhost:3000/users
pub async fn create(
    State(state): State<Database>,
    Json(payload): Json<UserCreateRequestParam>,
) -> (StatusCode, Json<UserResponse>) {
    let data_len = state.clone().lock().unwrap().len();
    let user = User {
        id: data_len as u64 + 1,
        first: payload.first,
        last: payload.last,
    };
    state.lock().unwrap().push(user.clone());

    (StatusCode::CREATED, Json(user))
}

// ex: curl -X POST -H "Content-Type: application/json" -d '{"id": 1, "first":"Alice","last":"Roberts"}' http://localhost:3000/users/update
pub async fn update(
    State(state): State<Database>,
    Json(payload): Json<User>,
) -> (StatusCode, Json<Option<UserResponse>>) {
    let mut is_updated = false;
    state.lock().unwrap().iter_mut().for_each(|user| {
        if user.id == payload.id {
            *user = payload.clone();
            is_updated = true;
        }
    });

    if is_updated {
        return (StatusCode::ACCEPTED, Json(Some(payload)));
    }
    (StatusCode::NOT_FOUND, Json(None))
}

// ex: curl -X POST -H "Content-Type: application/json" -d '1' http://localhost:3000/users/delete
pub async fn delete(
    State(state): State<Database>,
    Json(payload): Json<u64>,
) -> (StatusCode, Json<Option<UserResponse>>) {
    let data = state.clone().lock().unwrap().clone();
    let target = data.iter().find(|user| user.id == payload).cloned();

    if let Some(target) = target {
        state.lock().unwrap().retain(|user| user.id != payload);
        return (StatusCode::ACCEPTED, Json(Some(target)));
    }

    (StatusCode::NOT_FOUND, Json(None))
}
