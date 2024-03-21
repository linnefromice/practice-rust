use crate::common::StorageData;

use super::{User, UserCreateRequestParam};

#[allow(dead_code)]
pub fn create_internal(state: &mut StorageData, payload: UserCreateRequestParam) -> User {
    let user = User {
        id: state.uuid as u64 + 1,
        first: payload.first,
        last: payload.last,
    };
    state.users.push(user.clone());
    state.uuid += 1;
    user
}

#[allow(dead_code)]
pub fn update_internal(state: &mut StorageData, payload: User) -> bool {
    let mut is_updated = false;
    state.users.iter_mut().for_each(|user| {
        if user.id == payload.id {
            *user = payload.clone();
            is_updated = true;
        }
    });
    is_updated
}

#[allow(dead_code)]
pub fn delete_internal(state: &mut StorageData, payload: u64) -> Option<User> {
    let target = state.users.iter().find(|user| user.id == payload).cloned();

    if let Some(target) = target {
        state.users.retain(|user| user.id != payload);
        return Some(target);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data() -> StorageData {
        StorageData {
            uuid: 2,
            users: vec![
                User {
                    id: 1,
                    first: "Alice".to_string(),
                    last: "Roberts".to_string(),
                },
                User {
                    id: 2,
                    first: "Bob".to_string(),
                    last: "Smith".to_string(),
                },
            ],
        }
    }

    #[test]
    fn test_create_internal() {
        let mut state = data();
        let payload = UserCreateRequestParam {
            first: "Charlie".to_string(),
            last: "Brown".to_string(),
        };
        let user = create_internal(&mut state, payload.clone());
        assert_eq!(user.id, 3);
        assert_eq!(user.first, payload.first);
        assert_eq!(user.last, payload.last);
        assert_eq!(state.uuid, 3);
        assert_eq!(state.users.len(), 3);
        assert_eq!(state.users.last().unwrap(), &user);
    }

    #[test]
    fn test_update_internal() {
        let mut state = data();
        let payload = User {
            id: 2,
            first: "Charlie".to_string(),
            last: "Brown".to_string(),
        };
        let is_updated = update_internal(&mut state, payload.clone());
        assert_eq!(is_updated, true);
        assert_eq!(state.users.len(), 2);
        let updated = state
            .users
            .iter()
            .find(|user| user.id == payload.id)
            .unwrap();
        assert_eq!(updated, &payload);
    }

    #[test]
    fn test_delete_internal() {
        let mut state = data();
        let payload = 2;
        let target = delete_internal(&mut state, payload);
        assert_eq!(target.is_some(), true);
        assert_eq!(state.users.len(), 1);
        assert_eq!(state.users.iter().find(|user| user.id == payload), None);
    }
}
