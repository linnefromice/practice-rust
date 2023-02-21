use crate::table::Table;
use std::collections::HashMap;

pub struct WriteOnceTable<T>(HashMap<String, T>);

impl<T> WriteOnceTable<T> {
    pub fn new() -> WriteOnceTable<T> {
        WriteOnceTable(HashMap::new())
    }

    fn already_exists_guard(&self, name: &str) {
        if self.0.contains_key(name) {
            panic!("Error: redefining constant {} not allowed.", name);
        }
    }

    pub fn keys(&self) -> Vec<String> {
        let mut result = vec![];
        self.0.keys().for_each(|ref k| result.push(k.to_string()));
        result
    }
}

impl<T> Table for WriteOnceTable<T> {
    type Item = T;

    fn insert(&mut self, name: &str, value: Self::Item) {
        self.0.insert(name.to_string(), value);
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn contains_key(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    fn get(&self, name: &str) -> Option<&Self::Item> {
        self.0.get(name)
    }
}
