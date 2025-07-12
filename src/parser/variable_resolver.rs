// src/parser/variable_resolver.rs
// Handles dynamic variable resolution within command arguments.

use crate::core::variables::VariableManager;
use anyhow::Result;
use log::{debug, warn};

/// Resolves variables in a given string.
/// Variables are identified by `{variable_name}` syntax.
///
/// # Arguments
/// * `input` - The string potentially containing variables.
/// * `var_manager` - A reference to the `VariableManager` to look up variable values.
///
/// # Returns
/// A `Result` containing the string with all variables resolved, or an error if
/// a variable cannot be found.
pub fn resolve_variables(input: &str, var_manager: &VariableManager) -> Result<String> {
    let mut resolved_string = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '{' {
            // Check if it's the start of a variable
            let mut var_name = String::new();
            let mut in_variable = false;
            while let Some(&next_c) = chars.peek() {
                if next_c == '}' {
                    chars.next(); // Consume '}'
                    in_variable = true;
                    break;
                }
                var_name.push(chars.next().unwrap());
            }

            if in_variable {
                // Look up the variable in the store
                if let Some(value) = var_manager.get(&var_name) {
                    resolved_string.push_str(&value);
                    debug!("Resolved variable: {} -> {}", var_name, value);
                } else {
                    warn!("Unresolved variable: {{{}}}", var_name);
                    // If a variable is not found, return an error or keep it as is.
                    // For a production-grade shell, returning an error is safer.
                    anyhow::bail!("Variable not found: {{{}}}", var_name);
                }
            } else {
                // Not a variable, just a literal '{'
                resolved_string.push(c);
                resolved_string.push_str(&var_name);
            }
        } else {
            resolved_string.push(c);
        }
    }

    Ok(resolved_string)
}
