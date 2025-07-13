// src/commands/source.rs
// Implementation of the `source` command for running script files.

use anyhow::{Result, Context};
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use crate::commands::Command;
use crate::core::types::{CommandResult, CommandRegistry};
use crate::core::variables::VariableManager;
use crate::core::config::ShellConfig;
use crate::core::dispatcher::CommandDispatcher; // Need dispatcher to run commands
use log::{info, error, debug};

pub struct SourceCommand;

#[async_trait]
impl Command for SourceCommand {
    fn name(&self) -> &'static str {
        "source"
    }

    fn description(&self) -> &'static str {
        "Executes commands from a specified Shellce script file (.sf)." // Updated description
    }

    fn usage(&self) -> &'static str {
        "source <file_path>"
    }

    async fn execute(
        &self,
        args: Vec<String>,
        var_manager: &VariableManager,
        config: &ShellConfig,
        command_registry: &CommandRegistry,
    ) -> CommandResult {
        if args.is_empty() {
            return CommandResult::error(format!("Invalid usage. {}", self.usage()));
        }

        let file_path_str = &args[0];
        let path = PathBuf::from(file_path_str);

        info!("Attempting to source script from: {:?}", path);

        let file = match File::open(&path).await {
            Ok(f) => f,
            Err(e) => {
                error!("Failed to open script file {:?}: {}", path, e);
                return CommandResult::error(format!("Failed to open script file: {}", e));
            }
        };

        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let mut line_num = 0;
        let dispatcher = CommandDispatcher::new(command_registry.clone()); // Clone registry for dispatcher

        let mut success_count = 0;
        let mut error_count = 0;

        while let Some(line_result) = lines.next_line().await.context("Failed to read line from script") {
            line_num += 1;
            let line = match line_result {
                Ok(l) => l.trim().to_string(),
                Err(e) => {
                    error!("Error reading line {} from script {:?}: {}", line_num, path, e);
                    error_count += 1;
                    continue;
                }
            };

            if line.is_empty() || line.starts_with('#') { // Skip empty lines and comments
                continue;
            }

            debug!("Executing script line {}: '{}'", line_num, line);
            let result = dispatcher.dispatch_command(&line, var_manager, config).await;

            if result.success {
                success_count += 1;
                info!("Script line {} executed successfully.", line_num);
            } else {
                error_count += 1;
                error!("Script line {} failed: {:?}", line_num, result.error_message);
            }
        }

        let total_commands = success_count + error_count;
        let message = format!(
            "Script {:?} finished. Executed {} commands: {} successful, {} failed.",
            path, total_commands, success_count, error_count
        );
        info!("{}", message);

        if error_count > 0 {
            CommandResult::error(message)
        } else {
            CommandResult::success(Some(message), None)
        }
    }
}
