/*
 * E-ink Power CLI - Serial Connection Management
 * Copyright (c) 2025 Dynamic Devices Ltd
 * All rights reserved.
 */

use crate::error::{PowerCliError, Result};
use log::{debug, info, warn};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
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

        // Just verify the stream is available
        if self.stream.is_none() {
            return Err(PowerCliError::NotConnected);
        }

        // Send a ping command and wait for response
        // This helps verify the firmware is responding
        let _response = self.send_command("ping").await?;
        Ok(())
    }

    /// Send a command and wait for response
    pub async fn send_command(&mut self, command: &str) -> Result<String> {
        let stream = self.stream.as_mut().ok_or(PowerCliError::NotConnected)?;
        let timeout_duration = self.timeout_duration;

        debug!("Sending command: {}", command);

        // Clear any pending input
        let _ = Self::read_available_static(stream, Duration::from_millis(100)).await;

        // Send command with newline
        let cmd_bytes = format!("{}\n", command);
        stream
            .write_all(cmd_bytes.as_bytes())
            .await
            .map_err(|e| PowerCliError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to write to serial port: {}", e),
            )))?;

        // Flush to ensure command is sent
        stream
            .flush()
            .await
            .map_err(|e| PowerCliError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to flush serial port: {}", e),
            )))?;

        // Read response with timeout
        let response = timeout(timeout_duration, async {
            Self::read_response_static(stream).await
        })
        .await
        .map_err(|_| PowerCliError::Timeout {
            timeout: timeout_duration.as_secs(),
        })??;

        debug!("Received response ({} bytes)", response.len());
        Ok(response)
    }

    /// Read available data from serial port (non-blocking)
    async fn read_available_static(stream: &mut SerialStream, max_duration: Duration) -> String {
        let mut buffer = vec![0u8; 4096];
        let mut result = String::new();
        let start = std::time::Instant::now();

        while start.elapsed() < max_duration {
            match tokio::time::timeout(Duration::from_millis(50), stream.read(&mut buffer)).await {
                Ok(Ok(0)) => break, // EOF
                Ok(Ok(n)) => {
                    if let Ok(text) = String::from_utf8(buffer[..n].to_vec()) {
                        result.push_str(&text);
                    }
                }
                Ok(Err(_)) => break, // Error reading
                Err(_) => break,     // Timeout - no more data
            }
        }

        result
    }

    /// Read response from serial port until prompt or timeout
    async fn read_response_static(stream: &mut SerialStream) -> Result<String> {
        let mut response = String::new();
        let mut buffer = vec![0u8; 1024];
        let mut last_activity = std::time::Instant::now();
        let max_idle_time = Duration::from_millis(500); // 500ms idle = end of response

        loop {
            match tokio::time::timeout(Duration::from_millis(100), stream.read(&mut buffer)).await {
                Ok(Ok(0)) => break, // EOF
                Ok(Ok(n)) => {
                    if let Ok(text) = String::from_utf8(buffer[..n].to_vec()) {
                        response.push_str(&text);
                        last_activity = std::time::Instant::now();
                        
                        // Check for shell prompt (indicates command completed)
                        if response.contains(":~$") || response.contains("uart:") {
                            // Small delay to ensure all output is captured
                            tokio::time::sleep(Duration::from_millis(100)).await;
                            // Read any remaining data
                            let remaining = Self::read_available_static(stream, Duration::from_millis(200)).await;
                            if !remaining.is_empty() {
                                response.push_str(&remaining);
                            }
                            break;
                        }
                    }
                }
                Ok(Err(e)) => {
                    return Err(PowerCliError::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Serial read error: {}", e),
                    )));
                }
                Err(_) => {
                    // Timeout - check if we've been idle too long
                    if last_activity.elapsed() > max_idle_time {
                        break;
                    }
                }
            }
        }

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
