// src/commands/list_vars.rs
// Implementation of the `list-vars` command.

use anyhow::Result;
use async_trait::async_trait;
use serde_json::json;
use crate::commands::command::Command;
use crate::core::types::{CommandResult, CommandRegistry};
use crate::core::variables::VariableManager;
use crate::core::config::ShellConfig;
use log::info;

pub struct ListVarsCommand;

#[async_trait]
impl Command for ListVarsCommand {
    fn name(&self) -> &'static str {
        "list-vars"
    }

    fn description(&self) -> &'static str {
        "Lists all currently remembered variables and their values."
    }

    fn usage(&self) -> &'static str {
        "list-vars"
    }

    async fn execute(
        &self,
        _args: Vec<String>,
        var_manager: &VariableManager,
        _config: &ShellConfig,
        _command_registry: &CommandRegistry,
    ) -> CommandResult {
        let all_vars = var_manager.get_all();
        let mut message = String::new();
        let mut vars_json = json!({});

        if all_vars.is_empty() {
            message.push_str("No variables currently remembered.");
        } else {
            message.push_str("Remembered variables:\n");
            for (key, value) in &all_vars {
                message.push_str(&format!("  {} = {}\n", key, value));
                vars_json[key] = json!(value);
            }
        }
        info!("Listed variables.");

        CommandResult::success(
            Some(message),
            Some(vars_json),
        )
    }
}
