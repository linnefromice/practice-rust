use std::{
    env,
    sync::{Arc, Mutex},
};

use diesel::{Connection, SqliteConnection};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};

use crate::users::User;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StorageData {
    pub uuid: u64,
    pub users: Vec<User>,
}

pub type Database = Arc<Mutex<StorageData>>;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
