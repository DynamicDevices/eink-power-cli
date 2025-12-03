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
use log::{error, info, warn};
use std::process;

mod cli;
mod error;
mod power;
mod serial;

use cli::{Cli, Commands, PowerManagementCommands, PowerCommands, SystemCommands, BatteryCommands, GpioCommands, NfcCommands};
use error::PowerCliError;
use power::control::PowerController;
use serial::Connection;

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

    match cli.command {
        Some(cmd) => {
            // Create serial connection
            let mut connection = Connection::new(&cli.device, cli.baud)?;
            connection.set_timeout(cli.timeout);
            
            // Connect to serial device
            if let Err(e) = connection.connect().await {
                error!("Failed to connect to {}: {}", cli.device, e);
                return Err(e);
            }
            
            // Create power controller
            let mut power_ctrl = PowerController::new(connection);
            
            // Route command to appropriate handler
            match cmd {
                Commands::Version => {
                    let response = power_ctrl.protocol.execute_system_command("version").await?;
                    print_response(&response, &cli.format);
                }
                Commands::Ping => {
                    let response = power_ctrl.protocol.execute_system_command("ping").await?;
                    print_response(&response, &cli.format);
                }
                Commands::System(sys_cmd) => {
                    handle_system_command(&mut power_ctrl, sys_cmd, &cli.format).await?;
                }
                Commands::Power(pwr_cmd) => {
                    handle_power_command(&mut power_ctrl, pwr_cmd, &cli.format).await?;
                }
                Commands::Battery(bat_cmd) => {
                    handle_battery_command(&mut power_ctrl, bat_cmd, &cli.format).await?;
                }
                Commands::Gpio(gpio_cmd) => {
                    handle_gpio_command(&mut power_ctrl, gpio_cmd, &cli.format).await?;
                }
                Commands::Nfc(nfc_cmd) => {
                    handle_nfc_command(&mut power_ctrl, nfc_cmd, &cli.format).await?;
                }
                Commands::Pm(pm_cmd) => {
                    handle_pm_command(&mut power_ctrl, pm_cmd, &cli.format).await?;
                }
                Commands::Monitor { interval, continuous } => {
                    warn!("Monitor command not yet implemented (interval: {}s, continuous: {})", interval, continuous);
                    println!("Monitor command not yet implemented");
                }
                Commands::Batch { file } => {
                    warn!("Batch command not yet implemented (file: {:?})", file);
                    println!("Batch command not yet implemented");
                }
            }
            
            Ok(())
        }
        None => {
            // No command provided, show help
            println!("No command provided. Use --help for usage information.");
            Ok(())
        }
    }
}

/// Handle system commands
async fn handle_system_command(
    power_ctrl: &mut PowerController,
    cmd: SystemCommands,
    format: &cli::OutputFormat,
) -> Result<(), PowerCliError> {
    match cmd {
        SystemCommands::Info => {
            let response = power_ctrl.protocol.execute_system_command("system info").await?;
            print_response(&response, format);
        }
        SystemCommands::Reboot => {
            let response = power_ctrl.protocol.execute_system_command("reboot").await?;
            print_response(&response, format);
        }
        SystemCommands::Uptime => {
            let response = power_ctrl.protocol.execute_system_command("uptime").await?;
            print_response(&response, format);
        }
    }
    Ok(())
}

/// Handle power commands
async fn handle_power_command(
    power_ctrl: &mut PowerController,
    cmd: PowerCommands,
    format: &cli::OutputFormat,
) -> Result<(), PowerCliError> {
    match cmd {
        PowerCommands::Pmic { state } => {
            let response = power_ctrl.control_pmic(state.into()).await?;
            print_response(&response, format);
        }
        PowerCommands::Wifi { state } => {
            let response = power_ctrl.control_wifi(state.into()).await?;
            print_response(&response, format);
        }
        PowerCommands::Disp { state } => {
            let response = power_ctrl.control_display(state.into()).await?;
            print_response(&response, format);
        }
        PowerCommands::Stats => {
            let response = power_ctrl.protocol.execute_system_command("power stats").await?;
            print_response(&response, format);
        }
    }
    Ok(())
}

