// src/commands/remember.rs
// Implementation of the `remember` command for variable storage.

use anyhow::Result;
use async_trait::async_trait;
use serde_json::json;
use crate::commands::Command;
use crate::core::types::{CommandResult, CommandRegistry};
use crate::core::variables::VariableManager;
use crate::core::config::ShellConfig;
use log::info;

pub struct RememberCommand;

#[async_trait]
impl Command for RememberCommand {
    fn name(&self) -> &'static str {
        "remember"
    }

    fn description(&self) -> &'static str {
        "Stores a key-value pair in memory. Usage: remember key = value"
    }

    fn usage(&self) -> &'static str {
        "remember <key> = <value>"
    }

    async fn execute(
        &self,
        args: Vec<String>,
        var_manager: &VariableManager,
        _config: &ShellConfig,
        _command_registry: &CommandRegistry,
    ) -> CommandResult {
        let mut key = String::new();
        let mut value = String::new();
        let mut found_equals = false;

        for arg in args {
            if arg == "=" {
                found_equals = true;
                continue;
            }
            if !found_equals {
                key.push_str(&arg);
            } else {
                if !value.is_empty() {
                    value.push(' '); // Add space for multi-word values
                }
                value.push_str(&arg);
            }
        }

        if key.is_empty() || value.is_empty() || !found_equals {
            return CommandResult::error(format!(
                "Invalid usage. {}",
                self.usage()
            ));
        }

        var_manager.set(key.clone(), value.clone());
        info!("Remembered: {} = {}", key, value);

        CommandResult::success(
            Some(format!("Remembered: {} = {}", key, value)),
            Some(json!({ "key": key, "value": value })),
        )
    }
}
