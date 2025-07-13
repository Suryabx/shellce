// src/commands/exit.rs
// Implementation of the `exit` command.

use anyhow::Result;
use async_trait::async_trait;
use crate::commands::Command;
use crate::core::types::{CommandResult, CommandRegistry};
use crate::core::variables::VariableManager;
use crate::core::config::ShellConfig;
use log::info;

pub struct ExitCommand;

#[async_trait]
impl Command for ExitCommand {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn description(&self) -> &'static str {
        "Exits the Shellce terminal." // Updated description
    }

    fn usage(&self) -> &'static str {
        "exit"
    }

    async fn execute(
        &self,
        _args: Vec<String>,
        _var_manager: &VariableManager,
        _config: &ShellConfig,
        _command_registry: &CommandRegistry,
    ) -> CommandResult {
        info!("Exit command received. Initiating shutdown.");
        // This command will cause the main loop to break in `main.rs`
        // We return a special result indicating the shell should exit.
        CommandResult::success(Some("Exiting Shellce. Goodbye!".to_string()), None) // Updated message
    }
}
