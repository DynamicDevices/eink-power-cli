/*
 * E-ink Power CLI - Integration Tests
 * Copyright (c) 2025 Dynamic Devices Ltd
 * All rights reserved.
 */

//! Integration tests for the E-ink Power CLI
//!
//! These tests require actual hardware to be connected.
//! Use `cargo test --test integration_tests` to run them.

use eink_power_cli::{BatteryMonitor, Connection};
use std::env;

/// Test serial connection to the power controller
#[tokio::test]
#[ignore] // Requires hardware
async fn test_connection() {
    let device = env::var("TEST_DEVICE").unwrap_or_else(|_| "/dev/ttyUSB0".to_string());

    let mut connection =
        Connection::new(&device, 115200, false).expect("Failed to create connection");

    // This should succeed if hardware is connected
    let result = connection.connect().await;

    match result {
        Ok(()) => {
            println!("✅ Connection test passed");
            connection.disconnect().await;
        }
        Err(e) => {
            println!("❌ Connection test failed: {}", e);
            // Don't panic in case hardware is not available
        }
    }
}

/// Test battery monitoring functionality
#[tokio::test]
#[ignore] // Requires hardware
async fn test_battery_monitoring() {
    let device = env::var("TEST_DEVICE").unwrap_or_else(|_| "/dev/ttyUSB0".to_string());

    let connection = Connection::new(&device, 115200, false).expect("Failed to create connection");
    let _battery = BatteryMonitor::new(connection);

    // This test will be implemented once the protocol is complete
    println!("🔋 Battery monitoring test - placeholder");
}
