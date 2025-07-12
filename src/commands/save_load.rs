// save-memory, load-memory commands
use crate::core::types::CommandResult;
use super::Command;

pub struct SaveMemoryCommand;
pub struct LoadMemoryCommand;

impl Command for SaveMemoryCommand {
    fn name(&self) -> &'static str {
        "save-memory"
    }
    fn execute(&self, _args: &[String]) -> CommandResult {
        // TODO: Save variables to file
        CommandResult::Message("Memory saved.".to_string())
    }
}

impl Command for LoadMemoryCommand {
    fn name(&self) -> &'static str {
        "load-memory"
    }
    fn execute(&self, _args: &[String]) -> CommandResult {
        // TODO: Load variables from file
        CommandResult::Message("Memory loaded.".to_string())
    }
}
