use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::models::{user::User, Database};

// ex: curl -X GET http://localhost:3000/users
pub async fn index(State(state): State<Database>) -> (StatusCode, Json<Vec<User>>) {
    let data = &state.lock().unwrap().users;
    (StatusCode::OK, Json(data.clone()))
}

// ex: curl -X GET http://localhost:3000/users/1
pub async fn get(
    Path(id): Path<u64>,
    State(state): State<Database>,
) -> (StatusCode, Json<Option<User>>) {
    let data = &state.lock().unwrap().users;
    let user = data.iter().find(|u| u.id == id);
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
    data.uuid += 1;
    let user = User {
        id: data.uuid,
        first: param.first,
        last: param.last,
    };
    data.users.push(user.clone());
    (StatusCode::CREATED, Json(user))
}

// ex: curl -X POST -H "Content-Type: application/json" -d '{"id": 1,"first":"Alice","last":"Roberts"}' http://localhost:3000/users/update
pub async fn update(
    State(state): State<Database>,
    Json(param): Json<User>,
) -> (StatusCode, Json<Option<User>>) {
    let mut data = state.lock().unwrap();
    let user = data.users.iter_mut().find(|u| u.id == param.id);
    match user {
        Some(user) => {
            *user = param.clone();
            (StatusCode::OK, Json(Some(param)))
        }
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}

// ex: curl -X POST -H "Content-Type: application/json" -d '1' http://localhost:3000/users/delete
pub async fn delete(
    State(state): State<Database>,
    Json(param): Json<u64>,
) -> (StatusCode, Json<Option<User>>) {
    let mut data = state.lock().unwrap();
    let index = data.users.iter().position(|u| u.id == param);
    match index {
        Some(index) => {
            let user = data.users.remove(index);
            (StatusCode::OK, Json(Some(user)))
        }
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}
