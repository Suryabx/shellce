// src/commands/sleep.rs
// Implementation of the `sleep` command, demonstrating async behavior.

use anyhow::Result;
use async_trait::async_trait;
use serde_json::json;
use std::time::Duration;
use crate::commands::Command;
use crate::core::types::{CommandResult, CommandRegistry};
use crate::core::variables::VariableManager;
use crate::core::config::ShellConfig;
use log::{info, error};

pub struct SleepCommand;

#[async_trait]
impl Command for SleepCommand {
    fn name(&self) -> &'static str {
        "sleep"
    }

    fn description(&self) -> &'static str {
        "Pauses execution for a specified duration in seconds. (Async)"
    }

    fn usage(&self) -> &'static str {
        "sleep <seconds>"
    }

    async fn execute(
        &self,
        args: Vec<String>,
        _var_manager: &VariableManager,
        _config: &ShellConfig,
        _command_registry: &CommandRegistry,
    ) -> CommandResult {
        if args.is_empty() {
            return CommandResult::error(format!("Invalid usage. {}", self.usage()));
        }

        let seconds_str = &args[0];
        let seconds = match seconds_str.parse::<u64>() {
            Ok(s) => s,
            Err(_) => {
                return CommandResult::error(format!("Invalid duration: '{}'. Please provide a number of seconds.", seconds_str));
            }
        };

        info!("Sleeping for {} seconds...", seconds);
        // Use tokio::time::sleep for async non-blocking sleep
        tokio::time::sleep(Duration::from_secs(seconds)).await;
        info!("Finished sleeping for {} seconds.", seconds);

        CommandResult::success(
            Some(format!("Slept for {} seconds.", seconds)),
            Some(json!({ "duration_seconds": seconds })),
        )
    }
}
