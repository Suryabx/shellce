// src/commands/fs.rs
// Implements core file system commands: ls, create file, read file, delete file, cd, pwd.

use anyhow::{Result, Context};
use async_trait::async_trait;
use serde_json::json;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::path::PathBuf;
use chrono::{DateTime, Local};
use log::{info, error, debug};

use crate::commands::Command;
use crate::core::types::{CommandResult, CommandRegistry};
use crate::core::variables::VariableManager;
use crate::core::config::ShellConfig;

// --- ls command ---
pub struct LsCommand;

#[async_trait]
impl Command for LsCommand {
    fn name(&self) -> &'static str {
        "ls"
    }

    fn description(&self) -> &'static str {
        "Lists files and directories in the current directory."
    }

    fn usage(&self) -> &'static str {
        "ls [--json]"
    }

    async fn execute(
        &self,
        args: Vec<String>,
        _var_manager: &VariableManager,
        _config: &ShellConfig,
        _command_registry: &CommandRegistry,
    ) -> CommandResult {
        let use_json_output = args.contains(&"--json".to_string());
        let current_dir = PathBuf::from("."); // Represents the current working directory

        info!("Executing ls command in {:?}", current_dir);

        let mut entries = match fs::read_dir(&current_dir).await {
            Ok(dir) => dir,
            Err(e) => {
                error!("Failed to read directory {:?}: {}", current_dir, e);
                return CommandResult::error(format!("Failed to list directory: {}", e));
            }
        };

        let mut file_list = Vec::new();
        let mut output_message = String::new();

        // Add headers for non-JSON output
        if !use_json_output {
            output_message.push_str(&format!(
                "{:<20} {:<10} {:>10} {:<20}\n",
                "Name", "Type", "Size", "Created At"
            ));
            output_message.push_str(&format!(
                "{:-<20} {:-<10} {:-<10} {:-<20}\n",
                "", "", "", ""
            ));
        }


        while let Some(entry_res) = entries.next_entry().await.context("Failed to read directory entry") {
            let entry = match entry_res {
                Ok(e) => e,
                Err(e) => {
                    error!("Error reading directory entry: {}", e);
                    continue; // Skip problematic entry
                }
            };

            let path = entry.path();
            let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            let metadata = match entry.metadata().await {
                Ok(m) => m,
                Err(e) => {
                    error!("Failed to get metadata for {:?}: {}", path, e);
                    continue; // Skip entry if metadata can't be read
                }
            };

            let file_type = if metadata.is_dir() {
                "directory".to_string()
            } else if metadata.is_file() {
                "file".to_string()
            } else {
                "other".to_string()
            };

            let size = metadata.len();
            let created_at = metadata.created().ok().map(|st| {
                let datetime: DateTime<Local> = st.into();
                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
            });

            if use_json_output {
                file_list.push(json!({
                    "name": file_name,
                    "type": file_type,
                    "size": size,
                    "created_at": created_at,
                }));
            } else {
                output_message.push_str(&format!(
                    "{:<20} {:<10} {:>10} {:<20}\n",
                    file_name,
                    file_type,
                    size,
                    created_at.unwrap_or_else(|| "N/A".to_string())
                ));
            }
        }

        if use_json_output {
            CommandResult::success(
                Some("Directory listing (JSON):".to_string()),
                Some(json!({ "files": file_list })),
            )
        } else {
            CommandResult::success(
                Some(output_message),
                None,
            )
        }
    }
}

// --- create file command ---
pub struct CreateFileCommand;

#[async_trait]
impl Command for CreateFileCommand {
    fn name(&self) -> &'static str {
        "create" // Primary command name
    }

    fn description(&self) -> &'static str {
        "Creates a new empty file."
    }

    fn usage(&self) -> &'static str {
        "create file <filename>"
    }

    async fn execute(
        &self,
        args: Vec<String>,
        _var_manager: &VariableManager,
        _config: &ShellConfig,
        _command_registry: &CommandRegistry,
    ) -> CommandResult {
        if args.len() < 2 || args[0] != "file" {
            return CommandResult::error(format!("Invalid usage. {}", self.usage()));
        }
        let filename = args[1].clone();
        let path = PathBuf::from(&filename);

        info!("Attempting to create file: {:?}", path);

        if path.exists() {
            error!("File already exists: {:?}", path);
            return CommandResult::error(format!("File '{}' already exists.", filename));
        }

        match fs::File::create(&path).await {
            Ok(_) => {
                info!("File created successfully: {:?}", path);
                CommandResult::success(
                    Some(format!("File '{}' created successfully.", filename)),
                    Some(json!({ "filename": filename, "path": path.to_string_lossy() })),
                )
            },
            Err(e) => {
                error!("Failed to create file {:?}: {}", path, e);
                CommandResult::error(format!("Failed to create file '{}': {}", filename, e))
            },
        }
    }
}

// --- read file command ---
pub struct ReadFileCommand;

