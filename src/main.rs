// src/main.rs - Main entry point for Shellce

// Declare modules
mod commands;
mod core;
mod parser;
mod storage;
mod util;

use anyhow::{Result, Context};
use rustyline::error::ReadlineError;
use colored::Colorize;
use log::{info, error, debug};
use std::path::PathBuf;

use crate::core::types::{CommandResult, ShellFlowCompleter, ShellFlowHelper}; // Note: Completer/Helper names remain generic
use crate::core::variables::VariableManager;
use crate::core::config::ShellConfig;
use crate::core::dispatcher::CommandDispatcher;
use crate::commands::get_command_registry;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize Logging
    util::init_logging();
    info!("Shellce application starting..."); // Updated log message

    // 2. Load Configuration
    let config_path = "config.toml";
    let config = match ShellConfig::load(config_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to load config from {}: {}. Using default config.", config_path, e);
            ShellConfig::default()
        }
    };
    debug!("Loaded configuration: {:?}", config);

    // 3. Initialize Variable Manager
    let var_manager = VariableManager::new();

    // 4. Initialize Command Dispatcher
    let command_registry = get_command_registry();
    let dispatcher = CommandDispatcher::new(command_registry.clone());
    debug!("Command dispatcher initialized.");

    // 5. Setup Rustyline (REPL, History, Autocompletion)
    let history_path = PathBuf::from(&config.history_file);
    let mut rl = rustyline::Editor::<ShellFlowHelper>::new()?;

    // Set up completer with initial command names and variable names
    let mut completer = ShellFlowCompleter {
        commands: command_registry.keys().cloned().collect(),
        variables: var_manager.keys(), // Initial variables
    };
    let helper = ShellFlowHelper { completer: completer };
    rl.set_helper(Some(helper));

    if rl.load_history(&history_path).is_err() {
        info!("No history file found at {:?}, starting with empty history.", history_path);
    } else {
        info!("History loaded from {:?}", history_path);
    }

    // 6. REPL Loop
    println!("{}", "Welcome to Shellce!".cyan().bold()); // Updated welcome message
    println!("{}", "Type 'help' for a list of commands, or 'exit' to quit.".yellow());

    loop {
        // Update completer's variable list before each prompt
        if let Some(helper_mut) = rl.helper_mut() {
            helper_mut.completer.variables = var_manager.keys();
        }

        let prompt = format!("{}", config.prompt.color(config.theme.prompt_color.parse().unwrap_or(colored::Color::Green)));
        let readline = rl.readline(&prompt);

        match readline {
            Ok(line) => {
                let command_line = line.trim();
                if command_line.is_empty() {
                    continue;
                }

                rl.add_history_entry(command_line)?; // Add command to history

                // Handle 'exit' command specifically to break the loop
                if command_line == "exit" {
                    let exit_cmd = command_registry.get("exit").unwrap(); // ExitCommand is always present
                    let result = exit_cmd.execute(vec![], &var_manager, &config, command_registry).await;
                    if result.success {
                        println!("{}", result.output.unwrap().message.unwrap_or_default().cyan());
                    }
                    break; // Exit the REPL loop
                }

                // Dispatch and execute the command
                let result = dispatcher.dispatch_command(command_line, &var_manager, &config).await;

                // Print the result
                if result.success {
                    if let Some(output) = result.output {
                        if let Some(msg) = output.message {
                            println!("{}", msg.color(config.theme.success_color.parse().unwrap_or(colored::Color::Cyan)));
                        }
                        if let Some(data) = output.data {
                            match serde_json::to_string_pretty(&data) {
                                Ok(json_str) => println!("{}", json_str.truecolor(150, 150, 150)), // Grey for JSON
                                Err(e) => error!("Error serializing output data: {}", e),
                            }
                        }
                    }
                } else {
                    if let Some(err_msg) = result.error_message {
                        eprintln!("{}", format!("Error: {}", err_msg).color(config.theme.error_color.parse().unwrap_or(colored::Color::Red)));
                    } else {
                        eprintln!("{}", "An unknown error occurred.".color(config.theme.error_color.parse().unwrap_or(colored::Color::Red)));
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                // Ctrl-C
                println!("{}", "Ctrl-C received. Type 'exit' to quit.".yellow());
            },
            Err(ReadlineError::Eof) => {
                // Ctrl-D
                println!("{}", "Ctrl-D received. Exiting Shellce.".yellow()); // Updated exit message
                break;
            },
            Err(err) => {
                error!("Error reading line: {:?}", err);
                eprintln!("{}", format!("Error: {:?}", err).red());
                break;
            },
        }
    }

    // 7. Save History on Exit
    if let Err(err) = rl.save_history(&history_path) {
        error!("Failed to save history to {:?}: {}", history_path, err);
        eprintln!("{}", format!("Warning: Failed to save history: {}", err).yellow());
    } else {
        info!("History saved to {:?}", history_path);
    }

    info!("Shellce application exiting."); // Updated log message
    Ok(())
}
