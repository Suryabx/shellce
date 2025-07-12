// echo command implementation
use crate::core::types::CommandResult;
use super::Command;

pub struct EchoCommand;

impl Command for EchoCommand {
    fn name(&self) -> &'static str {
        "echo"
    }
    fn execute(&self, args: &[String]) -> CommandResult {
        CommandResult::Message(args.join(" "))
    }
}
