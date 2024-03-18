use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::users::User;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StorageData {
    pub uuid: u64,
    pub users: Vec<User>,
}

pub type Database = Arc<Mutex<StorageData>>;
