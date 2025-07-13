// src/commands/echo.rs
// Implementation of the `echo` command.

use anyhow::Result;
use async_trait::async_trait;
use serde_json::json;
use crate::commands::Command;
use crate::core::types::{CommandResult, CommandRegistry};
use crate::core::variables::VariableManager;
use crate::core::config::ShellConfig;
use log::info;

pub struct EchoCommand;

#[async_trait]
impl Command for EchoCommand {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn description(&self) -> &'static str {
        "Prints the given text to the console, resolving variables."
    }

    fn usage(&self) -> &'static str {
        "echo [text with {variables}]"
    }

    async fn execute(
        &self,
        args: Vec<String>,
        _var_manager: &VariableManager, // Variables are resolved by the parser before execute
        _config: &ShellConfig,
        _command_registry: &CommandRegistry,
    ) -> CommandResult {
        let text = args.join(" ");
        info!("Echoing: {}", text);

        CommandResult::success(
            Some(text.clone()),
            Some(json!({ "output_text": text })),
        )
    }
}
