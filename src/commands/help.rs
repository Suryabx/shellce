// help command implementation
use crate::core::types::CommandResult;
use super::Command;

pub struct HelpCommand;

impl Command for HelpCommand {
    fn name(&self) -> &'static str {
        "help"
    }
    fn execute(&self, _args: &[String]) -> CommandResult {
        CommandResult::Message("Available commands: greet, remember, echo, list-vars, save-memory, load-memory, help, exit, source".to_string())
    }
}
