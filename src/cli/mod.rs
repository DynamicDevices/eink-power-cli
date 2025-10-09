/*
 * E-ink Power CLI - Command Line Interface
 * Copyright (c) 2025 Dynamic Devices Ltd
 * All rights reserved.
 */

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// E-ink Power CLI - Command-line interface for power management controller
#[derive(Parser, Debug)]
#[command(
    name = "eink-power-cli",
    version,
    about = "Command-line interface for E-ink power management controller",
    long_about = "A Rust-based CLI tool for communicating with the MCXC143VFM power management controller over serial UART. Provides power control, battery monitoring, and system management capabilities."
)]
pub struct Cli {
    /// Serial device path
    #[arg(short, long, default_value = "/dev/ttyLP2", help = "Serial device path")]
    pub device: String,

    /// Baud rate for serial communication
    #[arg(short, long, default_value = "115200", help = "Serial baud rate")]
    pub baud: u32,

    /// Command timeout in seconds
    #[arg(short, long, default_value = "3", help = "Command timeout in seconds")]
    pub timeout: u64,

    /// Output format
    #[arg(short, long, default_value = "human", help = "Output format")]
    pub format: OutputFormat,

    /// Configuration file path
    #[arg(short, long, help = "Configuration file path")]
    pub config: Option<PathBuf>,

    /// Enable verbose logging
    #[arg(short, long, help = "Enable verbose logging")]
    pub verbose: bool,

    /// Suppress non-error output
    #[arg(short, long, help = "Suppress non-error output")]
    pub quiet: bool,

    /// Command to execute
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available output formats
#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    /// Human-readable format with colors and emojis
    Human,
    /// JSON format for machine processing
    Json,
    /// CSV format for data analysis
    Csv,
}

/// Available commands
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// System commands
    #[command(subcommand)]
    System(SystemCommands),

    /// Power management commands
    #[command(subcommand)]
    Power(PowerCommands),

    /// Battery monitoring commands
    #[command(subcommand)]
    Battery(BatteryCommands),

    /// GPIO control commands
    #[command(subcommand)]
    Gpio(GpioCommands),

    /// NFC interface commands
    #[command(subcommand)]
    Nfc(NfcCommands),

    /// Board control commands
    #[command(subcommand)]
    Board(BoardCommands),

    /// LTC2959 coulomb counter commands
    #[command(subcommand)]
    Ltc2959(Ltc2959Commands),

    /// Power management commands
    #[command(subcommand)]
    Pm(PowerManagementCommands),

    /// Connectivity test
    Ping,

    /// Get controller version
    Version,

    /// Monitor continuously
    Monitor {
        /// Monitoring interval in seconds
        #[arg(short, long, default_value = "30")]
        interval: u64,

        /// Run continuously
        #[arg(short, long)]
        continuous: bool,
    },

    /// Execute batch commands from file
    Batch {
        /// File containing commands to execute
        #[arg(short, long)]
        file: PathBuf,
    },
}

/// System-level commands
#[derive(Subcommand, Debug, Clone)]
pub enum SystemCommands {
    /// Get system information
    Info,
    /// Reboot the controller
    Reboot,
    /// Get system uptime
    Uptime,
}

/// Power control commands
#[derive(Subcommand, Debug, Clone)]
pub enum PowerCommands {
    /// Control PMIC power
    Pmic {
        /// Power state
        #[arg(value_enum)]
        state: PowerState,
    },
    /// Control WiFi power
    Wifi {
        /// Power state
        #[arg(value_enum)]
        state: PowerState,
    },
    /// Control display power
    Disp {
        /// Power state
        #[arg(value_enum)]
        state: PowerState,
    },
    /// Show power statistics
    Stats,
    /// Show battery coulomb counter readings
    Coulomb,
}

/// Battery monitoring commands
#[derive(Subcommand, Debug, Clone)]
pub enum BatteryCommands {
    /// Read battery measurements
    Read,
    /// Get battery status
    Status,
    /// Enable battery monitoring
    Enable,
    /// Disable battery monitoring
    Disable,
}

/// GPIO control commands
#[derive(Subcommand, Debug, Clone)]
pub enum GpioCommands {
    /// Read GPIO state
    Get {
        /// GPIO port (e.g., gpioa, gpiob)
        port: String,
        /// GPIO pin number
        pin: u8,
    },
    /// Set GPIO state
    Set {
        /// GPIO port (e.g., gpioa, gpiob)
        port: String,
        /// GPIO pin number
        pin: u8,
        /// Value to set (0 or 1)
        value: u8,
    },
}

/// NFC interface commands
#[derive(Subcommand, Debug, Clone)]
pub enum NfcCommands {
    /// Get NFC status
    Status,
    /// Get NFC device information
    Info,
    /// Check field detection
    FieldDetect,
}

/// Power management commands
#[derive(Subcommand, Debug, Clone)]
pub enum PowerManagementCommands {
    /// Show power management statistics
    Stats,
    /// Enter deep sleep
    Sleep {
        /// Sleep timeout in milliseconds
        timeout: Option<u32>,
    },
    /// Start/stop monitoring
    Monitor {
        /// Action to perform
        #[arg(value_enum)]
        action: MonitorAction,
        /// Monitoring interval in seconds
        interval: Option<u64>,
    },
    /// Perform battery health check
    BatteryCheck,
    /// Control i.MX93 power
    Imx93 {
        /// Power state
        #[arg(value_enum)]
        state: PowerState,
    },
}

/// Board control commands
#[derive(Subcommand, Debug, Clone)]
pub enum BoardCommands {
    /// Reset the E-Ink controller board (power cycle)
    Reset,
}

/// LTC2959 coulomb counter commands
#[derive(Subcommand, Debug, Clone)]
pub enum Ltc2959Commands {
    /// Initialize LTC2959 coulomb counter
    Init,
    /// Read voltage, current, charge, power
    Read,
    /// Show device status and alert flags
    Status,
    /// Enable ADC measurements (smart sleep)
    Enable,
    /// Disable ADC (ultra-low power mode)
    Disable,
    /// Scan I2C bus for LTC2959 device
    Scan,
    /// Control CC_GPIO pin
    CcGpio {
        /// GPIO state
        #[arg(value_enum)]
        state: PowerState,
    },
    /// Reset for fresh battery installation
    ProductionReset,
}

/// Power states
#[derive(ValueEnum, Clone, Debug)]
pub enum PowerState {
    On,
    Off,
    Status,
}

/// Monitor actions
#[derive(ValueEnum, Clone, Debug)]
pub enum MonitorAction {
    Start,
    Stop,
}
