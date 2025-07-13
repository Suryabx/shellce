use std::path::Path;

use anyhow::{Result, Context};
use async_trait::async_trait;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::core::types::{CommandResult, CommandRegistry};

pub struct SourceCommand;

#[async_trait]
use crate::commands::command::Command;

#[async_trait]
impl Command for SourceCommand {
    fn name(&self) -> &'static str { "source" }
    fn description(&self) -> &'static str { "Sources a script file." }
    fn usage(&self) -> &'static str { "source <script_file>" }
    async fn execute(
        &self,
        args: Vec<String>,
        _var_manager: &crate::core::variables::VariableManager,
        _config: &crate::core::config::ShellConfig,
        _command_registry: &crate::core::types::CommandRegistry,
    ) -> crate::core::types::CommandResult {
        if args.is_empty() {
            println!("Usage: source <script_file>");
            return Err("No script file specified".to_string());
        }

        let script_path = &args[0];
        let file = File::open(script_path)
            .await
            .context("Failed to open script file")?;

        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        while let Ok(Some(line_result)) =
            lines.next_line().await.context("Failed to read line from script")
        {
            println!("> {}", line_result); // Optional: echo command
            // Here you would parse and dispatch the command
        }

        Ok("Sourced script".to_string())
    }
}

