// remember command implementation
use crate::core::types::CommandResult;
use super::Command;

pub struct RememberCommand;

impl Command for RememberCommand {
    fn name(&self) -> &'static str {
        "remember"
    }
    fn execute(&self, args: &[String]) -> CommandResult {
        // TODO: Implement variable storing logic
        CommandResult::Message("Remembered!".to_string())
    }
}
