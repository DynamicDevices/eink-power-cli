/*
 * E-ink Power CLI - Command Line Interface for Power Management Controller
 * Copyright (c) 2025 Dynamic Devices Ltd
 * All rights reserved.
 *
 * This software is proprietary to Dynamic Devices Ltd and may not be
 * reproduced, distributed, or disclosed without written permission.
 *
 * Maintainer: Alex J Lennon <ajlennon@dynamicdevices.co.uk>
 * Company: Dynamic Devices Ltd
 * Contact: info@dynamicdevices.co.uk
 */

use clap::Parser;
use env_logger;
use log::{error, info};
use std::process;

mod cli;
mod error;
mod power;
mod serial;

use cli::Cli;
use error::PowerCliError;

/// Application version from Cargo.toml
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Application name
const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Print version header
    if !cli.quiet {
        println!("{} v{}", APP_NAME, VERSION);
        println!("Copyright (c) 2025 Dynamic Devices Ltd");
        println!();
    }

    // Execute the command
    if let Err(e) = run(cli).await {
        error!("Command failed: {}", e);

        // Print user-friendly error message
        eprintln!("Error: {}", e);

        // Exit with error code
        process::exit(1);
    }
}

/// Main application logic
async fn run(cli: Cli) -> Result<(), PowerCliError> {
    info!("Starting eink-power-cli v{}", VERSION);

    // TODO: Implement command execution
    // This will be implemented in Phase 1
    match cli.command {
        Some(cmd) => {
            info!("Executing command: {:?}", cmd);
            // Command execution will be implemented here
            println!("Command execution not yet implemented");
            Ok(())
        }
        None => {
            // No command provided, show help
            println!("No command provided. Use --help for usage information.");
            Ok(())
        }
    }
}
