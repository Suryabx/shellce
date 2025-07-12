// src/parser/mod.rs
// Contains the main parsing logic for command lines, including pipelines.

pub mod variable_resolver;

use anyhow::{Result, Context};
use crate::core::variables::VariableManager;
use crate::core::types::PipelineCommand;
use log::debug;

/// Parses a raw command line string into a vector of `PipelineCommand`s,
/// handling quoted strings, variable resolution, and pipe (`|`) operators.
///
/// # Arguments
/// * `command_line` - The raw string entered by the user.
/// * `var_manager` - A reference to the `VariableManager` for variable resolution.
///
/// # Returns
/// A `Result` containing a `Vec<PipelineCommand>` or an `anyhow::Error`.
pub fn parse_pipeline(command_line: &str, var_manager: &VariableManager) -> Result<Vec<PipelineCommand>> {
    let mut pipeline = Vec::new();
    let mut current_command_str = String::new();
    let mut chars = command_line.chars().peekable();
    let mut in_quote = false;

    while let Some(c) = chars.next() {
        match c {
            '|' if !in_quote => {
                // End of a command in the pipeline
                if !current_command_str.trim().is_empty() {
                    pipeline.push(parse_single_command(&current_command_str, var_manager)?);
                }
                current_command_str.clear();
            },
            '"' => {
                in_quote = !in_quote;
                current_command_str.push(c); // Keep quotes for internal parsing
            },
            _ => {
                current_command_str.push(c);
            }
        }
    }

    // Add the last command in the pipeline
    if !current_command_str.trim().is_empty() {
        pipeline.push(parse_single_command(&current_command_str, var_manager)?);
    }

    if pipeline.is_empty() {
        anyhow::bail!("No command entered.");
    }

    debug!("Parsed pipeline: {:?}", pipeline);
    Ok(pipeline)
}

/// Parses a single command string (which may contain quoted arguments and variables)
/// into a command name and its arguments.
fn parse_single_command(command_str: &str, var_manager: &VariableManager) -> Result<PipelineCommand> {
    let mut args = Vec::new();
    let mut chars = command_str.chars().peekable();
    let mut current_arg = String::new();
    let mut in_quote = false;

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                in_quote = !in_quote;
                // If ending a quote, and the current_arg is not empty, add it.
                // This handles cases like `echo "hello world"` where the quote ends the arg.
                // If starting a quote, we don't add the quote char to current_arg.
                if !in_quote && !current_arg.is_empty() {
                    args.push(current_arg.clone());
                    current_arg.clear();
                }
            },
            ' ' if !in_quote => {
                // Space outside quotes, end of argument
                if !current_arg.is_empty() {
                    args.push(current_arg.clone());
                    current_arg.clear();
                }
            },
            _ => {
                current_arg.push(c);
            }
        }
    }

    // Add the last argument if any
    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    if args.is_empty() {
        anyhow::bail!("Empty command in pipeline.");
    }

    let command_name = args[0].clone();
    let raw_args = args[1..].to_vec();

    let mut resolved_args = Vec::new();
    for arg in raw_args {
        let resolved_arg = variable_resolver::resolve_variables(&arg, var_manager)?;
        resolved_args.push(resolved_arg);
    }

    Ok(PipelineCommand {
        name: command_name,
        args: resolved_args,
    })
}
