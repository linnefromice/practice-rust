use serde::{Deserialize, Serialize};

pub mod presenter;
pub use presenter::*;
pub mod service;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    id: u64,
    first: String,
    last: String,
}
