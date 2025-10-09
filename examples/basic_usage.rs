/*
 * E-ink Power CLI - Basic Usage Example
 * Copyright (c) 2025 Dynamic Devices Ltd
 * All rights reserved.
 */

//! Basic usage example for the E-ink Power CLI library

use eink_power_cli::{BatteryMonitor, Connection};
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    info!("E-ink Power CLI - Basic Usage Example");

    // Create connection to power controller
    let mut connection = Connection::new("/dev/ttyUSB0", 115200, false)?;
    connection.set_timeout(5);

    // Connect to the device
    connection.connect().await?;

    // Create battery monitor
    let mut battery = BatteryMonitor::new(connection);

    // Read battery status
    let status = battery.read_status().await?;
    println!("{}", status.format_human());

    // Check if battery is charging
    if status.is_charging() {
        println!("🔌 Battery is charging");
    } else {
        println!("🔋 Battery is discharging");
    }

    // Check for low voltage
    if status.is_low_voltage(3200) {
        println!("⚠️  Low battery warning!");
    }

    Ok(())
}
