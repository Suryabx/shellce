// src/util/mod.rs
// Contains general utility functions for the ShellFlow application.

use log::LevelFilter;
use env_logger::{Builder, Target};

/// Initializes the logging system.
/// This should be called once at the start of the application.
pub fn init_logging() {
    Builder::new()
        .filter_level(LevelFilter::Info) // Set default log level
        .target(Target::Stdout) // Log to standard output
        .init();
}
