// greet command implementation
use crate::core::types::CommandResult;
use super::Command;

pub struct GreetCommand;

impl Command for GreetCommand {
    fn name(&self) -> &'static str {
        "greet"
    }
    fn execute(&self, args: &[String]) -> CommandResult {
        let name = args.get(0).cloned().unwrap_or_else(|| "stranger".to_string());
        CommandResult::Message(format!("Hello, {}!", name))
    }
}
