// src/parser/mod.rs
// Contains the main parsing logic for command lines.

pub mod variable_resolver;

use anyhow::Result;
use crate::core::variables::VariableManager;
use log::debug;

/// Parses a raw command line string into a command name and its arguments.
/// It also resolves any variables within the arguments.
///
/// # Arguments
/// * `command_line` - The raw string entered by the user.
/// * `var_manager` - A reference to the `VariableManager` for variable resolution.
///
/// # Returns
/// A `Result` containing a tuple of `(command_name, args)` or an `anyhow::Error`.
pub fn parse_command(command_line: &str, var_manager: &VariableManager) -> Result<(String, Vec<String>)> {
    let parts: Vec<&str> = command_line.split_whitespace().collect();

    if parts.is_empty() {
        anyhow::bail!("No command entered.");
    }

    let command_name = parts[0].to_string();
    let raw_args = parts[1..].to_vec();

    let mut resolved_args = Vec::new();
    for arg in raw_args {
        let resolved_arg = variable_resolver::resolve_variables(arg, var_manager)?;
        resolved_args.push(resolved_arg);
    }

    debug!("Parsed command: '{}', Args: {:?}", command_name, resolved_args);
    Ok((command_name, resolved_args))
}
