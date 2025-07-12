// src/commands/exec.rs
// Implements the `exec` command to run external programs.

use anyhow::Result;
use async_trait::async_trait;
use serde_json::json;
use tokio::process::Command as TokioCommand; // Alias to avoid conflict with crate::commands::Command
use log::{info, error, debug};

use crate::commands::Command;
use crate::core::types::{CommandResult, CommandRegistry};
use crate::core::variables::VariableManager;
use crate::core::config::ShellConfig;

pub struct ExecCommand;

#[async_trait]
impl Command for ExecCommand {
    fn name(&self) -> &'static str {
        "exec"
    }

    fn description(&self) -> &'static str {
        "Executes an external program with provided arguments."
    }

    fn usage(&self) -> &'static str {
        "exec <program> [args...]"
    }

    async fn execute(
        &self,
        args: Vec<String>,
        _var_manager: &VariableManager,
        _config: &ShellConfig,
        _command_registry: &CommandRegistry,
    ) -> CommandResult {
        if args.is_empty() {
            return CommandResult::error(format!("Invalid usage. {}", self.usage()));
        }

        let program = &args[0];
        let program_args = &args[1..];

        info!("Executing external program: '{}' with args: {:?}", program, program_args);

        // Use Tokio's Command for asynchronous process execution
        let mut command = TokioCommand::new(program);
        command.args(program_args);

        match command.output().await {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();

                debug!("Program '{}' stdout: {}", program, stdout);
                if !stderr.is_empty() {
                    debug!("Program '{}' stderr: {}", program, stderr);
                }

                if output.status.success() {
                    CommandResult::success(
                        Some(stdout),
                        Some(json!({
                            "program": program,
                            "args": program_args,
                            "status": output.status.code(),
                            "stdout": stdout,
                            "stderr": stderr
                        })),
                    )
                } else {
                    let error_msg = format!(
                        "Program '{}' exited with non-zero status: {:?}\nStderr: {}",
                        program,
                        output.status.code(),
                        stderr
                    );
                    error!("External program failed: {}", error_msg);
                    CommandResult::error(error_msg)
                }
            },
            Err(e) => {
                error!("Failed to execute program '{}': {}", program, e);
                CommandResult::error(format!("Failed to execute program '{}': {}", program, e))
            },
        }
    }
}