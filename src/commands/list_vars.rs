// list-vars command implementation
use crate::core::types::CommandResult;
use super::Command;

pub struct ListVarsCommand;

impl Command for ListVarsCommand {
    fn name(&self) -> &'static str {
        "list-vars"
    }
    fn execute(&self, _args: &[String]) -> CommandResult {
        // TODO: List all variables from VariableStore
        CommandResult::Message("Listing variables...".to_string())
    }
}
