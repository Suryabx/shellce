use std::path::PathBuf;

use rustyline::{Editor};
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use colored::Colorize;
use log::{info, error};

use crate::core::config::ShellConfig;
use crate::core::dispatcher::CommandDispatcher;
use crate::core::variables::VariableManager;
use crate::core::types::{ShellFlowHelper, ShellFlowCompleter};

use crate::commands::get_command_registry;

mod core;
mod parser;
mod commands;
mod storage;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    util::init_logging();
    info!("Shellce application starting...");

    let config_path = PathBuf::from("config.json");
    let config = match ShellConfig::load(config_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to load config: {}. Using default config.", e);
            ShellConfig::default()
        }
    };

    let var_manager = VariableManager::new();
    let command_registry = get_command_registry();
    let dispatcher = CommandDispatcher::new(command_registry.clone());

    let history_path = PathBuf::from(&config.history_file);

    let completer = ShellFlowCompleter {
        commands: command_registry.keys().cloned().collect(),
        variables: vec![],
    };

    let helper = ShellFlowHelper { completer };
    let mut rl = Editor::<ShellFlowHelper, DefaultHistory>::new()?;
    rl.set_helper(Some(helper));

    if rl.load_history(&history_path).is_err() {
        info!("No history file found, starting fresh.");
    }

    println!("{}", "Welcome to Shellce!".cyan().bold());
    println!("{}", "Type 'help' or 'exit'.".yellow());

    loop {
        let readline = rl.readline("shellce> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                if let Err(e) = dispatcher.handle_command(&line, &var_manager).await {
                    error!("Command error: {:?}", e);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("Exiting...");
                break;
            }
            Err(err) => {
                error!("Readline error: {:?}", err);
                break;
            }
        }
    }

    rl.append_history(&history_path)?;

    Ok(())
}
