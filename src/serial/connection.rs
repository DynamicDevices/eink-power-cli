/*
 * E-ink Power CLI - Serial Connection Management
 * Copyright (c) 2025 Dynamic Devices Ltd
 * All rights reserved.
 */

use crate::error::{PowerCliError, Result};
use log::{debug, info};
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
    quiet: bool,
}

impl Connection {
    /// Create a new connection instance
    pub fn new(device_path: &str, baud_rate: u32, quiet: bool) -> Result<Self> {
        Ok(Self {
            device_path: device_path.to_string(),
            baud_rate,
            timeout_duration: Duration::from_secs(3),
            stream: None,
            quiet,
        })
    }

    /// Set command timeout
    #[allow(dead_code)]  // Future use
    pub fn set_timeout(&mut self, timeout_secs: u64) {
        self.timeout_duration = Duration::from_secs(timeout_secs);
    }

    /// Connect to the serial device
    pub async fn connect(&mut self) -> Result<()> {
        debug!(
            "Connecting to {} at {} baud",
            self.device_path, self.baud_rate
        );

        // Log port usage at info level unless quiet mode is enabled
        if !self.quiet {
            info!("Using serial port: {} at {} baud", self.device_path, self.baud_rate);
        }

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
        debug!("Successfully connected to {}", self.device_path);

        Ok(())
    }

    /// Send a command and wait for response
    pub async fn send_command(&mut self, command: &str) -> Result<String> {
        // Auto-connect if not already connected
        if self.stream.is_none() {
            debug!("Auto-connecting to device before sending command");
            self.connect().await?;
        }

        let stream = self.stream.as_mut().unwrap();
        debug!("Sending command: {}", command);

        // Send command with newline
        let command_with_newline = format!("{}\n", command);
        stream.write_all(command_with_newline.as_bytes()).await?;
        stream.flush().await?;

        // Read response with timeout
        let response = timeout(self.timeout_duration, async {
            let mut buffer = Vec::new();
            let mut temp_buf = [0u8; 1024];
            
            loop {
                match stream.read(&mut temp_buf).await {
                    Ok(0) => break, // EOF
                    Ok(n) => {
                        buffer.extend_from_slice(&temp_buf[..n]);
                        let response_str = String::from_utf8_lossy(&buffer);
                        
                        // Look for shell prompt indicating end of response
                        if response_str.contains("prod:~$") || response_str.contains("debug:~$") {
                            break;
                        }
                        
                        // Also break on timeout if we have some data
                        if buffer.len() > 0 && response_str.trim().len() > 0 {
                            // Give a small additional delay for any remaining data
                            tokio::time::sleep(Duration::from_millis(100)).await;
                            break;
                        }
                    }
                    Err(e) => return Err(PowerCliError::Io(e)),
                }
            }
            
            Ok(String::from_utf8_lossy(&buffer).to_string())
        })
        .await
        .map_err(|_| PowerCliError::Timeout {
            timeout: self.timeout_duration.as_secs(),
        })??;

        debug!("Received response: {}", response);
        
        // Clean up the response by removing the command echo and prompt
        let cleaned_response = self.clean_response(&response, command);
        Ok(cleaned_response)
    }

    /// Send a command with a short timeout (for commands that may cause connection loss)
    pub async fn send_command_with_short_timeout(&mut self, command: &str) -> Result<String> {
        // Auto-connect if not already connected
        if self.stream.is_none() {
            debug!("Auto-connecting to device before sending command");
            self.connect().await?;
        }

        let stream = self.stream.as_mut().unwrap();
        debug!("Sending command with short timeout: {}", command);

        // Send command with newline
        let command_with_newline = format!("{}\n", command);
        stream.write_all(command_with_newline.as_bytes()).await?;
        stream.flush().await?;

        // Use a very short timeout (500ms) for commands that may cause connection loss
        let short_timeout = Duration::from_millis(500);
        let response = timeout(short_timeout, async {
            let mut buffer = Vec::new();
            let mut temp_buf = [0u8; 1024];
            
            // Try to read some response, but don't wait for full prompt
            match stream.read(&mut temp_buf).await {
                Ok(n) if n > 0 => {
                    buffer.extend_from_slice(&temp_buf[..n]);
                }
                _ => {} // Ignore errors or empty reads
            }
            
            String::from_utf8_lossy(&buffer).to_string()
        })
        .await
        .unwrap_or_else(|_| "Command sent (timeout expected for reset commands)".to_string());

        debug!("Received response (short timeout): {}", response);
        Ok(response)
    }

    /// Clean up the response by removing command echo and shell prompt
    fn clean_response(&self, response: &str, command: &str) -> String {
        let mut lines: Vec<&str> = response.lines().collect();
        
        // Remove command echo (usually the first line)
        if !lines.is_empty() && lines[0].trim() == command.trim() {
            lines.remove(0);
        }
        
        // Remove shell prompt (usually the last line)
        if !lines.is_empty() {
            let last_line = lines[lines.len() - 1].trim();
            if last_line.contains("prod:~$") || last_line.contains("debug:~$") {
                lines.pop();
            }
        }
        
        // Join remaining lines and trim
        lines.join("\n").trim().to_string()
    }

    /// Check if connection is active
    #[allow(dead_code)]  // Future use
    pub fn is_connected(&self) -> bool {
        self.stream.is_some()
    }

    /// Disconnect from the serial device
    #[allow(dead_code)]  // Future use
    pub async fn disconnect(&mut self) {
        if let Some(_stream) = self.stream.take() {
            debug!("Disconnected from {}", self.device_path);
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        if self.stream.is_some() {
            debug!("Connection automatically closed on drop");
        }
    }
}
