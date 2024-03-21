use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::common::Database;

use super::{data_access, User};

pub type UsersResponse = Vec<User>;
pub type UserResponse = User;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct UserCreateRequestParam {
    pub first: String,
    pub last: String,
}

// ex: curl -X GET http://localhost:3000/users
pub async fn index(State(_state): State<Database>) -> (StatusCode, Json<UsersResponse>) {
    let users: Vec<User> = data_access::select_all()
        .iter()
        .map(|user| user.clone().into())
        .collect();
    (StatusCode::OK, Json(users))
}

// ex: curl -X GET http://localhost:3000/users/1
pub async fn get(
    Path(id): Path<u64>,
    State(_state): State<Database>,
) -> (StatusCode, Json<Option<UserResponse>>) {
    let user = data_access::select(id as i32).ok().map(|user| user.into());

    match user {
        Some(user) => (StatusCode::OK, Json(Some(user))),
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}

// ex: curl -X POST -H "Content-Type: application/json" -d '{"first":"Alice","last":"Roberts"}' http://localhost:3000/users
pub async fn create(
    State(_state): State<Database>,
    Json(payload): Json<UserCreateRequestParam>,
) -> (StatusCode, Json<UserResponse>) {
    let dto = User {
        id: 0,
        first: payload.first.clone(),
        last: payload.last.clone(),
    };
    let _ = data_access::create(dto.clone().into());
    (StatusCode::CREATED, Json(dto.into()))
}

// ex: curl -X POST -H "Content-Type: application/json" -d '{"id": 1, "first":"Alice","last":"Roberts"}' http://localhost:3000/users/update
pub async fn update(
    State(_state): State<Database>,
    Json(payload): Json<User>,
) -> (StatusCode, Json<Option<UserResponse>>) {
    let is_updated = data_access::update(payload.clone().into()).is_ok();

    if is_updated {
        return (StatusCode::ACCEPTED, Json(Some(payload)));
    }
    (StatusCode::NOT_FOUND, Json(None))
}

// ex: curl -X POST -H "Content-Type: application/json" -d '1' http://localhost:3000/users/delete
pub async fn delete(
    State(_state): State<Database>,
    Json(payload): Json<u64>,
) -> (StatusCode, Json<Option<u64>>) {
    let is_delete = data_access::delete(payload as i32).is_ok();

    if is_delete {
        (StatusCode::ACCEPTED, Json(Some(payload as u64)))
    } else {
        (StatusCode::NOT_FOUND, Json(None))
    }
}
