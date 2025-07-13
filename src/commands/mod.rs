use std::collections::HashMap;

use crate::core::types::{CommandResult, CommandRegistry};
use crate::commands::{
    greet::*, remember::*, echo::*, list_vars::*, save_load::*, help::*,
    exit::*, source::*, ping::*, sleep::*, fs::*, count::*
};

mod greet;
mod remember;
mod echo;
mod list_vars;
mod save_load;
mod help;
mod exit;
mod source;
mod ping;
mod sleep;
mod fs;
mod count;

pub fn get_command_registry() -> CommandRegistry {
    let mut registry: CommandRegistry = HashMap::new();

    registry.insert("greet".to_string(), Box::new(GreetCommand));
    registry.insert("remember".to_string(), Box::new(RememberCommand));
    registry.insert("echo".to_string(), Box::new(EchoCommand));
    registry.insert("vars".to_string(), Box::new(ListVarsCommand));
    registry.insert("save".to_string(), Box::new(SaveMemoryCommand));
    registry.insert("load".to_string(), Box::new(LoadMemoryCommand));
    registry.insert("help".to_string(), Box::new(HelpCommand));
    registry.insert("exit".to_string(), Box::new(ExitCommand));
    registry.insert("source".to_string(), Box::new(SourceCommand));
    registry.insert("ping".to_string(), Box::new(PingCommand));
    registry.insert("sleep".to_string(), Box::new(SleepCommand));
    registry.insert("ls".to_string(), Box::new(FsCommand));
    registry.insert("count".to_string(), Box::new(CountCommand));

    registry
}

