// src/parser/mod.rs
// Contains the main parsing logic for command lines.

pub mod variable_resolver;

use anyhow::{Result, Context};
use crate::core::variables::VariableManager;
use log::debug;

/// Parses a raw command line string into a command name and its arguments,
/// handling quoted strings and resolving variables.
///
/// # Arguments
/// * `command_line` - The raw string entered by the user.
/// * `var_manager` - A reference to the `VariableManager` for variable resolution.
///
/// # Returns
/// A `Result` containing a tuple of `(command_name, args)` or an `anyhow::Error`.
pub fn parse_command(command_line: &str, var_manager: &VariableManager) -> Result<(String, Vec<String>)> {
    let mut args = Vec::new();
    let mut chars = command_line.chars().peekable();
    let mut current_arg = String::new();
    let mut in_quote = false;

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                in_quote = !in_quote;
                if !in_quote && !current_arg.is_empty() {
                    // End of a quoted argument, add it
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
        anyhow::bail!("No command entered.");
    }

    let command_name = args[0].clone();
    let raw_args = args[1..].to_vec();

    let mut resolved_args = Vec::new();
    for arg in raw_args {
        let resolved_arg = variable_resolver::resolve_variables(&arg, var_manager)?;
        resolved_args.push(resolved_arg);
    }

    debug!("Parsed command: '{}', Args: {:?}", command_name, resolved_args);
    Ok((command_name, resolved_args))
}
