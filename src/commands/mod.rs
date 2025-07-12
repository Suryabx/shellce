// src/commands/mod.rs
// Defines the `Command` trait and registers all available commands.

use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use lazy_static::lazy_static;
use log::info;
use serde_json::Value as JsonValue; // Import JsonValue for trait signature

use crate::core::types::{CommandResult, CommandRegistry};
use crate::core::variables::VariableManager;
use crate::core::config::ShellConfig;

// Declare individual command modules
pub mod greet;
pub mod remember;
pub mod echo;
pub mod list_vars;
pub mod save_load;
pub mod help;
pub mod exit;
pub mod source;
pub mod ping;
pub mod sleep;
pub mod fs;
pub mod exec;
pub mod count; // New: Declare the count command module

/// The `Command` trait defines the interface for all executable commands in ShellFlow.
///
/// Commands must be `Send` and `Sync` to be safely used across async tasks
/// and stored in a shared `CommandRegistry`.
#[async_trait]
pub trait Command: Send + Sync {
    /// Returns the primary name of the command.
    fn name(&self) -> &'static str;

    /// Returns a brief description of the command's purpose.
    fn description(&self) -> &'static str;

    /// Returns a string detailing the command's usage syntax.
    fn usage(&self) -> &'static str;

    /// Executes the command with the given arguments and environment.
    ///
    /// # Arguments
    /// * `args` - A vector of string arguments passed to the command.
    /// * `var_manager` - A reference to the global `VariableManager` for variable access.
    /// * `config` - A reference to the global `ShellConfig` for configuration access.
    /// * `command_registry` - A reference to the `CommandRegistry` for command introspection (e.g., for `help`).
    ///
    /// # Returns
    /// A `CommandResult` indicating success or failure, and any output.
    async fn execute(
        &self,
        args: Vec<String>,
        var_manager: &VariableManager,
        config: &ShellConfig,
        command_registry: &CommandRegistry,
    ) -> CommandResult;
}

// Use lazy_static to create a globally accessible, mutable (once initialized)
// HashMap for our command registry.
lazy_static! {
    /// The global registry of all available commands.
    /// Commands are registered here at application startup.
    pub static ref COMMAND_REGISTRY: CommandRegistry = {
        let mut registry: CommandRegistry = HashMap::new();
        // Register all built-in commands
        register_command(&mut registry, Box::new(greet::GreetCommand));
        register_command(&mut registry, Box::new(remember::RememberCommand));
        register_command(&mut registry, Box::new(echo::EchoCommand));
        register_command(&mut registry, Box::new(list_vars::ListVarsCommand));
        register_command(&mut registry, Box::new(save_load::SaveMemoryCommand));
        register_command(&mut registry, Box::new(save_load::LoadMemoryCommand));
        register_command(&mut registry, Box::new(help::HelpCommand));
        register_command(&mut registry, Box::new(exit::ExitCommand));
        register_command(&mut registry, Box::new(source::SourceCommand));
        register_command(&mut registry, Box::new(ping::PingCommand));
        register_command(&mut registry, Box::new(sleep::SleepCommand));
        // Register file system commands
        register_command(&mut registry, Box::new(fs::LsCommand));
        register_command(&mut registry, Box::new(fs::CreateFileCommand));
        register_command(&mut registry, Box::new(fs::ReadFileCommand));
        register_command(&mut registry, Box::new(fs::DeleteFileCommand));
        register_command(&mut registry, Box::new(fs::CdCommand));
        register_command(&mut registry, Box::new(fs::PwdCommand));
        register_command(&mut registry, Box::new(exec::ExecCommand));
        register_command(&mut registry, Box::new(count::CountCommand)); // New: Register count command


        info!("Registered {} commands.", registry.len());
        registry
    };
}

/// Helper function to register a command into the registry.
fn register_command(registry: &mut CommandRegistry, command: Box<dyn Command + Send + Sync>) {
    registry.insert(command.name().to_string(), command);
}

/// Returns a reference to the global command registry.
pub fn get_command_registry() -> &'static CommandRegistry {
    &COMMAND_REGISTRY
}

