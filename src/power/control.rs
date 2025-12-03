/*
 * E-ink Power CLI - Power Control
 * Copyright (c) 2025 Dynamic Devices Ltd
 * All rights reserved.
 */

use crate::cli;
use crate::error::Result;
use crate::serial::{Connection, Protocol};
use log::{debug, info};
use serde::{Deserialize, Serialize};

/// Power controller interface
pub struct PowerController {
    pub protocol: Protocol,
}

impl PowerController {
    /// Create a new power controller instance
    pub fn new(connection: Connection) -> Self {
        Self {
            protocol: Protocol::new(connection),
        }
    }

    /// Control PMIC power
    pub async fn control_pmic(&mut self, state: PowerState) -> Result<String> {
        info!("Controlling PMIC power: {:?}", state);

        let state_str = match state {
            PowerState::On => "on",
            PowerState::Off => "off",
            PowerState::Status => "status",
        };

        self.protocol.execute_power_command("pmic", state_str).await
    }

    /// Control WiFi power
    pub async fn control_wifi(&mut self, state: PowerState) -> Result<String> {
        info!("Controlling WiFi power: {:?}", state);

        let state_str = match state {
            PowerState::On => "on",
            PowerState::Off => "off",
            PowerState::Status => "status",
        };

        self.protocol.execute_power_command("wifi", state_str).await
    }

    /// Control display power
    pub async fn control_display(&mut self, state: PowerState) -> Result<String> {
        info!("Controlling display power: {:?}", state);

        let state_str = match state {
            PowerState::On => "on",
            PowerState::Off => "off",
            PowerState::Status => "status",
        };

        self.protocol.execute_power_command("disp", state_str).await
    }

    /// Get power statistics
    pub async fn get_power_stats(&mut self) -> Result<PowerStats> {
        info!("Getting power statistics");

        let response = self.protocol.execute_system_command("power stats").await?;
        self.parse_power_stats(&response)
    }

    /// Enter deep sleep with all peripherals OFF (PMIC, WiFi, Display)
    pub async fn deep_sleep_all_off(&mut self, timeout_ms: Option<u32>) -> Result<String> {
        info!("Entering deep sleep with all peripherals OFF (timeout: {:?} ms)", timeout_ms);

        let command = if let Some(timeout) = timeout_ms {
            format!("pm deep_sleep_all_off {}", timeout)
        } else {
            "pm deep_sleep_all_off".to_string()
        };

        self.protocol.execute_system_command(&command).await
    }

    /// Control GPIO pin
    pub async fn control_gpio(
        &mut self,
        port: &str,
        pin: u8,
        action: GpioAction,
    ) -> Result<String> {
        info!("Controlling GPIO {}{}: {:?}", port, pin, action);

        match action {
            GpioAction::Get => {
                self.protocol
                    .execute_gpio_command("get", port, pin, None)
                    .await
            }
            GpioAction::Set(value) => {
                self.protocol
                    .execute_gpio_command("set", port, pin, Some(value))
                    .await
            }
        }
    }

    /// Parse power statistics response
    fn parse_power_stats(&self, response: &str) -> Result<PowerStats> {
        debug!("Parsing power stats: {}", response);

        // TODO: Implement actual parsing based on controller response format
        // This is a placeholder implementation
        Ok(PowerStats {
            active_time_ms: 123456,
            sleep_count: 42,
            wake_count: 38,
            rtc_wake_count: 15,
            nfc_wake_count: 12,
            uart_wake_count: 11,
            timestamp: chrono::Utc::now(),
        })
    }
}

/// Power states
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PowerState {
    On,
    Off,
    Status,
}

impl From<cli::PowerState> for PowerState {
    fn from(state: cli::PowerState) -> Self {
        match state {
            cli::PowerState::On => PowerState::On,
            cli::PowerState::Off => PowerState::Off,
            cli::PowerState::Status => PowerState::Status,
        }
    }
}

/// GPIO actions
#[derive(Debug, Clone)]
pub enum GpioAction {
    Get,
    Set(u8),
}

/// Power management statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerStats {
    /// Active time in milliseconds
    pub active_time_ms: u32,
    /// Number of sleep cycles
    pub sleep_count: u32,
    /// Number of wake events
    pub wake_count: u32,
    /// RTC wake events
    pub rtc_wake_count: u32,
    /// NFC wake events
    pub nfc_wake_count: u32,
    /// UART wake events
    pub uart_wake_count: u32,
    /// Timestamp of measurement
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl PowerStats {
    /// Format for human-readable display
    pub fn format_human(&self) -> String {
        format!(
            "⚡ Power Management Statistics:\n   Active time: {} ms\n   Sleep count: {}\n   Wake events: {}\n   RTC wake events: {}\n   NFC wake events: {}\n   UART wake events: {}",
            self.active_time_ms,
            self.sleep_count,
            self.wake_count,
            self.rtc_wake_count,
            self.nfc_wake_count,
            self.uart_wake_count
        )
    }
}