/// Handle battery commands
async fn handle_battery_command(
    power_ctrl: &mut PowerController,
    cmd: BatteryCommands,
    format: &cli::OutputFormat,
) -> Result<(), PowerCliError> {
    match cmd {
        BatteryCommands::Read => {
            let response = power_ctrl.protocol.execute_battery_command("read").await?;
            print_response(&response, format);
        }
        BatteryCommands::Status => {
            let response = power_ctrl.protocol.execute_battery_command("status").await?;
            print_response(&response, format);
        }
        BatteryCommands::Enable => {
            let response = power_ctrl.protocol.execute_battery_command("enable").await?;
            print_response(&response, format);
        }
        BatteryCommands::Disable => {
            let response = power_ctrl.protocol.execute_battery_command("disable").await?;
            print_response(&response, format);
        }
    }
    Ok(())
}

/// Handle GPIO commands
async fn handle_gpio_command(
    power_ctrl: &mut PowerController,
    cmd: GpioCommands,
    format: &cli::OutputFormat,
) -> Result<(), PowerCliError> {
    match cmd {
        GpioCommands::Get { port, pin } => {
            let response = power_ctrl.control_gpio(&port, pin, power::control::GpioAction::Get).await?;
            print_response(&response, format);
        }
        GpioCommands::Set { port, pin, value } => {
            let response = power_ctrl.control_gpio(&port, pin, power::control::GpioAction::Set(value)).await?;
            print_response(&response, format);
        }
    }
    Ok(())
}

/// Handle NFC commands
async fn handle_nfc_command(
    power_ctrl: &mut PowerController,
    cmd: NfcCommands,
    format: &cli::OutputFormat,
) -> Result<(), PowerCliError> {
    match cmd {
        NfcCommands::Status => {
            let response = power_ctrl.protocol.execute_nfc_command("status").await?;
            print_response(&response, format);
        }
        NfcCommands::Info => {
            let response = power_ctrl.protocol.execute_nfc_command("info").await?;
            print_response(&response, format);
        }
        NfcCommands::FieldDetect => {
            let response = power_ctrl.protocol.execute_nfc_command("ed").await?;
            print_response(&response, format);
        }
    }
    Ok(())
}

/// Handle power management commands
async fn handle_pm_command(
    power_ctrl: &mut PowerController,
    cmd: PowerManagementCommands,
    format: &cli::OutputFormat,
) -> Result<(), PowerCliError> {
    match cmd {
        PowerManagementCommands::Stats => {
            let response = power_ctrl.protocol.execute_system_command("pm stats").await?;
            print_response(&response, format);
        }
        PowerManagementCommands::Sleep { timeout } => {
            let cmd = if let Some(ms) = timeout {
                format!("pm deep_sleep {}", ms)
            } else {
                "pm deep_sleep".to_string()
            };
            let response = power_ctrl.protocol.execute_system_command(&cmd).await?;
            print_response(&response, format);
        }
        PowerManagementCommands::DeepSleepAllOff { timeout } => {
            let response = power_ctrl.deep_sleep_all_off(timeout).await?;
            print_response(&response, format);
        }
        PowerManagementCommands::Monitor { action, interval } => {
            warn!("PM monitor command not yet implemented (action: {:?}, interval: {:?})", action, interval);
            println!("PM monitor command not yet implemented");
        }
        PowerManagementCommands::BatteryCheck => {
            let response = power_ctrl.protocol.execute_system_command("pm battery-check").await?;
            print_response(&response, format);
        }
        PowerManagementCommands::Imx93 { state } => {
            let state_str = match state {
                cli::PowerState::On => "on",
                cli::PowerState::Off => "off",
                cli::PowerState::Status => "status",
            };
            let response = power_ctrl.protocol.execute_system_command(&format!("power imx93 {}", state_str)).await?;
            print_response(&response, format);
        }
    }
    Ok(())
}

/// Print response in requested format
fn print_response(response: &str, format: &cli::OutputFormat) {
    match format {
        cli::OutputFormat::Human => {
            println!("{}", response);
        }
        cli::OutputFormat::Json => {
            // TODO: Parse response and format as JSON
            println!("{}", serde_json::json!({
                "response": response,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }));
        }
        cli::OutputFormat::Csv => {
            // TODO: Parse response and format as CSV
            println!("timestamp,response");
            println!("{},{}", chrono::Utc::now().to_rfc3339(), response.replace('\n', " "));
        }
    }
}
