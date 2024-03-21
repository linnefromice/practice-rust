use diesel::RunQueryDsl;

use crate::{common::establish_connection, schema::users};

use super::{InsertUserDto, UserDto};

pub fn select_all() -> Vec<UserDto> {
    let mut connection = establish_connection();
    let results = users::table
        .load::<UserDto>(&mut connection)
        .expect("Error loading users");

    results
}

pub fn create(datum: InsertUserDto) -> InsertUserDto {
    let mut connection = establish_connection();
    diesel::insert_into(users::table)
        .values(&datum)
        .execute(&mut connection)
        .expect("Error saving new user");
    datum
}
