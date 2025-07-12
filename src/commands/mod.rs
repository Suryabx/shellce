// Defines Command trait, registers all commands
pub mod greet;
pub mod remember;
pub mod echo;
pub mod list_vars;
pub mod save_load;
pub mod help;
pub mod exit;
pub mod source;

pub trait Command {
    fn name(&self) -> &'static str;
    fn execute(&self, args: &[String]) -> crate::core::types::CommandResult;
}
