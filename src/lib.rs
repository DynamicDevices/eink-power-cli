/*
 * E-ink Power CLI - Library Interface
 * Copyright (c) 2025 Dynamic Devices Ltd
 * All rights reserved.
 */

//! # E-ink Power CLI
//!
//! Command-line interface for communicating with the MCXC143VFM E-ink power management controller.
//!
//! This library provides a Rust interface for:
//! - Serial communication with the power controller
//! - Power management and battery monitoring
//! - System control and GPIO manipulation
//! - NFC interface control
//!
//! ## Example
//!
//! ```rust,no_run
//! use eink_power_cli::serial::Connection;
//! use eink_power_cli::power::BatteryMonitor;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut conn = Connection::new("/dev/ttyUSB0", 115200)?;
//!     conn.connect().await?;
//!     
//!     let mut battery = BatteryMonitor::new(conn);
//!     let status = battery.read_status().await?;
//!     
//!     println!("Battery voltage: {}mV", status.voltage_mv);
//!     Ok(())
//! }
//! ```

pub mod cli;
pub mod error;
pub mod json;
pub mod power;
pub mod serial;

// Re-export commonly used types
pub use error::PowerCliError;
pub use power::BatteryMonitor;
pub use serial::Connection;
