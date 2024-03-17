use std::sync::{Arc, Mutex};

use crate::users::User;

pub type Database = Arc<Mutex<Vec<User>>>;
