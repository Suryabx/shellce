// src/commands/greet.rs
// Implementation of the `greet` command.

use anyhow::Result;
use async_trait::async_trait;
use serde_json::json;
use crate::commands::Command;
use crate::core::types::{CommandResult, CommandRegistry};
use crate::core::variables::VariableManager;
use crate::core::config::ShellConfig;
use log::info;

pub struct GreetCommand;

#[async_trait]
impl Command for GreetCommand {
    fn name(&self) -> &'static str {
        "greet"
    }

    fn description(&self) -> &'static str {
        "Greets the specified name or a default value."
    }

    fn usage(&self) -> &'static str {
        "greet [name]"
    }

    async fn execute(
        &self,
        args: Vec<String>,
        _var_manager: &VariableManager,
        _config: &ShellConfig,
        _command_registry: &CommandRegistry,
    ) -> CommandResult {
        let name = if args.is_empty() {
            "World".to_string() // Default name if none provided
        } else {
            args.join(" ") // Join all arguments as the name
        };

        let message = format!("Hello, {}! Welcome to Shellce.", name); // Updated message
        info!("Executed greet command for: {}", name);

        CommandResult::success(
            Some(message.clone()),
            Some(json!({ "greeting": message, "name": name })),
        )
    }
}
