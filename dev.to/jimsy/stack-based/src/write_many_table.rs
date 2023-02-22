use crate::table::Table;
use std::collections::HashMap;

pub struct WriteManyTable<T>(HashMap<String, T>);

impl<T> WriteManyTable<T> {
    pub fn new() -> WriteManyTable<T> {
        WriteManyTable(HashMap::new())
    }
}

impl<T> Table for WriteManyTable<T> {
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
#[cfg(test)]
mod test {
    use crate::table::Table;

    use super::WriteManyTable;

    #[test]
    fn new() {
        let write_many_table: WriteManyTable<usize> = WriteManyTable::new();
        assert!(write_many_table.is_empty());
    }

    #[test]
    fn insert() {
        let mut write_many_table: WriteManyTable<usize> = WriteManyTable::new();
        write_many_table.insert("example", 13);
        assert!(!write_many_table.is_empty());
        assert!(write_many_table.contains_key("example"));
        assert_eq!(*write_many_table.get("example").unwrap(), 13);
    }

    #[test]
    fn insert_uniq() {
        let mut write_many_table: WriteManyTable<usize> = WriteManyTable::new();
        write_many_table.insert("example", 13);
        assert_eq!(*write_many_table.get("example").unwrap(), 13);
        write_many_table.insert("example", 14);
        assert_eq!(*write_many_table.get("example").unwrap(), 14);
    }
}
