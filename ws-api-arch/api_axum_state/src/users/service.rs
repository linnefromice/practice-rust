use crate::models::{user::User, StorageData};

pub fn index(state: &StorageData) -> Vec<User> {
    state.users.clone()
}

pub fn get(state: &StorageData, id: u64) -> Option<User> {
    state.users.iter().find(|u| u.id == id).cloned()
}

pub fn create(state: &mut StorageData, first: String, last: String) -> User {
    state.uuid += 1;
    let user = User {
        id: state.uuid,
        first,
        last,
    };
    state.users.push(user.clone());
    user
}

pub fn update(state: &mut StorageData, user: &User) -> Option<User> {
    let mut updated = None;
    state.users.iter_mut().for_each(|dto| {
        if user.id == dto.id {
            *dto = user.clone();
            updated = Some(user.clone());
        }
    });
    updated
}

pub fn delete(state: &mut StorageData, id: u64) -> Option<User> {
    let index = state.users.iter().position(|u| u.id == id);
    match index {
        Some(index) => Some(state.users.remove(index)),
        None => None,
    }
}
