/*
 * E-ink Power CLI - Communication Protocol
 * Copyright (c) 2025 Dynamic Devices Ltd
 * All rights reserved.
 */

use crate::error::{PowerCliError, Result};
use crate::serial::Connection;
use log::debug;
use serde_json::Value;

/// Protocol handler for communicating with the power management controller
pub struct Protocol {
    connection: Connection,
}

impl Protocol {
    /// Create a new protocol instance
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    /// Execute a system command
    pub async fn execute_system_command(&mut self, command: &str) -> Result<String> {
        debug!("Executing system command: {}", command);

        let response = self.connection.send_command(command).await?;
        self.parse_response(&response)
    }

    /// Execute a power control command
    pub async fn execute_power_command(&mut self, rail: &str, state: &str) -> Result<String> {
        let command = format!("power {} {}", rail, state);
        debug!("Executing power command: {}", command);

        let response = self.connection.send_command(&command).await?;
        self.parse_response(&response)
    }

    /// Execute a battery monitoring command
    #[allow(dead_code)]  // Future use
    pub async fn execute_battery_command(&mut self, command: &str) -> Result<String> {
        let full_command = if command == "read" {
            "ltc2959 read".to_string()
        } else {
            format!("ltc2959 {}", command)
        };

        debug!("Executing battery command: {}", full_command);

        let response = self.connection.send_command(&full_command).await?;
        self.parse_response(&response)
    }

    /// Execute a GPIO command
    pub async fn execute_gpio_command(
        &mut self,
        action: &str,
        port: &str,
        pin: u8,
        value: Option<u8>,
    ) -> Result<String> {
        let command = match action {
            "get" => format!("gpio get {} {}", port, pin),
            "set" => {
                let val = value.ok_or_else(|| PowerCliError::InvalidCommand {
                    command: "GPIO set requires a value".to_string(),
                })?;
                format!("gpio set {} {} {}", port, pin, val)
            }
            _ => {
                return Err(PowerCliError::InvalidCommand {
                    command: format!("Unknown GPIO action: {}", action),
                })
            }
        };

        debug!("Executing GPIO command: {}", command);

        let response = self.connection.send_command(&command).await?;
        self.parse_response(&response)
    }

    /// Execute an NFC command
    pub async fn execute_nfc_command(&mut self, command: &str) -> Result<String> {
        let full_command = format!("nfc {}", command);
        debug!("Executing NFC command: {}", full_command);

        let response = self.connection.send_command(&full_command).await?;
        self.parse_response(&response)
    }

    /// Execute a board control command
    pub async fn execute_board_command(&mut self, command: &str) -> Result<String> {
        let full_command = format!("board {}", command);
        debug!("Executing board command: {}", full_command);

        // Special handling for reset command - it will cause connection loss
        if command == "reset" {
            return self.execute_board_reset_command(&full_command).await;
        }

        let response = self.connection.send_command(&full_command).await?;
        self.parse_response(&response)
    }

    /// Execute board reset command with special handling for connection loss
    async fn execute_board_reset_command(&mut self, command: &str) -> Result<String> {
        debug!("Executing board reset command with short timeout");
        
        // Send the command but don't wait for a full response since the board will reset
        let _response = self.connection.send_command_with_short_timeout(command).await?;
        
        // Return a success message regardless of response since reset will cut connection
        Ok("Board reset sequence initiated. Connection will be lost during power cycle.".to_string())
    }

    /// Execute an LTC2959 coulomb counter command
    pub async fn execute_ltc2959_command(&mut self, command: &str) -> Result<String> {
        let full_command = format!("ltc2959 {}", command);
        debug!("Executing LTC2959 command: {}", full_command);

        let response = self.connection.send_command(&full_command).await?;
        self.parse_response(&response)
    }

    /// Parse the response from the controller
    fn parse_response(&self, response: &str) -> Result<String> {
        debug!("Parsing response: {}", response);

        // Check for error responses
        if response.contains("Error:") || response.contains("Failed:") {
            return Err(PowerCliError::ControllerError {
                message: response.to_string(),
            });
        }

        // TODO: Implement more sophisticated response parsing
        // For now, return the raw response
        Ok(response.to_string())
    }

    /// Parse battery data from response
    #[allow(dead_code)]  // Future use
    pub fn parse_battery_data(&self, response: &str) -> Result<BatteryData> {
        debug!("Parsing battery data from: {}", response);

        // TODO: Implement actual parsing based on controller response format
        // This is a placeholder implementation
        Ok(BatteryData {
            voltage_mv: 3850,
            current_ma: 125,
            charge_mah: 2450,
            temperature_c: 23,
        })
    }

    /// Format response as JSON
    #[allow(dead_code)]  // Future use
    pub fn format_as_json(&self, data: &str) -> Result<Value> {
        // TODO: Implement JSON formatting
        // For now, create a simple JSON structure
        let json = serde_json::json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "status": "success",
            "data": data
        });

        Ok(json)
    }

    /// Execute a power management command
    pub async fn execute_pm_command(&mut self, command: &str) -> Result<String> {
        let full_command = format!("pm {}", command);
        debug!("Executing PM command: {}", full_command);

        let response = self.connection.send_command(&full_command).await?;
        self.parse_response(&response)
    }
}

/// Battery monitoring data structure
#[derive(Debug, Clone)]
#[allow(dead_code)]  // Future use
pub struct BatteryData {
    pub voltage_mv: u16,
    pub current_ma: i16,
    pub charge_mah: u32,
    pub temperature_c: i16,
}
