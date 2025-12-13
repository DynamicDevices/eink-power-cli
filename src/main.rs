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
fn output_response(
    cli: &Cli,
    command: &str,
    response: &str,
    emoji: &str,
    title: &str,
) -> Result<(), PowerCliError> {
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
                    let gpio_data =
                        json::ResponseParser::parse_gpio_response(response, "unknown", 0);
                    serde_json::to_value(gpio_data)?
                }
                cmd if cmd.contains("rtc") => {
                    let rtc_data = json::ResponseParser::parse_rtc_status(response);
                    serde_json::to_value(rtc_data)?
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
            println!(
                "{},{},success,\"{}\"",
                chrono::Utc::now().to_rfc3339(),
                command,
                response.replace("\"", "\"\"")
            );
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
                    let response = controller
                        .control_board(power::control::BoardAction::Reset)
                        .await?;
                    if !cli.quiet {
                        println!("🔄 Board reset initiated:");
                        println!("{}", response);
                    }
                }
                BoardCommands::Shutdown => {
                    let response = controller
                        .control_board(power::control::BoardAction::Shutdown)
                        .await?;
                    if !cli.quiet {
                        println!("🔌 Board shutdown initiated:");
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
                Ltc2959Commands::SetCharge { charge } => {
                    let response = controller.control_ltc2959(&format!("set_charge {}", charge)).await?;
                    if !cli.quiet {
                        println!("🔋 LTC2959 Set Charge:");
                        println!("{}", response);
                    }
                }
                Ltc2959Commands::ChargeComplete => {
                    let response = controller.control_ltc2959("charge_complete").await?;
                    if !cli.quiet {
                        println!("🔋 LTC2959 Charge Complete:");
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
                Ltc2959Commands::AdcMode { mode } => {
                    let response = controller.control_ltc2959(&format!("adc_mode {}", mode)).await?;
                    if !cli.quiet {
                        println!("🔧 LTC2959 ADC Mode:");
                        println!("{}", response);
                    }
                }
                Ltc2959Commands::RegRead { address } => {
                    let response = controller.control_ltc2959(&format!("reg_read {}", address)).await?;
                    if !cli.quiet {
                        println!("📖 LTC2959 Register Read:");
                        println!("{}", response);
                    }
                }
                Ltc2959Commands::RegWrite { address, value } => {
                    let response = controller.control_ltc2959(&format!("reg_write {} {}", address, value)).await?;
                    if !cli.quiet {
                        println!("✍️ LTC2959 Register Write:");
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
                    let response = controller
                        .control_gpio(&port, pin, power::control::GpioAction::Get)
                        .await?;
                    if !cli.quiet {
                        println!("📌 GPIO {}{}:", port, pin);
                        println!("{}", response);
                    }
                }
                GpioCommands::Set { port, pin, value } => {
                    let response = controller
                        .control_gpio(&port, pin, power::control::GpioAction::Set(value))
                        .await?;
                    if !cli.quiet {
                        println!("📌 GPIO {}{} set to {}:", port, pin, value);
                        println!("{}", response);
                    }
                }
                GpioCommands::Config { port, pin, mode } => {
                    let response = controller.control_gpio_config(&port, pin, &mode).await?;
                    if !cli.quiet {
                        println!("📌 GPIO {}{} configured to {}:", port, pin, mode);
                        println!("{}", response);
                    }
                }
            }
        }
        Commands::System(system_cmd) => {
            use cli::{EraseCommands, SystemCommands};
            match system_cmd {
                SystemCommands::Info => {
                    let response = controller.get_system_info_detailed().await?;
                    output_response(cli, "system info", &response, "🖥️", "System Information")?;
                }
                SystemCommands::Reboot { cold } => {
                    let cmd = if cold { "system reset cold" } else { "system reset" };
                    let response = controller.pm_command(cmd).await?;
                    output_response(cli, "system reboot", &response, "🔄", "System Reboot")?;
                }
                SystemCommands::Uptime => {
                    let response = controller.get_system_uptime().await?;
                    output_response(cli, "system uptime", &response, "⏱️", "System Uptime")?;
                }
                SystemCommands::DfuMode { timeout } => {
                    let response = controller.pm_command(&format!("system dfu-mode {}", timeout)).await?;
                    output_response(cli, "system dfu-mode", &response, "🔄", "DFU Mode")?;
                }
                SystemCommands::Erase(erase_cmd) => {
                    match erase_cmd {
                        EraseCommands::App => {
                            let response = controller.pm_command("system erase app").await?;
                            output_response(cli, "system erase app", &response, "🗑️", "Erase Application")?;
                        }
                        EraseCommands::Defaults => {
                            let response = controller.pm_command("system erase defaults").await?;
                            output_response(cli, "system erase defaults", &response, "🗑️", "Erase Defaults")?;
                        }
                    }
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
                    output_response(
                        cli,
                        "battery enable",
                        &response,
                        "✅",
                        "Battery Monitoring Enabled",
                    )?;
                }
                BatteryCommands::Disable => {
                    let response = controller.battery_disable().await?;
                    output_response(
                        cli,
                        "battery disable",
                        &response,
                        "❌",
                        "Battery Monitoring Disabled",
                    )?;
                }
            }
        }
        Commands::Pm(pm_cmd) => {
            use cli::{DefaultsCommands, DeviceAction, PowerManagementCommands, PowerState};
            match pm_cmd {
                PowerManagementCommands::Stats => {
                    let response = controller.pm_stats().await?;
                    if !cli.quiet {
                        println!("📊 Power Management Statistics:");
                        println!("{}", response);
                    }
                }
                PowerManagementCommands::Sleep {
                    time,
                    pmic,
                    wifi,
                    disp,
                    alloff,
                    vlls0,
                    vlls1,
                    vlls2,
                    vlls3,
                } => {
                    let mut cmd_parts = vec!["sleep".to_string()];
                    if let Some(t) = time {
                        cmd_parts.push(t);
                    }
                    if pmic {
                        cmd_parts.push("--pmic".to_string());
                    }
                    if wifi {
                        cmd_parts.push("--wifi".to_string());
                    }
                    if disp {
                        cmd_parts.push("--disp".to_string());
                    }
                    if alloff {
                        cmd_parts.push("--alloff".to_string());
                    }
                    if vlls0 {
                        cmd_parts.push("--vlls0".to_string());
                    }
                    if vlls1 {
                        cmd_parts.push("--vlls1".to_string());
                    }
                    if vlls2 {
                        cmd_parts.push("--vlls2".to_string());
                    }
                    if vlls3 {
                        cmd_parts.push("--vlls3".to_string());
                    }
                    let cmd = cmd_parts.join(" ");
                    let response = controller.pm_command(&cmd).await?;
                    if !cli.quiet {
                        println!("😴 Entering Low Power Mode:");
                        println!("{}", response);
                    }
                }
                PowerManagementCommands::Wake => {
                    let response = controller.pm_command("wake").await?;
                    if !cli.quiet {
                        println!("⏰ Last Wake Source:");
                        println!("{}", response);
                    }
                }
                PowerManagementCommands::Measure => {
                    let response = controller.pm_command("measure").await?;
                    if !cli.quiet {
                        println!("🔋 Battery Measurement:");
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
                PowerManagementCommands::All { state } => {
                    let state_str = match state {
                        PowerState::On => "on",
                        PowerState::Off => "off",
                        PowerState::Status => "status",
                    };
                    let response = controller.pm_command(&format!("all {}", state_str)).await?;
                    if !cli.quiet {
                        println!("⚡ All Power Rails:");
                        println!("{}", response);
                    }
                }
                PowerManagementCommands::Pmic { state } => {
                    let state_str = match state {
                        PowerState::On => "on",
                        PowerState::Off => "off",
                        PowerState::Status => "status",
                    };
                    let response = controller.pm_command(&format!("pmic {}", state_str)).await?;
                    if !cli.quiet {
                        println!("⚡ PMIC Control:");
                        println!("{}", response);
                    }
                }
                PowerManagementCommands::Wifi { state } => {
                    let state_str = match state {
                        PowerState::On => "on",
                        PowerState::Off => "off",
                        PowerState::Status => "status",
                    };
                    let response = controller.pm_command(&format!("wifi {}", state_str)).await?;
                    if !cli.quiet {
                        println!("📶 WiFi Control:");
                        println!("{}", response);
                    }
                }
                PowerManagementCommands::Disp { state } => {
                    let state_str = match state {
                        PowerState::On => "on",
                        PowerState::Off => "off",
                        PowerState::Status => "status",
                    };
                    let response = controller.pm_command(&format!("disp {}", state_str)).await?;
                    if !cli.quiet {
                        println!("🖥️ Display Control:");
                        println!("{}", response);
                    }
                }
                PowerManagementCommands::Defaults(defaults_cmd) => {
                    match defaults_cmd {
                        DefaultsCommands::Show => {
                            let response = controller.pm_command("defaults").await?;
                            if !cli.quiet {
                                println!("⚙️ Power Rail Defaults:");
                                println!("{}", response);
                            }
                        }
                        DefaultsCommands::Save => {
                            let response = controller.pm_command("defaults save").await?;
                            if !cli.quiet {
                                println!("💾 Saving Power Rail Defaults:");
                                println!("{}", response);
                            }
                        }
                        DefaultsCommands::Pmic { state } => {
                            let state_str = match state {
                                PowerState::On => "on",
                                PowerState::Off => "off",
                                PowerState::Status => "status",
                            };
                            let response = controller.pm_command(&format!("defaults pmic {}", state_str)).await?;
                            if !cli.quiet {
                                println!("⚙️ PMIC Default:");
                                println!("{}", response);
                            }
                        }
                        DefaultsCommands::Wifi { state } => {
                            let state_str = match state {
                                PowerState::On => "on",
                                PowerState::Off => "off",
                                PowerState::Status => "status",
                            };
                            let response = controller.pm_command(&format!("defaults wifi {}", state_str)).await?;
                            if !cli.quiet {
                                println!("⚙️ WiFi Default:");
                                println!("{}", response);
                            }
                        }
                        DefaultsCommands::Disp { state } => {
                            let state_str = match state {
                                PowerState::On => "on",
                                PowerState::Off => "off",
                                PowerState::Status => "status",
                            };
                            let response = controller.pm_command(&format!("defaults disp {}", state_str)).await?;
                            if !cli.quiet {
                                println!("⚙️ Display Default:");
                                println!("{}", response);
                            }
                        }
                    }
                }
                PowerManagementCommands::Ltc2959 { action } => {
                    let action_str = match action {
                        DeviceAction::Wake => "wake",
                        DeviceAction::Sleep => "sleep",
                    };
                    let response = controller.pm_command(&format!("ltc2959 {}", action_str)).await?;
                    if !cli.quiet {
                        println!("🔋 LTC2959 Control:");
                        println!("{}", response);
                    }
                }
                PowerManagementCommands::Nfc { action } => {
                    let action_str = match action {
                        DeviceAction::Wake => "wake",
                        DeviceAction::Sleep => "sleep",
                    };
                    let response = controller.pm_command(&format!("nfc {}", action_str)).await?;
                    if !cli.quiet {
                        println!("📡 NFC Control:");
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
                        PowerState::On => "imx93 on",
                        PowerState::Off => "imx93 off",
                        PowerState::Status => "imx93 status",
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
                NfcCommands::Scan => {
                    let response = controller.nfc_command("scan").await?;
                    if !cli.quiet {
                        println!("🔍 NFC I2C Scan:");
                        println!("{}", response);
                    }
                }
                NfcCommands::Status => {
                    let response = controller.nfc_command("status").await?;
                    if !cli.quiet {
                        println!("📡 NFC Status:");
                        println!("{}", response);
                    }
                }
                NfcCommands::Init => {
                    let response = controller.nfc_command("init").await?;
                    if !cli.quiet {
                        println!("🔧 NFC Initialization:");
                        println!("{}", response);
                    }
                }
                NfcCommands::Debug => {
                    let response = controller.nfc_command("debug").await?;
                    if !cli.quiet {
                        println!("🐛 NFC Debug:");
                        println!("{}", response);
                    }
                }
                NfcCommands::Rfdbg => {
                    let response = controller.nfc_command("rfdbg").await?;
                    if !cli.quiet {
                        println!("📡 NFC RF Diagnostic:");
                        println!("{}", response);
                    }
                }
                NfcCommands::Ed => {
                    let response = controller.nfc_command("ed").await?;
                    if !cli.quiet {
                        println!("📡 NFC Field Detection:");
                        println!("{}", response);
                    }
                }
                NfcCommands::Enable => {
                    let response = controller.nfc_command("enable").await?;
                    if !cli.quiet {
                        println!("✅ NFC RF Enabled:");
                        println!("{}", response);
                    }
                }
                NfcCommands::Disable => {
                    let response = controller.nfc_command("disable").await?;
                    if !cli.quiet {
                        println!("❌ NFC RF Disabled:");
                        println!("{}", response);
                    }
                }
                NfcCommands::Reset => {
                    let response = controller.nfc_command("reset").await?;
                    if !cli.quiet {
                        println!("🔄 NFC Reset:");
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
        Commands::Rtc(rtc_cmd) => {
            use cli::{ExternalRtcAction, RtcCommands};
            match rtc_cmd {
                RtcCommands::Status => {
                    let response = controller.rtc_status().await?;
                    output_response(cli, "rtc status", &response, "🕐", "RTC Status")?;
                }
                RtcCommands::Get => {
                    let response = controller.rtc_get().await?;
                    output_response(cli, "rtc get", &response, "🕐", "RTC Counter")?;
                }
                RtcCommands::Config { action } => {
                    let action_str = match action {
                        ExternalRtcAction::None => "none",
                        ExternalRtcAction::Wake => "wake",
                        ExternalRtcAction::Auto => "auto",
                    };
                    let response = controller.rtc_config(action_str).await?;
                    output_response(cli, "rtc config", &response, "⚙️", "RTC Configuration")?;
                }
                RtcCommands::Show => {
                    let response = controller.rtc_show_config().await?;
                    output_response(cli, "rtc show", &response, "📋", "RTC Configuration")?;
                }
            }
        }
        Commands::Comm(comm_cmd) => {
            use cli::{CommCommands, PowerState};
            match comm_cmd {
                CommCommands::BtWake { state } => {
                    let state_str = match state {
                        PowerState::On => "on",
                        PowerState::Off => "off",
                        PowerState::Status => "status",
                    };
                    let response = controller.control_comm("bt_wake", state_str).await?;
                    if !cli.quiet {
                        println!("📡 BT_WAKE_HOST:");
                        println!("{}", response);
                    }
                }
                CommCommands::WlWake { state } => {
                    let state_str = match state {
                        PowerState::On => "on",
                        PowerState::Off => "off",
                        PowerState::Status => "status",
                    };
                    let response = controller.control_comm("wl_wake", state_str).await?;
                    if !cli.quiet {
                        println!("📡 WL_WAKE_HOST:");
                        println!("{}", response);
                    }
                }
            }
        }
        Commands::Firmware(firmware_cmd) => {
            use cli::FirmwareCommands;

            // Extract port and baud from the command
            let (port, baud) = match firmware_cmd {
                FirmwareCommands::Upload { ref port, baud, .. } => {
                    (port.clone(), baud.unwrap_or(115200))
                }
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
                    output_response(
                        cli,
                        "firmware info",
                        &response,
                        "ℹ️",
                        "Firmware Information",
                    )?;
                }
                FirmwareCommands::Reset => {
                    let response = firmware_manager.reset_to_bootloader().await?;
                    output_response(cli, "firmware reset", &response, "🔄", "Bootloader Reset")?;
                }
                FirmwareCommands::Upload {
                    file, skip_reset, ..
                } => {
                    let response = firmware_manager
                        .upload_firmware(file.as_path(), skip_reset)
                        .await?;
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
