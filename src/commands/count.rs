// src/commands/count.rs
// Implementation of the `count` command, designed to work with pipelines.

use anyhow::Result;
use async_trait::async_trait;
use serde_json::json;
use crate::commands::command::Command;
use crate::core::types::{CommandResult, CommandRegistry};
use crate::core::variables::VariableManager;
use crate::core::config::ShellConfig;
use log::{info, error, debug};

pub struct CountCommand;

#[async_trait]
impl Command for CountCommand {
    fn name(&self) -> &'static str {
        "count"
    }

    fn description(&self) -> &'static str {
        "Counts items in structured input (e.g., from a pipeline)."
    }

    fn usage(&self) -> &'static str {
        "count [--input-data <json_string>]"
    }

    async fn execute(
        &self,
        args: Vec<String>,
        _var_manager: &VariableManager,
        _config: &ShellConfig,
        _command_registry: &CommandRegistry,
    ) -> CommandResult {
        let mut input_data: Option<serde_json::Value> = None;

        // Look for the --input-data argument
        let mut i = 0;
        while i < args.len() {
            if args[i] == "--input-data" {
                if i + 1 < args.len() {
                    let json_str = &args[i+1];
                    match serde_json::from_str(json_str) {
                        Ok(val) => {
                            input_data = Some(val);
                            debug!("Count command received piped input: {}", json_str);
                        },
                        Err(e) => {
                            error!("Failed to parse input data JSON: {}", e);
                            return CommandResult::error(format!("Invalid input data: {}", e));
                        }
                    }
                } else {
                    return CommandResult::error("Missing JSON string after --input-data.".to_string());
                }
                i += 2; // Skip both --input-data and the JSON string
            } else {
                // If we add more arguments to `count`, they would be processed here.
                // For now, any other args are considered invalid.
                return CommandResult::error(format!("Unknown argument: '{}'. Usage: {}", args[i], self.usage()));
            }
        }

        let count = match input_data {
            Some(serde_json::Value::Array(arr)) => arr.len(),
            Some(serde_json::Value::Object(_)) => 1, // Treat a single object as 1 item
            Some(_) => {
                info!("Count command received non-array/object input, treating as 0 or 1.");
                1 // Treat any other single value as 1 item for counting purposes
            },
            None => 0, // No input data
        };

        info!("Counted {} items.", count);
        CommandResult::success(
            Some(format!("Counted {} items.", count)),
            Some(json!({ "count": count })),
        )
    }
}
