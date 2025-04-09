use std::{
    collections::{BTreeMap, HashMap},
    sync::{Arc, Mutex},
};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum StorageError {
    #[error("Failed to acquire mutex lock")]
    MutexLock,
    #[error("Column family not found")]
    CFKeyNotFound,
    #[error("Key not found in the specified column family")]
    KeyNotFound,
    #[error("Failed to insert value")]
    InsertionFailed,
}

/// Using BTreeMap to support range queries within a CF.
/// Howeve, BTreeMap will yield slower insertions and deletions when compared to HashMap.
#[derive(Default, Debug)]
pub struct Storage(Arc<Mutex<HashMap<String, BTreeMap<String, String>>>>);

impl Storage {
    /// replaces the value for a particular key for the specified CF in the database
    pub fn put(&self, column_family: &str, k: &str, v: &str) -> Result<(), StorageError> {
        let mut storage = self.0.lock().map_err(|_| StorageError::MutexLock)?;
        storage
            .entry(column_family.to_string())
            .or_insert(BTreeMap::new())
            .insert(k.to_string(), v.to_string());
        Ok(())
    }
    /// fetches the current value for a key for the specified CF
    pub fn get(&self, column_family: &str, k: &str) -> Result<String, StorageError> {
        let storage = self.0.lock().map_err(|_| StorageError::MutexLock)?;
        let family = storage
            .get(column_family)
            .ok_or(StorageError::CFKeyNotFound)?;
        let found_value = family.get(k).ok_or(StorageError::KeyNotFound)?;
        Ok(found_value.clone())
    }

    /// deletes the key's value for the specified CF
    pub fn delete(&self, column_family: &str, k: &str) -> Result<String, StorageError> {
        let mut storage = self.0.lock().map_err(|_| StorageError::MutexLock)?;
        let family = storage
            .get_mut(column_family)
            .ok_or(StorageError::CFKeyNotFound)?;
        let removed_value = family.remove(k).ok_or(StorageError::KeyNotFound)?;
        Ok(removed_value)
    }

    /// fetches the current value for a series of keys for the specified CF
    pub fn scan(
        &self,
        column_family: &str,
        start_k: &str,
        limit: usize,
    ) -> Result<Vec<(String, String)>, StorageError> {
        let storage = self.0.lock().map_err(|_| StorageError::MutexLock)?;
        let family = storage
            .get(column_family)
            .ok_or(StorageError::CFKeyNotFound)?;

        let values: Vec<(String, String)> = family
            .range(start_k.to_string()..)
            .take(limit)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        match values.len() {
            0 => Err(StorageError::KeyNotFound),
            _ => Ok(values),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put() {
        let column_family = "c";
        let storage = Storage::default();
        assert_eq!(storage.put(column_family, "k", "v"), Ok(()));
    }

    #[test]
    fn test_get() {
        let column_family = "c";
        let storage = Storage::default();
        storage.put(column_family, "k", "v").unwrap();

        assert_eq!(storage.get(column_family, "k"), Ok(String::from("v")));
        assert!(matches!(
            storage.get(column_family, "a"),
            Err(StorageError::KeyNotFound)
        ));
    }

    #[test]
    fn test_delete() {
        let column_family = "c";
        let storage = Storage::default();
        storage.put(column_family, "k", "v").unwrap();

        assert_eq!(storage.delete(column_family, "k"), Ok(String::from("v")));
        assert!(matches!(
            storage.delete(column_family, "k"),
            Err(StorageError::KeyNotFound)
        ));
    }

    #[test]
    fn test_scan() {
        let column_family = "c";
        let storage = Storage::default();
        storage.put(column_family, "0", "000").unwrap();
        storage.put(column_family, "1", "111").unwrap();
        storage.put(column_family, "2", "222").unwrap();
        storage.put(column_family, "3", "333").unwrap();
        storage.put(column_family, "4", "444").unwrap();
        storage.put(column_family, "5", "555").unwrap();

        let values = storage.scan(column_family, "2", 2).unwrap();
        assert_eq!(
            values,
            vec![("2".into(), "222".into()), ("3".into(), "333".into())]
        );

        let values = storage.scan(column_family, "2", 10).unwrap();
        assert_eq!(
            values,
            vec![
                ("2".into(), "222".into()),
                ("3".into(), "333".into()),
                ("4".into(), "444".into()),
                ("5".into(), "555".into())
            ]
        );

        // TODO: test when limit is 0
    }
}
