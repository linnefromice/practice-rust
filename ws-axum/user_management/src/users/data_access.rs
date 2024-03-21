use diesel::{QueryDsl, RunQueryDsl};

use crate::{common::establish_connection, schema::users};

use super::{InsertUserDto, UserDto};

pub fn select_all() -> Vec<UserDto> {
    let mut connection = establish_connection();
    let results = users::table
        .load::<UserDto>(&mut connection)
        .expect("Error loading users");

    results
}

pub fn select(id: i32) -> Result<UserDto, diesel::result::Error> {
    let mut connection = establish_connection();
    let result = users::table.find(id).first::<UserDto>(&mut connection);

    result
}

pub fn create(datum: InsertUserDto) -> Result<InsertUserDto, diesel::result::Error> {
    let mut connection = establish_connection();
    let res = diesel::insert_into(users::table)
        .values(&datum)
        .execute(&mut connection);
    return match res {
        Ok(_) => Ok(datum),
        Err(e) => Err(e),
    };
}

pub fn update(datum: UserDto) -> Result<UserDto, diesel::result::Error> {
    let mut connection = establish_connection();
    let res = diesel::update(users::table.find(datum.id))
        .set(&datum)
        .execute(&mut connection);
    return match res {
        Ok(_) => Ok(datum),
        Err(e) => Err(e),
    };
}

pub fn delete(id: i32) -> Result<(), diesel::result::Error> {
    let mut connection = establish_connection();
    let res = diesel::delete(users::table.find(id)).execute(&mut connection);
    return match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    };
}
