use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::models::{user::User, Database};

use super::service;

// ex: curl -X GET http://localhost:3000/users
pub async fn index(State(state): State<Database>) -> (StatusCode, Json<Vec<User>>) {
    let data = service::index(&state.lock().unwrap());
    (StatusCode::OK, Json(data))
}

// ex: curl -X GET http://localhost:3000/users/1
pub async fn get(
    Path(id): Path<u64>,
    State(state): State<Database>,
) -> (StatusCode, Json<Option<User>>) {
    let user = service::get(&state.lock().unwrap(), id);
    match user {
        Some(user) => (StatusCode::OK, Json(Some(user.clone()))),
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}

// ex: curl -X POST -H "Content-Type: application/json" -d '{"first":"Alice","last":"Roberts"}' http://localhost:3000/users
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct CreateParam {
    pub first: String,
    pub last: String,
}
pub async fn create(
    State(state): State<Database>,
    Json(param): Json<CreateParam>,
) -> (StatusCode, Json<User>) {
    let mut data = state.lock().unwrap();
    let created = service::create(&mut data, param.first, param.last);
    (StatusCode::CREATED, Json(created))
}

// ex: curl -X POST -H "Content-Type: application/json" -d '{"id": 1,"first":"Alice","last":"Roberts"}' http://localhost:3000/users/update
pub async fn update(
    State(state): State<Database>,
    Json(param): Json<User>,
) -> (StatusCode, Json<Option<User>>) {
    let mut data = state.lock().unwrap();
    let user = service::update(&mut data, &param);
    match user {
        Some(_) => (StatusCode::OK, Json(Some(param))),
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}

// ex: curl -X POST -H "Content-Type: application/json" -d '1' http://localhost:3000/users/delete
pub async fn delete(
    State(state): State<Database>,
    Json(param): Json<u64>,
) -> (StatusCode, Json<Option<User>>) {
    let mut data = state.lock().unwrap();
    let user = service::delete(&mut data, param);
    match user {
        Some(user) => (StatusCode::OK, Json(Some(user))),
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}
