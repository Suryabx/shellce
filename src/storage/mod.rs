// src/storage/mod.rs
// Handles file-based persistence for variables.

use anyhow::{Result, Context};
use serde_json;
use std::fs;
use std::path::Path;
use log::{info, error};

use crate::core::types::VariableStore;

/// `MemoryStorage` provides methods to save and load the `VariableStore` to/from a file.
pub struct MemoryStorage;

impl MemoryStorage {
    /// Saves the given `VariableStore` to a JSON file.
    ///
    /// # Arguments
    /// * `store` - The `VariableStore` to save.
    /// * `path` - The file path to save to.
    pub async fn save(store: &VariableStore, path: &Path) -> Result<()> {
        info!("Attempting to save memory to: {:?}", path);
        let json_string = serde_json::to_string_pretty(store)
            .context("Failed to serialize variable store to JSON")?;

        // Use tokio::fs for async file operations
        tokio::fs::write(path, json_string)
            .await
            .with_context(|| format!("Failed to write memory file: {:?}", path))?;

        info!("Memory saved successfully to: {:?}", path);
        Ok(())
    }

    /// Loads a `VariableStore` from a JSON file.
    ///
    /// # Arguments
    /// * `path` - The file path to load from.
    ///
    /// # Returns
    /// A `Result` containing the loaded `VariableStore` or an `anyhow::Error`.
    pub async fn load(path: &Path) -> Result<VariableStore> {
        info!("Attempting to load memory from: {:?}", path);
        // Use tokio::fs for async file operations
        let json_string = tokio::fs::read_to_string(path)
            .await
            .with_context(|| format!("Failed to read memory file: {:?}", path))?;

        let store: VariableStore = serde_json::from_str(&json_string)
            .with_context(|| format!("Failed to parse memory file: {:?}", path))?;

        info!("Memory loaded successfully from: {:?}", path);
        Ok(store)
    }
}
