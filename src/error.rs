/*
 * E-ink Power CLI - Error Handling
 * Copyright (c) 2025 Dynamic Devices Ltd
 * All rights reserved.
 */

use thiserror::Error;

/// Main error type for the E-ink Power CLI application
#[derive(Error, Debug)]
pub enum PowerCliError {
    /// Serial communication errors
    #[error("Serial communication error: {0}")]
    Serial(#[from] serialport::Error),

    /// Tokio serial errors
    #[error("Async serial error: {0}")]
    TokioSerial(String),

    /// I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Command timeout
    #[error("Command timeout after {timeout}s")]
    Timeout { timeout: u64 },

    /// Invalid response from controller
    #[error("Invalid response from controller: {response}")]
    InvalidResponse { response: String },

    /// Controller returned an error
    #[error("Controller error: {message}")]
    ControllerError { message: String },

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    /// JSON parsing errors
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    /// Device not found
    #[error("Device not found: {device}")]
    DeviceNotFound { device: String },

    /// Connection not established
    #[error("Connection not established - call connect() first")]
    NotConnected,

    /// Invalid command format
    #[error("Invalid command format: {command}")]
    InvalidCommand { command: String },

    /// Battery monitoring error
    #[error("Battery monitoring error: {message}")]
    BatteryError { message: String },

    /// Power control error
    #[error("Power control error: {message}")]
    PowerError { message: String },

    /// NFC interface error
    #[error("NFC interface error: {message}")]
    NfcError { message: String },

    /// GPIO control error
    #[error("GPIO control error: {message}")]
    GpioError { message: String },
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, PowerCliError>;
