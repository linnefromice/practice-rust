use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::common::Database;

use super::{service, User};

pub type UsersResponse = Vec<User>;
pub type UserResponse = User;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UserCreateRequestParam {
    pub first: String,
    pub last: String,
}

// ex: curl -X GET http://localhost:3000/users
pub async fn index(State(state): State<Database>) -> (StatusCode, Json<UsersResponse>) {
    let users = state.clone().lock().unwrap().clone().users;
    (StatusCode::OK, Json(users))
}

// ex: curl -X GET http://localhost:3000/users/1
pub async fn get(
    Path(id): Path<u64>,
    State(state): State<Database>,
) -> (StatusCode, Json<Option<UserResponse>>) {
    let users = state.clone().lock().unwrap().clone().users;
    let user = users.iter().find(|user| user.id == id).cloned();

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
    let mut data = state.lock().unwrap();
    let user = service::create_internal(&mut data, payload);
    (StatusCode::CREATED, Json(user))
}

// ex: curl -X POST -H "Content-Type: application/json" -d '{"id": 1, "first":"Alice","last":"Roberts"}' http://localhost:3000/users/update
pub async fn update(
    State(state): State<Database>,
    Json(payload): Json<User>,
) -> (StatusCode, Json<Option<UserResponse>>) {
    let is_updated = service::update_internal(&mut state.lock().unwrap(), payload.clone());

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
    let target = service::delete_internal(&mut state.lock().unwrap(), payload);

    if target.is_some() {
        (StatusCode::ACCEPTED, Json(target))
    } else {
        (StatusCode::NOT_FOUND, Json(None))
    }
}
