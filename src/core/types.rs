// src/core/types.rs
// Defines the fundamental data structures for Shellce.

use anyhow::Result;
use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use rustyline::error::ReadlineError; // Corrected import path
use rustyline::completion::extract_word; // Corrected import for extract_word

/// Represents the structured output of any Shellce command.
#[derive(Debug, Serialize, Deserialize, Clone)] // Added Clone derive
pub struct CommandOutput {
    /// A human-readable message, optional.
    pub message: Option<String>,
    /// Structured data, typically JSON, for programmatic use.
    pub data: Option<JsonValue>,
}

/// Represents the result of a Shellce command execution.
/// Uses `anyhow::Result` for ergonomic error handling.
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult {
    /// True if the command executed successfully, false otherwise.
    pub success: bool,
    /// Optional output if the command was successful.
    pub output: Option<CommandOutput>,
    /// Optional error details if the command failed.
    pub error_message: Option<String>, // Simplified error message for display
}

impl CommandResult {
    /// Creates a successful command result with an optional message and data.
    pub fn success(message: Option<String>, data: Option<JsonValue>) -> Self {
        CommandResult {
            success: true,
            output: Some(CommandOutput { message, data }),
            error_message: None,
        }
    }

    /// Creates a failed command result with an error message.
    pub fn error(message: String) -> Self {
        CommandResult {
            success: false,
            output: None,
            error_message: Some(message),
        }
    }
}

/// A type alias for the variable store, mapping string keys to string values.
pub type VariableStore = std::collections::HashMap<String, String>;

/// A type alias for the command dispatcher, mapping command names to boxed Command trait objects.
pub type CommandRegistry = std::collections::HashMap<String, Box<dyn crate::commands::Command + Send + Sync>>;

/// Represents a single command within a pipeline.
#[derive(Debug, Clone)]
pub struct PipelineCommand {
    pub name: String,
    pub args: Vec<String>,
}

// Custom completer for rustyline, used for autocompletion
pub struct ShellFlowCompleter {
    pub commands: Vec<String>,
    pub variables: Vec<String>,
}

impl rustyline::completion::Completer for ShellFlowCompleter {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> Result<(usize, Vec<String>), ReadlineError> { // Corrected ReadlineError path
        let (start, word) = extract_word(line, pos, None); // Corrected usage

        let mut completions = Vec::new();

        // Command completions
        for cmd in &self.commands {
            if cmd.starts_with(word) {
                completions.push(cmd.clone());
            }
        }

        // Variable completions (e.g., for `{var}` syntax)
        if word.starts_with('{') {
            for var in &self.variables {
                let full_var_syntax = format!("{{{}}}", var);
                if full_var_syntax.starts_with(word) {
                    completions.push(full_var_syntax);
                }
            }
        }

        Ok((start, completions))
    }
}

// Helper struct for rustyline's Helper trait
pub struct ShellFlowHelper {
    pub completer: ShellFlowCompleter,
}

impl rustyline::completion::Completer for ShellFlowHelper {
    type Candidate = String;
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> Result<(usize, Vec<String>), ReadlineError> { // Corrected ReadlineError path
        self.completer.complete(line, pos, ctx)
    }
}

impl rustyline::highlight::Highlighter for ShellFlowHelper {}
impl rustyline::hint::Hinter for ShellFlowHelper {
    type Hint = String;
}
impl rustyline::validate::Validator for ShellFlowHelper {}
impl rustyline::Helper for ShellFlowHelper {}
