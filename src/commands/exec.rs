use std::process::Command;

use crate::core::types::{CommandResult, ShellCommand};
use async_trait::async_trait;

pub struct ExecCommand;

#[async_trait]
impl ShellCommand for ExecCommand {
    async fn execute(&self, args: Vec<String>) -> CommandResult {
        if args.is_empty() {
            println!("Usage: exec <command>");
            return Ok(());
        }

        let output = Command::new(&args[0])
            .args(&args[1..])
            .output()?;

        if !output.stdout.is_empty() {
            let stdout_str = String::from_utf8_lossy(&output.stdout).to_string();
            println!("{}", stdout_str);
        }

        if !output.stderr.is_empty() {
            let stderr_str = String::from_utf8_lossy(&output.stderr).to_string();
            eprintln!("{}", stderr_str);
        }

        Ok(())
    }
}
