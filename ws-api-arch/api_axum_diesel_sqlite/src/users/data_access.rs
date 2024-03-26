use diesel::{QueryDsl, RunQueryDsl};

use crate::{
    database::establish_connection,
    models::user::{UpdatableUserDto, UserDto},
    schema::users,
};

pub fn index() -> Vec<UserDto> {
    let mut connection = establish_connection();

    let results = users::table
        .load::<UserDto>(&mut connection)
        .expect("Error loading users");

    results
}

pub fn get(id: u64) -> UserDto {
    let mut connection = establish_connection();

    let result = users::table
        .find(id as i32)
        .first::<UserDto>(&mut connection)
        .expect("Error loading user");

    result
}

pub fn create(first: String, last: String) -> UpdatableUserDto {
    let mut connection = establish_connection();

    let new_user = UpdatableUserDto {
        id: None,
        first_name: first,
        last_name: last,
        age: None,
        dob: None,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut connection)
        .expect("Error saving new user");

    new_user
}

pub fn update(user: &UpdatableUserDto) -> &UpdatableUserDto {
    let mut connection = establish_connection();

    // todo: update `updated_at` field

    diesel::update(users::table.find(user.id.unwrap() as i32))
        .set(user)
        .execute(&mut connection)
        .expect("Error updating user");

    user
}

pub fn delete(id: i32) -> i32 {
    let mut connection = establish_connection();

    diesel::delete(users::table.find(id))
        .execute(&mut connection)
        .expect("Error deleting user");

    id
}
