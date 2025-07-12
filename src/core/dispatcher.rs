// src/core/dispatcher.rs
// Contains the main command dispatching logic, including pipeline execution.

use anyhow::{Result, Context};
use log::{info, error, debug};
use crate::core::types::{CommandResult, CommandRegistry, PipelineCommand, CommandOutput};
use crate::core::variables::VariableManager;
use crate::core::config::ShellConfig;
use crate::parser; // Import the parser module
use serde_json::Value as JsonValue;

/// `CommandDispatcher` is responsible for parsing input, resolving aliases,
/// and executing the appropriate command or pipeline.
pub struct CommandDispatcher {
    command_registry: CommandRegistry,
}

impl CommandDispatcher {
    /// Creates a new `CommandDispatcher` with a given command registry.
    pub fn new(command_registry: CommandRegistry) -> Self {
        CommandDispatcher { command_registry }
    }

    /// Dispatches a command or pipeline based on the input line.
    ///
    /// This method performs:
    /// 1. Alias resolution.
    /// 2. Pipeline parsing and variable resolution.
    /// 3. Sequential command execution within the pipeline, passing output as input.
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

        // 2. Pipeline Parsing and Variable Resolution
        let pipeline_commands = match parser::parse_pipeline(&effective_command_line, var_manager) {
            Ok(cmds) => cmds,
            Err(e) => {
                error!("Pipeline parsing error: {}", e);
                return CommandResult::error(format!("Parsing error: {}", e));
            }
        };

        let mut last_output_data: Option<JsonValue> = None;

        for (i, p_cmd) in pipeline_commands.into_iter().enumerate() {
            let cmd_name = p_cmd.name;
            let mut args = p_cmd.args;

            // If there's previous output, pass it as a special argument (e.g., `--input-data`)
            // This is a common pattern for structured pipelines.
            if let Some(prev_data) = last_output_data.take() {
                // Here, we could decide how to pass structured data.
                // For simplicity, we'll stringify it and add it as an argument.
                // A more advanced system might use a dedicated input stream or context.
                args.push("--input-data".to_string());
                args.push(prev_data.to_string()); // Pass JSON string
                debug!("Piped data from previous command to '{}': {}", cmd_name, prev_data);
            }

            // 3. Command Execution
            if let Some(command) = self.command_registry.get(&cmd_name) {
                // Check if command is enabled by config (if `enabled_commands` is not empty)
                if !config.enabled_commands.is_empty() && !config.enabled_commands.contains(&cmd_name) {
                    error!("Command '{}' is disabled by configuration.", cmd_name);
                    return CommandResult::error(format!("Command '{}' is disabled.", cmd_name));
                }

                info!("Executing command: '{}' with args: {:?}", cmd_name, args);
                let result = command.execute(args, var_manager, config, &self.command_registry).await;

                if result.success {
                    // Capture output data for the next command in the pipeline
                    last_output_data = result.output.and_then(|o| o.data);
                    // If it's the last command, return its full result
                    if i == pipeline_commands.len() - 1 {
                        return result;
                    }
                } else {
                    // If any command in the pipeline fails, the whole pipeline fails
                    error!("Pipeline command '{}' failed: {:?}", cmd_name, result.error_message);
                    return result;
                }
            } else {
                error!("Unknown command in pipeline: '{}'", cmd_name);
                return CommandResult::error(format!("Unknown command in pipeline: '{}'.", cmd_name));
            }
        }

        // If the pipeline ran successfully but the last command had no output, return success.
        CommandResult::success(Some("Pipeline executed successfully.".to_string()), last_output_data)
    }
}
