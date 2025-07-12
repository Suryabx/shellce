// source command for script loading
use crate::core::types::CommandResult;
use super::Command;

pub struct SourceCommand;

impl Command for SourceCommand {
    fn name(&self) -> &'static str {
        "source"
    }
    fn execute(&self, args: &[String]) -> CommandResult {
        // TODO: Load and execute script file
        CommandResult::Message(format!("Sourced file: {}", args.get(0).unwrap_or(&"<none>".to_string())) )
    }
}
