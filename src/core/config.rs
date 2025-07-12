// src/core/config.rs
// Manages application configuration loaded from config.toml.

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::fs;
use log::{info, error};

/// Represents the structure of the `config.toml` file.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShellConfig {
    #[serde(default = "default_prompt")]
    pub prompt: String,
    #[serde(default)]
    pub aliases: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub enabled_commands: Vec<String>, // For "soft" plugin system
    #[serde(default = "default_history_file")]
    pub history_file: String,
    #[serde(default)]
    pub theme: ThemeConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ThemeConfig {
    #[serde(default = "default_prompt_color")]
    pub prompt_color: String,
    #[serde(default = "default_success_color")]
    pub success_color: String,
    #[serde(default = "default_error_color")]
    pub error_color: String,
}

impl Default for ShellConfig {
    fn default() -> Self {
        ShellConfig {
            prompt: default_prompt(),
            aliases: std::collections::HashMap::new(),
            enabled_commands: Vec::new(), // By default, all registered commands are enabled
            history_file: default_history_file(),
            theme: ThemeConfig::default(),
        }
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        ThemeConfig {
            prompt_color: default_prompt_color(),
            success_color: default_success_color(),
            error_color: default_error_color(),
        }
    }
}

fn default_prompt() -> String {
    "shellflow> ".to_string()
}

fn default_history_file() -> String {
    "shellflow_history.txt".to_string()
}

fn default_prompt_color() -> String {
    "green".to_string()
}

fn default_success_color() -> String {
    "cyan".to_string()
}

fn default_error_color() -> String {
    "red".to_string()
}

impl ShellConfig {
    /// Loads the configuration from the specified TOML file.
    pub fn load(path: &str) -> Result<Self> {
        info!("Attempting to load configuration from: {}", path);
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path))?;
        let config: ShellConfig = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path))?;
        info!("Configuration loaded successfully.");
        Ok(config)
    }

    /// Saves the current configuration to the specified TOML file.
    pub fn save(&self, path: &str) -> Result<()> {
        info!("Attempting to save configuration to: {}", path);
        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config to TOML")?;
        fs::write(path, content)
            .with_context(|| format!("Failed to write config file: {}", path))?;
        info!("Configuration saved successfully to: {}", path);
        Ok(())
    }
}
