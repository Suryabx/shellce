// src/core/dispatcher.rs
// Contains the main command dispatching logic.

use anyhow::{Result, Context};
use log::{info, error, debug};
use crate::core::types::{CommandResult, CommandRegistry};
use crate::core::variables::VariableManager;
use crate::core::config::ShellConfig;
use crate::parser; // Import the parser module

/// `CommandDispatcher` is responsible for parsing input, resolving aliases,
/// and executing the appropriate command.
pub struct CommandDispatcher {
    command_registry: CommandRegistry,
}

impl CommandDispatcher {
    /// Creates a new `CommandDispatcher` with a given command registry.
    pub fn new(command_registry: CommandRegistry) -> Self {
        CommandDispatcher { command_registry }
    }

    /// Dispatches a command based on the input line.
    ///
    /// This method performs:
    /// 1. Alias resolution.
    /// 2. Command parsing and variable resolution.
    /// 3. Command execution via the `Command` trait.
    ///
    /// # Arguments
    /// * `command_line` - The raw string input from the user.
    /// * `var_manager` - Reference to the variable store.
    /// * `config` - Reference to the shell configuration.
    ///
    /// # Returns
    /// A `CommandResult` indicating the outcome of the command execution.
    pub async fn dispatch_command(
        &self,
        command_line: &str,
        var_manager: &VariableManager,
        config: &ShellConfig,
    ) -> CommandResult {
        debug!("Dispatching command: '{}'", command_line);

        // 1. Alias Resolution
        let mut effective_command_line = command_line.to_string();
        for (alias, real_command) in &config.aliases {
            if effective_command_line.starts_with(alias) {
                effective_command_line = effective_command_line.replacen(alias, real_command, 1);
                info!("Alias resolved: '{}' -> '{}'", alias, effective_command_line);
                break;
            }
        }

        // 2. Command Parsing and Variable Resolution
        let (cmd_name, args) = match parser::parse_command(&effective_command_line, var_manager) {
            Ok((name, parsed_args)) => (name, parsed_args),
            Err(e) => {
                error!("Command parsing error: {}", e);
                return CommandResult::error(format!("Parsing error: {}", e));
            }
        };

        // 3. Command Execution
        if let Some(command) = self.command_registry.get(&cmd_name) {
            // Check if command is enabled by config (if `enabled_commands` is not empty)
            if !config.enabled_commands.is_empty() && !config.enabled_commands.contains(&cmd_name) {
                error!("Command '{}' is disabled by configuration.", cmd_name);
                return CommandResult::error(format!("Command '{}' is disabled.", cmd_name));
            }

            info!("Executing command: '{}' with args: {:?}", cmd_name, args);
            command.execute(args, var_manager, config, &self.command_registry).await
        } else {
            error!("Unknown command: '{}'", cmd_name);
            CommandResult::error(format!("Unknown command: '{}'. Type 'help' for a list of commands.", cmd_name))
        }
    }
}
