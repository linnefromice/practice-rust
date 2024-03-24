use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

pub mod user;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StorageData {
    pub uuid: u64,
    pub users: Vec<user::User>,
}

pub type Database = Arc<Mutex<StorageData>>;
