// src/commands/ping.rs
// Implementation of the `ping` command.

use anyhow::Result;
use async_trait::async_trait;
use serde_json::json;
use crate::commands::command::Command;
use crate::core::types::{CommandResult, CommandRegistry};
use crate::core::variables::VariableManager;
use crate::core::config::ShellConfig;
use log::info;

pub struct PingCommand;

#[async_trait]
impl Command for PingCommand {
    fn name(&self) -> &'static str {
        "ping"
    }

    fn description(&self) -> &'static str {
        "Responds with 'pong'."
    }

    fn usage(&self) -> &'static str {
        "ping"
    }

    async fn execute(
        &self,
        _args: Vec<String>,
        _var_manager: &VariableManager,
        _config: &ShellConfig,
        _command_registry: &CommandRegistry,
    ) -> CommandResult {
        info!("Executed ping command.");
        CommandResult::success(
            Some("pong".to_string()),
            Some(json!({ "response": "pong" })),
        )
    }
}
