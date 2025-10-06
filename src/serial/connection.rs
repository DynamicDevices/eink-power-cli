/*
 * E-ink Power CLI - Serial Connection Management
 * Copyright (c) 2025 Dynamic Devices Ltd
 * All rights reserved.
 */

use crate::error::{PowerCliError, Result};
use log::{debug, info, warn};
use std::time::Duration;
use tokio::time::timeout;
use tokio_serial::{SerialPortBuilderExt, SerialStream};

/// Serial connection to the power management controller
pub struct Connection {
    device_path: String,
    baud_rate: u32,
    timeout_duration: Duration,
    stream: Option<SerialStream>,
}

impl Connection {
    /// Create a new connection instance
    pub fn new(device_path: &str, baud_rate: u32) -> Result<Self> {
        Ok(Self {
            device_path: device_path.to_string(),
            baud_rate,
            timeout_duration: Duration::from_secs(3),
            stream: None,
        })
    }

    /// Set command timeout
    pub fn set_timeout(&mut self, timeout_secs: u64) {
        self.timeout_duration = Duration::from_secs(timeout_secs);
    }

    /// Connect to the serial device
    pub async fn connect(&mut self) -> Result<()> {
        info!(
            "Connecting to {} at {} baud",
            self.device_path, self.baud_rate
        );

        // Check if device exists
        if !std::path::Path::new(&self.device_path).exists() {
            return Err(PowerCliError::DeviceNotFound {
                device: self.device_path.clone(),
            });
        }

        // Open serial port
        let stream = tokio_serial::new(&self.device_path, self.baud_rate)
            .data_bits(tokio_serial::DataBits::Eight)
            .parity(tokio_serial::Parity::None)
            .stop_bits(tokio_serial::StopBits::One)
            .flow_control(tokio_serial::FlowControl::None)
            .open_native_async()?;

        self.stream = Some(stream);
        info!("Successfully connected to {}", self.device_path);

        // Test connection with a ping
        self.test_connection().await?;

        Ok(())
    }

    /// Test the connection by sending a ping command
    async fn test_connection(&mut self) -> Result<()> {
        debug!("Testing connection with ping command");

        // TODO: Implement actual ping command
        // For now, just verify the stream is available
        if self.stream.is_none() {
            return Err(PowerCliError::NotConnected);
        }

        Ok(())
    }

    /// Send a command and wait for response
    pub async fn send_command(&mut self, command: &str) -> Result<String> {
        if self.stream.is_none() {
            return Err(PowerCliError::NotConnected);
        }

        debug!("Sending command: {}", command);

        // TODO: Implement actual command sending
        // This is a placeholder implementation
        let response = timeout(self.timeout_duration, async {
            // Simulate command execution
            tokio::time::sleep(Duration::from_millis(100)).await;
            Ok::<String, PowerCliError>(format!("Response to: {}", command))
        })
        .await
        .map_err(|_| PowerCliError::Timeout {
            timeout: self.timeout_duration.as_secs(),
        })??;

        debug!("Received response: {}", response);
        Ok(response)
    }

    /// Check if connection is active
    pub fn is_connected(&self) -> bool {
        self.stream.is_some()
    }

    /// Disconnect from the serial device
    pub async fn disconnect(&mut self) {
        if let Some(_stream) = self.stream.take() {
            info!("Disconnected from {}", self.device_path);
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        if self.stream.is_some() {
            warn!("Connection dropped without explicit disconnection");
        }
    }
}
