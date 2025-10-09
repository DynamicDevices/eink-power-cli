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
use log::{debug, error};
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
    // Parse command line arguments first to get verbose flag
    let cli = Cli::parse();

    // Initialize logging based on verbose flag
    let log_level = if cli.verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Warn
    };

    env_logger::Builder::from_default_env()
        .filter_level(log_level)
        .init();

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
    debug!("Starting eink-power-cli v{}", VERSION);

    // Create serial connection
    let connection = serial::Connection::new(&cli.device, cli.baud, cli.quiet)?;
    let mut power_controller = power::control::PowerController::new(connection);

    match cli.command {
        Some(ref cmd) => {
            debug!("Executing command: {:?}", cmd);
            execute_command(cmd.clone(), &mut power_controller, &cli).await?;
            Ok(())
        }
        None => {
            // No command provided, show help
            println!("No command provided. Use --help for usage information.");
            Ok(())
        }
    }
}

/// Execute a specific command
async fn execute_command(
    command: cli::Commands,
    controller: &mut power::control::PowerController,
    cli: &Cli,
) -> Result<(), PowerCliError> {
    use cli::Commands;
    
    match command {
        Commands::Version => {
            let response = controller.get_system_info().await?;
            if !cli.quiet {
                println!("🔧 PMU Controller Version:");
                println!("{}", response);
            }
        }
        Commands::Ping => {
            let response = controller.ping().await?;
            if !cli.quiet {
                println!("🏓 Ping response: {}", response);
            }
        }
        Commands::Board(board_cmd) => {
            use cli::BoardCommands;
            match board_cmd {
                BoardCommands::Reset => {
                    let response = controller.control_board(power::control::BoardAction::Reset).await?;
                    if !cli.quiet {
                        println!("🔄 Board reset initiated:");
                        println!("{}", response);
                    }
                }
            }
        }
        Commands::Ltc2959(ltc2959_cmd) => {
            use cli::Ltc2959Commands;
            match ltc2959_cmd {
                Ltc2959Commands::Init => {
                    let response = controller.control_ltc2959("init").await?;
                    if !cli.quiet {
                        println!("🔋 LTC2959 Initialization:");
                        println!("{}", response);
                    }
                }
                Ltc2959Commands::Read => {
                    let response = controller.control_ltc2959("read").await?;
                    if !cli.quiet {
                        println!("📊 LTC2959 Readings:");
                        println!("{}", response);
                    }
                }
                Ltc2959Commands::Status => {
                    let response = controller.control_ltc2959("status").await?;
                    if !cli.quiet {
                        println!("📋 LTC2959 Status:");
                        println!("{}", response);
                    }
                }
                Ltc2959Commands::Enable => {
                    let response = controller.control_ltc2959("enable").await?;
                    if !cli.quiet {
                        println!("✅ LTC2959 Enabled:");
                        println!("{}", response);
                    }
                }
                Ltc2959Commands::Disable => {
                    let response = controller.control_ltc2959("disable").await?;
                    if !cli.quiet {
                        println!("❌ LTC2959 Disabled:");
                        println!("{}", response);
                    }
                }
                Ltc2959Commands::Scan => {
                    let response = controller.control_ltc2959("scan").await?;
                    if !cli.quiet {
                        println!("🔍 LTC2959 I2C Scan:");
                        println!("{}", response);
                    }
                }
                Ltc2959Commands::CcGpio { state } => {
                    let cmd = match state {
                        cli::PowerState::On => "cc_gpio on",
                        cli::PowerState::Off => "cc_gpio off",
                        cli::PowerState::Status => "cc_gpio status",
                    };
                    let response = controller.control_ltc2959(cmd).await?;
                    if !cli.quiet {
                        println!("🔌 LTC2959 CC_GPIO:");
                        println!("{}", response);
                    }
                }
                Ltc2959Commands::ProductionReset => {
                    let response = controller.control_ltc2959("production_reset").await?;
                    if !cli.quiet {
                        println!("🏭 LTC2959 Production Reset:");
                        println!("{}", response);
                    }
                }
            }
        }
        Commands::Power(power_cmd) => {
            use cli::{PowerCommands, PowerState};
            match power_cmd {
                PowerCommands::Pmic { state } => {
                    let power_state = match state {
                        PowerState::On => power::control::PowerState::On,
                        PowerState::Off => power::control::PowerState::Off,
                        PowerState::Status => power::control::PowerState::Status,
                    };
                    let response = controller.control_pmic(power_state).await?;
                    if !cli.quiet {
                        println!("⚡ PMIC Control:");
                        println!("{}", response);
                    }
                }
                PowerCommands::Wifi { state } => {
                    let power_state = match state {
                        PowerState::On => power::control::PowerState::On,
                        PowerState::Off => power::control::PowerState::Off,
                        PowerState::Status => power::control::PowerState::Status,
                    };
                    let response = controller.control_wifi(power_state).await?;
                    if !cli.quiet {
                        println!("📶 WiFi Control:");
                        println!("{}", response);
                    }
                }
                PowerCommands::Disp { state } => {
                    let power_state = match state {
                        PowerState::On => power::control::PowerState::On,
                        PowerState::Off => power::control::PowerState::Off,
                        PowerState::Status => power::control::PowerState::Status,
                    };
                    let response = controller.control_display(power_state).await?;
                    if !cli.quiet {
                        println!("🖥️ Display Control:");
                        println!("{}", response);
                    }
                }
                PowerCommands::Stats => {
                    let stats = controller.get_power_stats().await?;
                    if !cli.quiet {
                        println!("{}", stats.format_human());
                    }
                }
            }
        }
        Commands::Gpio(gpio_cmd) => {
            use cli::GpioCommands;
            match gpio_cmd {
                GpioCommands::Get { port, pin } => {
                    let response = controller.control_gpio(&port, pin, power::control::GpioAction::Get).await?;
                    if !cli.quiet {
                        println!("📌 GPIO {}{}:", port, pin);
                        println!("{}", response);
                    }
                }
                GpioCommands::Set { port, pin, value } => {
                    let response = controller.control_gpio(&port, pin, power::control::GpioAction::Set(value)).await?;
                    if !cli.quiet {
                        println!("📌 GPIO {}{} set to {}:", port, pin, value);
                        println!("{}", response);
                    }
                }
            }
        }
        _ => {
            println!("Command not yet implemented: {:?}", command);
        }
    }
    
    Ok(())
}
