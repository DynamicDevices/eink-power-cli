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
        /// Cold reset (full power cycle)
        #[arg(long)]
        cold: bool,
    },
    /// Get system uptime
    Uptime,
    /// Request bootloader DFU mode
    DfuMode {
        /// Timeout in seconds (optional)
        timeout: Option<u32>,
    },
    /// Erase operations
    Erase {
        /// Erase target: app (both slots), defaults (configuration)
        #[arg(value_enum)]
        target: EraseTarget,
    },
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
    /// Control all power rails
    All {
        /// Power state
        #[arg(value_enum)]
        state: PowerState,
    },
    /// Power rail defaults for bootloader
    Defaults {
        /// Action: save, or set <rail> <on|off>
        action: Option<String>,
        /// Rail name (if setting)
        rail: Option<String>,
        /// Rail state (if setting)
        rail_state: Option<String>,
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
        /// GPIO port (e.g., a, b, c, d, e)
        port: String,
        /// GPIO pin number
        pin: u8,
    },
    /// Set GPIO state
    Set {
        /// GPIO port (e.g., a, b, c, d, e)
        port: String,
        /// GPIO pin number
        pin: u8,
        /// Value to set (0 or 1)
        value: u8,
    },
    /// Configure GPIO mode
    Config {
        /// GPIO port (e.g., a, b, c, d, e)
        port: String,
        /// GPIO pin number
        pin: u8,
        /// Mode: input, output, input_pullup, input_pulldown
        mode: String,
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
    /// Scan I2C bus for NTA5332 NFC chip
    Scan,
    /// Initialize NTA5332 chip (use if chip not responding)
    Init,
    /// Comprehensive NFC debug information
    Debug,
    /// RF interface diagnostic (CRITICAL for RF enable issues)
    Rfdbg,
    /// Show NFC field detection status
    Ed,
    /// Enable NFC RF interface for field detection
    Enable,
    /// Disable NFC RF interface
    Disable,
    /// System reset NTA5332 (activates SRAM after CONFIG changes)
    Reset,
}

/// Power management commands
#[derive(Subcommand, Debug, Clone)]
pub enum PowerManagementCommands {
    /// Show power management statistics
    Stats,
    /// Enter low power mode
    Sleep {
        /// Sleep time (supports units: d/h/m/s, e.g., "1d2h30m" or "10s")
        time: Option<String>,
        /// Turn off PMIC during sleep
        #[arg(long)]
        pmic: bool,
        /// Turn off WiFi during sleep
        #[arg(long)]
        wifi: bool,
        /// Turn off display during sleep
        #[arg(long)]
        disp: bool,
        /// Turn off all power rails during sleep
        #[arg(long)]
        alloff: bool,
        /// Use VLLS0 mode
        #[arg(long)]
        vlls0: bool,
        /// Use VLLS1 mode
        #[arg(long)]
        vlls1: bool,
        /// Use VLLS2 mode
        #[arg(long)]
        vlls2: bool,
        /// Use VLLS3 mode
        #[arg(long)]
        vlls3: bool,
    },
    /// Show last LLS wake source (debug for --alloff)
    Wake,
    /// Battery voltage and current measurement
    Measure,
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
    /// LTC2959 ADC control
    Ltc2959 {
        /// Action: wake or sleep
        #[arg(value_enum)]
        action: Ltc2959AdcAction,
    },
    /// NTA5332 NFC control
    Nfc {
        /// Action: wake or sleep
        #[arg(value_enum)]
        action: NfcAdcAction,
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
    /// Control CC_GPIO pin
    CcGpio {
        /// GPIO state
        #[arg(value_enum)]
        state: PowerState,
    },
    /// Reset for fresh battery installation
    ProductionReset,
    /// Set accumulated charge (mAh)
    SetCharge {
        /// Charge value in mAh
        charge_mah: u32,
    },
    /// Trigger charge complete
    ChargeComplete,
    /// Set ADC mode (0-6)
    AdcMode {
        /// ADC mode: 0=Sleep, 1=Smart Sleep, 2=Continuous V, 3=Continuous I, 4=Continuous V+I, 5=Single Shot, 6=Continuous V+I+T
        mode: u8,
    },
    /// Read register (hex address)
    RegRead {
        /// Register address in hex (e.g., 0x00)
        address: String,
    },
    /// Write register (hex address and value)
    RegWrite {
        /// Register address in hex (e.g., 0x00)
        address: String,
        /// Register value in hex (e.g., 0xFF)
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

/// Erase targets
#[derive(ValueEnum, Clone, Debug)]
pub enum EraseTarget {
    /// Erase both application partitions (slot0 and slot1)
    App,
    /// Erase configuration defaults from storage
    Defaults,
}

/// LTC2959 ADC actions
#[derive(ValueEnum, Clone, Debug)]
pub enum Ltc2959AdcAction {
    /// Wake LTC2959 ADC
    Wake,
    /// Sleep LTC2959 ADC
    Sleep,
}

/// NFC ADC actions
#[derive(ValueEnum, Clone, Debug)]
pub enum NfcAdcAction {
    /// Wake NFC
    Wake,
    /// Sleep NFC
    Sleep,
}

/// Communication control commands
#[derive(Subcommand, Debug, Clone)]
pub enum CommCommands {
    /// Control BT_WAKE_HOST signal
    BtWake {
        /// Signal state
        #[arg(value_enum)]
        state: PowerState,
    },
    /// Control WL_WAKE_HOST signal
    WlWake {
        /// Signal state
        #[arg(value_enum)]
        state: PowerState,
    },
}
