use diesel::{deserialize::Queryable, AsChangeset, Insertable, Selectable};
use serde::{Deserialize, Serialize};

pub mod presenter;
pub use presenter::*;
pub mod data_access;
pub mod service;

// Database
#[derive(Clone, Debug, PartialEq, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserDto {
    id: i32,
    first_name: String,
    last_name: String,
    age: Option<i32>,
    dob: Option<String>,
}
impl From<User> for UserDto {
    fn from(user: User) -> Self {
        UserDto {
            id: user.id as i32,
            first_name: user.first,
            last_name: user.last,
            age: None,
            dob: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Insertable)]
#[diesel(table_name = crate::schema::users)]

pub struct InsertUserDto {
    first_name: String,
    last_name: String,
}
impl From<User> for InsertUserDto {
    fn from(user: User) -> Self {
        InsertUserDto {
            first_name: user.first,
            last_name: user.last,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    id: u64,
    first: String,
    last: String,
}
impl From<UserDto> for User {
    fn from(user: UserDto) -> Self {
        User {
            id: user.id as u64,
            first: user.first_name,
            last: user.last_name,
        }
    }
}
