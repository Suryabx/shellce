// src/commands/save_load.rs
// Implementations for `save-memory` and `load-memory` commands.

use anyhow::Result;
use async_trait::async_trait;
use std::path::PathBuf;
use crate::commands::command::Command;
use crate::core::types::{CommandResult, CommandRegistry};
use crate::core::variables::VariableManager;
use crate::core::config::ShellConfig;
use crate::storage::MemoryStorage;
use log::{info, error};

pub struct SaveMemoryCommand;

#[async_trait]
#[async_trait]
impl Command for SaveMemoryCommand {
    fn name(&self) -> &'static str {
        "save-memory"
    }

    fn description(&self) -> &'static str {
        "Saves all current variables to a file."
    }

    fn usage(&self) -> &'static str {
        "save-memory [path]"
    }

    async fn execute(
        &self,
        args: Vec<String>,
        var_manager: &VariableManager,
        config: &ShellConfig,
        command_registry: &CommandRegistry,
    ) -> CommandResult {
        let path_str = if args.is_empty() {
            "shellce_memory.json".to_string() // Updated default path
        } else {
            args[0].clone()
        };
        let path = PathBuf::from(path_str.clone());

        info!("Attempting to save memory to: {:?}", path);

        let store = var_manager.get_all();
        match MemoryStorage::save(&store, &path).await {
            Ok(_) => {
                info!("Memory saved to: {:?}", path);
                CommandResult::success(
                    Some(format!("Variables saved to {:?}", path)),
                    None,
                )
            },
            Err(e) => {
                error!("Failed to save memory: {:?}", e);
                CommandResult::error(format!("Failed to save memory: {}", e))
            },
        }
    }
}

pub struct LoadMemoryCommand;

#[async_trait]
impl Command for LoadMemoryCommand {
    fn name(&self) -> &'static str {
        "load-memory"
    }

    fn description(&self) -> &'static str {
        "Loads variables from a file, overwriting current ones."
    }

    fn usage(&self) -> &'static str {
        "load-memory [path]"
    }

    async fn execute(
        &self,
        args: Vec<String>,
        var_manager: &VariableManager,
        _config: &ShellConfig,
        _command_registry: &CommandRegistry,
    ) -> CommandResult {
        let path_str = if args.is_empty() {
            "shellce_memory.json".to_string() // Updated default path
        } else {
            args[0].clone()
        };
        let path = PathBuf::from(path_str.clone());

        info!("Attempting to load memory from: {:?}", path);

        match MemoryStorage::load(&path).await {
            Ok(loaded_store) => {
                var_manager.set_all(loaded_store);
                info!("Memory loaded from: {:?}", path);
                CommandResult::success(
                    Some(format!("Variables loaded from {:?}", path)),
                    None,
                )
            },
            Err(e) => {
                error!("Failed to load memory: {:?}", e);
                CommandResult::error(format!("Failed to load memory: {}", e))
            },
        }
    }
}
