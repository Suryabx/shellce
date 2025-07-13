// src/commands/command.rs
// Defines the Command trait for Shellce commands.

use async_trait::async_trait;
use crate::core::types::{CommandResult, CommandRegistry};
use crate::core::variables::VariableManager;
use crate::core::config::ShellConfig;

#[async_trait]
pub trait Command: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn usage(&self) -> &'static str;
    async fn execute(
        &self,
        args: Vec<String>,
        var_manager: &VariableManager,
        config: &ShellConfig,
        command_registry: &CommandRegistry,
    ) -> CommandResult;
}
