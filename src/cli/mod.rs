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
    #[arg(
        short,
        long,
        default_value = "/dev/ttyLP2",
        help = "Serial device path"
    )]
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

    /// RTC (Real-Time Clock) commands
    #[command(subcommand)]
    Rtc(RtcCommands),

    /// Firmware management commands
    #[command(subcommand)]
    Firmware(FirmwareCommands),

    /// Communication control commands
    #[command(subcommand)]
    Comm(CommCommands),

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
    Reboot {
        /// Cold reset (default: warm reset)
        #[arg(long)]
        cold: bool,
    },
    /// Get system uptime
    Uptime,
    /// Request bootloader DFU mode
    DfuMode {
        /// Timeout in seconds (0-255, default: 20, 0=infinite)
        #[arg(default_value = "20")]
        timeout: u8,
    },
    /// Erase operations
    #[command(subcommand)]
    Erase(EraseCommands),
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
    /// Configure GPIO pin
    Config {
        /// GPIO port (e.g., gpioa, gpiob)
        port: String,
        /// GPIO pin number
        pin: u8,
        /// GPIO mode (input, output, etc.)
        mode: String,
    },
}

/// NFC interface commands
#[derive(Subcommand, Debug, Clone)]
pub enum NfcCommands {
    /// Scan I2C bus for NTA5332 NFC chip
    Scan,
    /// Get NFC status
    Status,
    /// Initialize NTA5332 chip
    Init,
    /// Comprehensive NFC debug information
    Debug,
    /// RF interface diagnostic
    Rfdbg,
    /// Show NFC field detection status
    Ed,
    /// Enable NFC RF interface
    Enable,
    /// Disable NFC RF interface
    Disable,
    /// System reset NTA5332
    Reset,
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
    /// Enter low power mode
    Sleep {
        /// Sleep duration (e.g., 30s, 5m, 2h, 1d, or combinations like 1d12h30m)
        #[arg(short, long)]
        time: Option<String>,
        /// Turn off PMIC before sleep
        #[arg(long)]
        pmic: bool,
        /// Turn off WiFi before sleep
        #[arg(long)]
        wifi: bool,
        /// Turn off display before sleep
        #[arg(long)]
        disp: bool,
        /// Turn off all peripherals before sleep
        #[arg(long)]
        alloff: bool,
        /// VLLS0 mode (~150 nA, external wake only)
        #[arg(long)]
        vlls0: bool,
        /// VLLS1 mode (~200 nA, internal RTC wake enabled)
        #[arg(long)]
        vlls1: bool,
        /// VLLS2 mode (~350 nA, more RAM retention)
        #[arg(long)]
        vlls2: bool,
        /// VLLS3 mode (~412 nA, full RAM, most wake sources)
        #[arg(long)]
        vlls3: bool,
    },
    /// Show last LLS wake source
    Wake,
    /// Battery voltage and current measurement (one-time)
    Measure,
    /// Start/stop monitoring
    Monitor {
        /// Action to perform
        #[arg(value_enum)]
        action: MonitorAction,
        /// Monitoring interval in seconds
        interval: Option<u64>,
    },
    /// Control all power rails
    All {
        /// Power state
        #[arg(value_enum)]
        state: PowerState,
    },
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
    /// Power rail defaults for bootloader
    #[command(subcommand)]
    Defaults(DefaultsCommands),
    /// LTC2959 ADC control
    Ltc2959 {
        /// Action (wake or sleep)
        #[arg(value_enum)]
        action: DeviceAction,
    },
    /// NTA5332 NFC control
    Nfc {
        /// Action (wake or sleep)
        #[arg(value_enum)]
        action: DeviceAction,
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
    /// Shutdown the E-Ink controller board (permanent power off)
    Shutdown,
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
    /// Set accumulated charge (mAh)
    SetCharge {
        /// Charge value in mAh
        charge: u32,
    },
    /// Trigger charge complete
    ChargeComplete,
    /// Control CC_GPIO pin
    CcGpio {
        /// GPIO state
        #[arg(value_enum)]
        state: PowerState,
    },
    /// Reset for fresh battery installation
    ProductionReset,
    /// Set ADC mode (0-6)
    AdcMode {
        /// ADC mode value (0-6)
        mode: u8,
    },
    /// Read register (hex)
    RegRead {
        /// Register address in hex
        address: String,
    },
    /// Write register (hex)
    RegWrite {
        /// Register address in hex
        address: String,
        /// Value to write in hex
        value: String,
    },
}

/// Firmware management commands
#[derive(Subcommand, Debug, Clone)]
pub enum FirmwareCommands {
    /// List installed firmware images
    List,
    /// Upload new firmware image
    Upload {
        /// Firmware file path
        #[arg(short, long)]
        file: std::path::PathBuf,
        /// Skip system reset (assume already in bootloader mode)
        #[arg(long)]
        skip_reset: bool,
        /// Custom serial port (default: /dev/ttyLP2)
        #[arg(long)]
        port: Option<String>,
        /// Custom baud rate (default: 115200)
        #[arg(long)]
        baud: Option<u32>,
    },
    /// Reset PMU into bootloader mode
    Reset,
    /// Get firmware slot information
    Info,
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

/// RTC (Real-Time Clock) commands
#[derive(Subcommand, Debug, Clone)]
pub enum RtcCommands {
    /// Show RTC status (internal + external PCF2131)
    Status,
    /// Get internal RTC counter value (uptime)
    Get,
    /// Configure external RTC interrupt action
    Config {
        /// External RTC interrupt action
        #[arg(value_enum)]
        action: ExternalRtcAction,
    },
    /// Show external RTC interrupt configuration
    Show,
}

/// External RTC interrupt actions
#[derive(ValueEnum, Clone, Debug)]
pub enum ExternalRtcAction {
    /// No action - just log the event
    None,
    /// Always wake i.MX93 when interrupt occurs
    Wake,
    /// Auto-power i.MX93 if PMIC is currently off
    Auto,
}

/// Erase commands
#[derive(Subcommand, Debug, Clone)]
pub enum EraseCommands {
    /// Erase both application partitions (slot0 and slot1)
    App,
    /// Erase configuration defaults from storage
    Defaults,
}

/// Power rail defaults commands
#[derive(Subcommand, Debug, Clone)]
pub enum DefaultsCommands {
    /// Show current defaults stored in flash
    Show,
    /// Save current power rail states as defaults
    Save,
    /// Set PMIC_EN default state
    Pmic {
        /// Power state
        #[arg(value_enum)]
        state: PowerState,
    },
    /// Set WiFi_EN default state
    Wifi {
        /// Power state
        #[arg(value_enum)]
        state: PowerState,
    },
    /// Set DISP_EN default state
    Disp {
        /// Power state
        #[arg(value_enum)]
        state: PowerState,
    },
}

/// Device actions (wake/sleep)
#[derive(ValueEnum, Clone, Debug)]
pub enum DeviceAction {
    /// Wake device
    Wake,
    /// Sleep device
    Sleep,
}

/// Communication control commands
#[derive(Subcommand, Debug, Clone)]
pub enum CommCommands {
    /// Control BT_WAKE_HOST signal (PTC1)
    BtWake {
        /// Power state
        #[arg(value_enum)]
        state: PowerState,
    },
    /// Control WL_WAKE_HOST signal (PTC3)
    WlWake {
        /// Power state
        #[arg(value_enum)]
        state: PowerState,
    },
}
