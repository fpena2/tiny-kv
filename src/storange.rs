use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum StorageError {
    #[error("Failed to acquire mutex lock")]
    MutexLock,
    #[error("Key not found in the specified column family")]
    KeyNotFound,
    #[error("Failed to insert value")]
    InsertionFailed,
}

#[derive(Default)]
struct Storage(Arc<Mutex<HashMap<String, HashMap<String, String>>>>);

impl Storage {
    /// replaces the value for a particular key for the specified CF in the database
    pub fn put(&self, column_family: &str, k: &str, v: &str) -> Result<(), StorageError> {
        let mut storage = self.0.lock().map_err(|_| StorageError::MutexLock)?;
        storage
            .entry(column_family.to_string())
            .or_insert(HashMap::new())
            .insert(k.to_string(), v.to_string());

        Ok(())
    }
    /// fetches the current value for a key for the specified CF
    pub fn get(&self, column_family: &str, k: &str) -> Result<String, StorageError> {
        let storage = self.0.lock().map_err(|_| StorageError::MutexLock)?;
        if let Some(family) = storage.get(column_family) {
            if let Some(value) = family.get(k) {
                return Ok(value.clone());
            }
        }
        Err(StorageError::KeyNotFound)
    }

    /// deletes the key's value for the specified CF
    pub fn delete(&self, column_family: &str, k: &str) -> Result<String, StorageError> {
        let mut storage = self.0.lock().map_err(|_| StorageError::MutexLock)?;
        if let Some(family) = storage.get_mut(column_family) {
            if let Some(value) = family.remove(k) {
                return Ok(value);
            }
        }
        Err(StorageError::KeyNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put() {
        let column_family = "c";
        let k = "k";
        let v = "v";

        let storage = Storage::default();

        assert_eq!(storage.put(column_family, k, v), Ok(()));
    }
    #[test]
    fn test_get() {
        let column_family = "c";
        let k = "k";
        let v = "v";

        let storage = Storage::default();
        storage.put(column_family, k, v).unwrap();

        assert_eq!(storage.get(column_family, k), Ok(String::from(v)));
    }

    #[test]
    fn test_delete() {
        let column_family = "c";
        let k = "k";
        let v = "v";

        let storage = Storage::default();
        storage.put(column_family, k, v).unwrap();

        assert_eq!(storage.delete(column_family, k), Ok(String::from(v)));
    }
}
