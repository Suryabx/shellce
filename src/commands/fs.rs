use std::path::Path;
use anyhow::{Result, Context};
use async_trait::async_trait;
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::core::types::{CommandResult, CommandRegistry};

pub struct FsCommand;

#[async_trait]
use crate::commands::command::Command;

#[async_trait]
impl Command for FsCommand {
    fn name(&self) -> &'static str { "ls" }
    fn description(&self) -> &'static str { "Lists files in a directory." }
    fn usage(&self) -> &'static str { "ls <directory>" }
    async fn execute(
        &self,
        args: Vec<String>,
        _var_manager: &crate::core::variables::VariableManager,
        _config: &crate::core::config::ShellConfig,
        _command_registry: &crate::core::types::CommandRegistry,
    ) -> crate::core::types::CommandResult {
        if args.is_empty() {
            println!("Usage: ls <directory>");
            return Err("No directory specified".to_string());
        }

        let dir_path = &args[0];
        let mut entries = fs::read_dir(dir_path)
            .await
            .context("Failed to read directory")?;

        println!("Contents of {}:", dir_path);

        while let Ok(Some(entry)) =
            entries.next_entry().await.context("Failed to read directory entry")
        {
            let file_name = entry
                .file_name()
                .into_string()
                .unwrap_or_else(|_| "<invalid utf-8>".to_string());
            println!(" - {}", file_name);
        }

        Ok("Listed files".to_string())
    }
}
