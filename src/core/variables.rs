// src/core/variables.rs
// Manages the in-memory variable store for ShellFlow.

use crate::core::types::VariableStore;
use std::sync::{Arc, Mutex};
use log::{info, warn};

/// `VariableManager` holds and manages the in-memory key-value store.
/// It uses `Arc<Mutex<...>>` for thread-safe access, as multiple commands
/// might need to read/write variables concurrently in an async environment.
#[derive(Debug, Clone)]
pub struct VariableManager {
    store: Arc<Mutex<VariableStore>>,
}

impl VariableManager {
    /// Creates a new, empty `VariableManager`.
    pub fn new() -> Self {
        info!("Initializing VariableManager.");
        VariableManager {
            store: Arc::new(Mutex::new(VariableStore::new())),
        }
    }

    /// Inserts or updates a variable in the store.
    pub fn set(&self, key: String, value: String) {
        let mut store = self.store.lock().unwrap();
        store.insert(key.clone(), value.clone());
        info!("Variable set: {} = {}", key, value);
    }

    /// Retrieves the value of a variable from the store.
    pub fn get(&self, key: &str) -> Option<String> {
        let store = self.store.lock().unwrap();
        let value = store.get(key).cloned();
        if value.is_some() {
            info!("Variable retrieved: {}", key);
        } else {
            warn!("Attempted to retrieve non-existent variable: {}", key);
        }
        value
    }

    /// Removes a variable from the store.
    pub fn remove(&self, key: &str) -> Option<String> {
        let mut store = self.store.lock().unwrap();
        let value = store.remove(key);
        if value.is_some() {
            info!("Variable removed: {}", key);
        } else {
            warn!("Attempted to remove non-existent variable: {}", key);
        }
        value
    }

    /// Returns a clone of the entire variable store.
    pub fn get_all(&self) -> VariableStore {
        let store = self.store.lock().unwrap();
        store.clone()
    }

    /// Replaces the entire variable store with a new one.
    pub fn set_all(&self, new_store: VariableStore) {
        let mut store = self.store.lock().unwrap();
        *store = new_store;
        info!("Variable store replaced.");
    }

    /// Returns a list of all variable keys.
    pub fn keys(&self) -> Vec<String> {
        let store = self.store.lock().unwrap();
        store.keys().cloned().collect()
    }
}

impl Default for VariableManager {
    fn default() -> Self {
        Self::new()
    }
}
