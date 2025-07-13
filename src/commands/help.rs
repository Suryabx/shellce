// src/commands/help.rs
// Implementation of the `help` command.

use anyhow::Result;
use async_trait::async_trait;
use serde_json::json;
use crate::commands::Command;
use crate::core::types::{CommandResult, CommandRegistry};
use crate::core::variables::VariableManager;
use crate::core::config::ShellConfig;
use log::info;

pub struct HelpCommand;

#[async_trait]
impl Command for HelpCommand {
    fn name(&self) -> &'static str {
        "help"
    }

    fn description(&self) -> &'static str {
        "Displays help information for all commands or a specific command."
    }

    fn usage(&self) -> &'static str {
        "help [command_name]"
    }

    async fn execute(
        &self,
        args: Vec<String>,
        _var_manager: &VariableManager,
        _config: &ShellConfig,
        command_registry: &CommandRegistry,
    ) -> CommandResult {
        let mut message = String::new();
        let mut commands_json = json!({});

        if args.is_empty() {
            // List all commands
            message.push_str("Available commands:\n");
            let mut command_names: Vec<&String> = command_registry.keys().collect();
            command_names.sort_unstable(); // Sort alphabetically

            for cmd_name in command_names {
                if let Some(cmd) = command_registry.get(cmd_name) {
                    message.push_str(&format!("  {:<15} - {}\n", cmd.name(), cmd.description()));
                    commands_json[cmd.name()] = json!({
                        "description": cmd.description(),
                        "usage": cmd.usage()
                    });
                }
            }
            message.push_str("\nType 'help <command_name>' for more details.");
            info!("Displayed general help.");
        } else {
            // Show help for a specific command
            let cmd_name = &args[0];
            if let Some(cmd) = command_registry.get(cmd_name) {
                message.push_str(&format!("Help for '{}':\n", cmd.name()));
                message.push_str(&format!("  Description: {}\n", cmd.description()));
                message.push_str(&format!("  Usage:       {}\n", cmd.usage()));
                commands_json[cmd.name()] = json!({
                    "description": cmd.description(),
                    "usage": cmd.usage()
                });
                info!("Displayed help for command: {}", cmd_name);
            } else {
                return CommandResult::error(format!("Command '{}' not found.", cmd_name));
            }
        }

        CommandResult::success(
            Some(message),
            Some(commands_json),
        )
    }
}
