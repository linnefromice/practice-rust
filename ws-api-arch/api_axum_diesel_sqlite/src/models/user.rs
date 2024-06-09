use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub first: String,
    pub last: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserDto {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: Option<i32>,
    pub dob: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]

pub struct UpdatableUserDto {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub age: Option<i32>,
    pub dob: Option<String>,
}