#[async_trait]
impl Command for ReadFileCommand {
    fn name(&self) -> &'static str {
        "read" // Primary command name
    }

    fn description(&self) -> &'static str {
        "Reads the content of a file."
    }

    fn usage(&self) -> &'static str {
        "read file <filename>"
    }

    async fn execute(
        &self,
        args: Vec<String>,
        _var_manager: &VariableManager,
        _config: &ShellConfig,
        _command_registry: &CommandRegistry,
    ) -> CommandResult {
        if args.len() < 2 || args[0] != "file" {
            return CommandResult::error(format!("Invalid usage. {}", self.usage()));
        }
        let filename = args[1].clone();
        let path = PathBuf::from(&filename);

        info!("Attempting to read file: {:?}", path);

        if !path.exists() {
            error!("File not found: {:?}", path);
            return CommandResult::error(format!("File '{}' not found.", filename));
        }
        if path.is_dir() {
            error!("Path is a directory, not a file: {:?}", path);
            return CommandResult::error(format!("'{}' is a directory, not a file.", filename));
        }

        match fs::read_to_string(&path).await {
            Ok(content) => {
                info!("File read successfully: {:?}", path);
                CommandResult::success(
                    Some(format!("Content of '{}':\n{}", filename, content)),
                    Some(json!({ "filename": filename, "content": content })),
                )
            },
            Err(e) => {
                error!("Failed to read file {:?}: {}", path, e);
                CommandResult::error(format!("Failed to read file '{}': {}", filename, e))
            },
        }
    }
}

// --- delete file command ---
pub struct DeleteFileCommand;

#[async_trait]
impl Command for DeleteFileCommand {
    fn name(&self) -> &'static str {
        "delete" // Primary command name
    }

    fn description(&self) -> &'static str {
        "Deletes a file."
    }

    fn usage(&self) -> &'static str {
        "delete file <filename>"
    }

    async fn execute(
        &self,
        args: Vec<String>,
        _var_manager: &VariableManager,
        _config: &ShellConfig,
        _command_registry: &CommandRegistry,
    ) -> CommandResult {
        if args.len() < 2 || args[0] != "file" {
            return CommandResult::error(format!("Invalid usage. {}", self.usage()));
        }
        let filename = args[1].clone();
        let path = PathBuf::from(&filename);

        info!("Attempting to delete file: {:?}", path);

        if !path.exists() {
            error!("File not found for deletion: {:?}", path);
            return CommandResult::error(format!("File '{}' not found.", filename));
        }
        if path.is_dir() {
            error!("Cannot delete directory using 'delete file': {:?}", path);
            return CommandResult::error(format!("'{}' is a directory. Use 'delete dir' (if implemented) to remove directories.", filename));
        }

        match fs::remove_file(&path).await {
            Ok(_) => {
                info!("File deleted successfully: {:?}", path);
                CommandResult::success(
                    Some(format!("File '{}' deleted successfully.", filename)),
                    Some(json!({ "filename": filename, "path": path.to_string_lossy() })),
                )
            },
            Err(e) => {
                error!("Failed to delete file {:?}: {}", path, e);
                CommandResult::error(format!("Failed to delete file '{}': {}", filename, e))
            },
        }
    }
}

// --- cd command ---
pub struct CdCommand;

#[async_trait]
impl Command for CdCommand {
    fn name(&self) -> &'static str {
        "cd"
    }

    fn description(&self) -> &'static str {
        "Changes the current working directory."
    }

    fn usage(&self) -> &'static str {
        "cd [path]"
    }

    async fn execute(
        &self,
        args: Vec<String>,
        _var_manager: &VariableManager,
        _config: &ShellConfig,
        _command_registry: &CommandRegistry,
    ) -> CommandResult {
        let path_str = if args.is_empty() {
            "~".to_string() // Default to home directory if no path is given
        } else {
            args[0].clone()
        };

        let path = PathBuf::from(&path_str);
        info!("Attempting to change directory to: {:?}", path);

        // Resolve "~" to home directory
        let expanded_path = if path_str == "~" {
            dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"))
        } else {
            path
        };

        match std::env::set_current_dir(&expanded_path) {
            Ok(_) => {
                let new_current_dir = std::env::current_dir().unwrap_or_default();
                info!("Changed directory to: {:?}", new_current_dir);
                CommandResult::success(
                    Some(format!("Changed directory to: {}", new_current_dir.display())),
                    Some(json!({ "new_directory": new_current_dir.to_string_lossy() })),
                )
            },
            Err(e) => {
                error!("Failed to change directory to {:?}: {}", expanded_path, e);
                CommandResult::error(format!("Failed to change directory to '{}': {}", path_str, e))
            },
        }
    }
}

// --- pwd command ---
pub struct PwdCommand;

#[async_trait]
impl Command for PwdCommand {
    fn name(&self) -> &'static str {
        "pwd"
    }

    fn description(&self) -> &'static str {
        "Prints the current working directory."
    }

    fn usage(&self) -> &'static str {
        "pwd"
    }

    async fn execute(
        &self,
        _args: Vec<String>,
        _var_manager: &VariableManager,
        _config: &ShellConfig,
        _command_registry: &CommandRegistry,
    ) -> CommandResult {
        match std::env::current_dir() {
            Ok(path) => {
                info!("Current directory: {:?}", path);
                CommandResult::success(
                    Some(path.to_string_lossy().to_string()),
                    Some(json!({ "current_directory": path.to_string_lossy() })),
                )
            },
            Err(e) => {
                error!("Failed to get current directory: {}", e);
                CommandResult::error(format!("Failed to get current directory: {}", e))
            },
        }
    }
}
