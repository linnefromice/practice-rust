use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::models::{
    user::{UpdatableUserDto, User, UserDto},
    Database,
};

use super::data_access;

// ex: curl -X GET http://localhost:3000/users
pub async fn index(State(_state): State<Database>) -> (StatusCode, Json<Vec<UserDto>>) {
    // let data = service::index(&state.lock().unwrap());
    let data = data_access::index();
    (StatusCode::OK, Json(data))
}

// ex: curl -X GET http://localhost:3000/users/1
pub async fn get(
    Path(id): Path<u64>,
    State(_state): State<Database>,
) -> (StatusCode, Json<Option<UserDto>>) {
    // let user = service::get(&state.lock().unwrap(), id);
    // match user {
    //     Some(user) => (StatusCode::OK, Json(Some(user.clone()))),
    //     None => (StatusCode::NOT_FOUND, Json(None)),
    // }
    let data = data_access::get(id);
    (StatusCode::OK, Json(Some(data)))
}

// ex: curl -X POST -H "Content-Type: application/json" -d '{"first":"Alice","last":"Roberts"}' http://localhost:3000/users
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct CreateParam {
    pub first: String,
    pub last: String,
}
pub async fn create(
    State(_state): State<Database>,
    Json(param): Json<CreateParam>,
) -> (StatusCode, Json<UpdatableUserDto>) {
    // let mut data = state.lock().unwrap();
    // let created = service::create(&mut data, param.first, param.last);
    let created = data_access::create(param.first, param.last);
    (StatusCode::CREATED, Json(created))
}

// ex: curl -X POST -H "Content-Type: application/json" -d '{"id": 1,"first":"Alice","last":"Roberts"}' http://localhost:3000/users/update
pub async fn update(
    State(_state): State<Database>,
    Json(param): Json<User>,
) -> (StatusCode, Json<Option<User>>) {
    // let mut data = state.lock().unwrap();
    // let user = service::update(&mut data, &param);
    // match user {
    //     Some(_) => (StatusCode::OK, Json(Some(param))),
    //     None => (StatusCode::NOT_FOUND, Json(None)),
    // }
    let data = UpdatableUserDto {
        id: Some(param.id as i32),
        first_name: param.first.clone(),
        last_name: param.last.clone(),
        age: None,
        dob: None,
    };
    let _ = data_access::update(&data);
    (StatusCode::OK, Json(Some(param)))
}

// ex: curl -X POST -H "Content-Type: application/json" -d '1' http://localhost:3000/users/delete
pub async fn delete(
    State(_state): State<Database>,
    Json(param): Json<u64>,
) -> (StatusCode, Json<Option<u64>>) {
    // let mut data = state.lock().unwrap();
    // let user = service::delete(&mut data, param);
    // match user {
    //     Some(user) => (StatusCode::OK, Json(Some(user))),
    //     None => (StatusCode::NOT_FOUND, Json(None)),
    // }
    let _ = data_access::delete(param as i32);
    (StatusCode::OK, Json(Some(param)))
}
