// exit command implementation
use crate::core::types::CommandResult;
use super::Command;

pub struct ExitCommand;

impl Command for ExitCommand {
    fn name(&self) -> &'static str {
        "exit"
    }
    fn execute(&self, _args: &[String]) -> CommandResult {
        CommandResult::Exit
    }
}
