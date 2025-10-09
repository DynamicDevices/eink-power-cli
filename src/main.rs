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
mod firmware;
mod json;
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

/// Output a response in the requested format
fn output_response(cli: &Cli, command: &str, response: &str, emoji: &str, title: &str) -> Result<(), PowerCliError> {
    if cli.quiet {
        return Ok(());
    }

    match cli.format {
        cli::OutputFormat::Human => {
            println!("{} {}:", emoji, title);
            println!("{}", response);
        }
        cli::OutputFormat::Json => {
            // Try to parse the response into structured JSON based on command type
            let json_data = match command {
                cmd if cmd.contains("battery") || cmd.contains("coulomb") => {
                    let battery_data = json::ResponseParser::parse_battery_response(response);
                    serde_json::to_value(battery_data)?
                }
                cmd if cmd.contains("system") || cmd.contains("version") => {
                    let system_data = json::ResponseParser::parse_system_info(response);
                    serde_json::to_value(system_data)?
                }
                cmd if cmd.contains("nfc") => {
                    let nfc_data = json::ResponseParser::parse_nfc_status(response);
                    serde_json::to_value(nfc_data)?
                }
                cmd if cmd.contains("ltc2959") => {
                    let ltc_data = json::ResponseParser::parse_ltc2959_status(response);
                    serde_json::to_value(ltc_data)?
                }
                cmd if cmd.contains("gpio") => {
                    // For GPIO, we need to extract port and pin from the command
                    // This is a simplified approach - in a real implementation, you'd pass these as parameters
                    let gpio_data = json::ResponseParser::parse_gpio_response(response, "unknown", 0);
                    serde_json::to_value(gpio_data)?
                }
                _ => {
                    // Generic response - just wrap the raw text
                    serde_json::json!({
                        "raw_response": response,
                        "parsed": false
                    })
                }
            };

            let json_response = json::JsonResponse::success_with_raw(command, json_data, response);
            println!("{}", serde_json::to_string_pretty(&json_response)?);
        }
        cli::OutputFormat::Csv => {
            // CSV format - simplified implementation
            println!("timestamp,command,status,response");
            println!("{},{},success,\"{}\"", chrono::Utc::now().to_rfc3339(), command, response.replace("\"", "\"\""));
        }
    }

    Ok(())
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
            output_response(cli, "version", &response, "🔧", "PMU Controller Version")?;
        }
        Commands::Ping => {
            let response = controller.ping().await?;
            output_response(cli, "ping", &response, "🏓", "Ping response")?;
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
                PowerCommands::Coulomb => {
                    let response = controller.get_coulomb_counter().await?;
                    output_response(cli, "power coulomb", &response, "🔋", "Coulomb Counter")?;
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
        Commands::System(system_cmd) => {
            use cli::SystemCommands;
            match system_cmd {
                SystemCommands::Info => {
                    let response = controller.get_system_info_detailed().await?;
                    output_response(cli, "system info", &response, "🖥️", "System Information")?;
                }
                SystemCommands::Reboot => {
                    let response = controller.reboot_system().await?;
                    output_response(cli, "system reboot", &response, "🔄", "System Reboot")?;
                }
                SystemCommands::Uptime => {
                    let response = controller.get_system_uptime().await?;
                    output_response(cli, "system uptime", &response, "⏱️", "System Uptime")?;
                }
            }
        }
        Commands::Battery(battery_cmd) => {
            use cli::BatteryCommands;
            match battery_cmd {
                BatteryCommands::Read => {
                    let response = controller.battery_read().await?;
                    output_response(cli, "battery read", &response, "🔋", "Battery Measurements")?;
                }
                BatteryCommands::Status => {
                    let response = controller.battery_status().await?;
                    output_response(cli, "battery status", &response, "📋", "Battery Status")?;
                }
                BatteryCommands::Enable => {
                    let response = controller.battery_enable().await?;
                    output_response(cli, "battery enable", &response, "✅", "Battery Monitoring Enabled")?;
                }
                BatteryCommands::Disable => {
                    let response = controller.battery_disable().await?;
                    output_response(cli, "battery disable", &response, "❌", "Battery Monitoring Disabled")?;
                }
            }
        }
        Commands::Pm(pm_cmd) => {
            use cli::PowerManagementCommands;
            match pm_cmd {
                PowerManagementCommands::Stats => {
                    let response = controller.pm_stats().await?;
                    if !cli.quiet {
                        println!("📊 Power Management Statistics:");
                        println!("{}", response);
                    }
                }
                PowerManagementCommands::Sleep { timeout } => {
                    let cmd = if let Some(timeout_ms) = timeout {
                        format!("deep_sleep {}", timeout_ms)
                    } else {
                        "deep_sleep".to_string()
                    };
                    let response = controller.pm_command(&cmd).await?;
                    if !cli.quiet {
                        println!("😴 Entering Deep Sleep:");
                        println!("{}", response);
                    }
                }
                PowerManagementCommands::Monitor { action, interval } => {
                    let cmd = match action {
                        cli::MonitorAction::Start => {
                            if let Some(interval_s) = interval {
                                format!("monitor start {}", interval_s)
                            } else {
                                "monitor start".to_string()
                            }
                        }
                        cli::MonitorAction::Stop => "monitor stop".to_string(),
                    };
                    let response = controller.pm_command(&cmd).await?;
                    if !cli.quiet {
                        println!("📊 Power Monitoring:");
                        println!("{}", response);
                    }
                }
                PowerManagementCommands::BatteryCheck => {
                    let response = controller.pm_command("battery_check").await?;
                    if !cli.quiet {
                        println!("🔋 Battery Health Check:");
                        println!("{}", response);
                    }
                }
                PowerManagementCommands::Imx93 { state } => {
                    let cmd = match state {
                        cli::PowerState::On => "imx93 on",
                        cli::PowerState::Off => "imx93 off", 
                        cli::PowerState::Status => "imx93 status",
                    };
                    let response = controller.pm_command(cmd).await?;
                    if !cli.quiet {
                        println!("🖥️ i.MX93 Power Control:");
                        println!("{}", response);
                    }
                }
            }
        }
        Commands::Nfc(nfc_cmd) => {
            use cli::NfcCommands;
            match nfc_cmd {
                NfcCommands::Status => {
                    let response = controller.nfc_command("status").await?;
                    if !cli.quiet {
                        println!("📡 NFC Status:");
                        println!("{}", response);
                    }
                }
                NfcCommands::Info => {
                    let response = controller.nfc_command("info").await?;
                    if !cli.quiet {
                        println!("ℹ️ NFC Device Information:");
                        println!("{}", response);
                    }
                }
                NfcCommands::FieldDetect => {
                    let response = controller.nfc_command("field_detect").await?;
                    if !cli.quiet {
                        println!("📡 NFC Field Detection:");
                        println!("{}", response);
                    }
                }
            }
        }
        Commands::Firmware(firmware_cmd) => {
            use cli::FirmwareCommands;
            
            // Extract port and baud from the command
            let (port, baud) = match firmware_cmd {
                FirmwareCommands::Upload { ref port, baud, .. } => (port.clone(), baud.unwrap_or(115200)),
                _ => (None, 115200),
            };
            
            let connection = serial::Connection::new(&cli.device, cli.baud, cli.quiet)?;
            let mut firmware_manager = firmware::FirmwareManager::new(connection, port, baud);
            
            match firmware_cmd {
                FirmwareCommands::List => {
                    let response = firmware_manager.list_images().await?;
                    output_response(cli, "firmware list", &response, "📋", "Firmware Images")?;
                }
                FirmwareCommands::Info => {
                    let response = firmware_manager.get_info().await?;
                    output_response(cli, "firmware info", &response, "ℹ️", "Firmware Information")?;
                }
                FirmwareCommands::Reset => {
                    let response = firmware_manager.reset_to_bootloader().await?;
                    output_response(cli, "firmware reset", &response, "🔄", "Bootloader Reset")?;
                }
                FirmwareCommands::Upload { file, skip_reset, .. } => {
                    let response = firmware_manager.upload_firmware(file.as_path(), skip_reset).await?;
                    output_response(cli, "firmware upload", &response, "⬆️", "Firmware Upload")?;
                }
            }
        }
        _ => {
            println!("Command not yet implemented: {:?}", command);
        }
    }
    
    Ok(())
}
