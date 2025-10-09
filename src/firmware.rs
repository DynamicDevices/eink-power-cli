/*
 * Firmware Management Module for E-ink Power CLI
 * Copyright (c) 2025 Dynamic Devices Ltd
 * All rights reserved.
 */

use crate::error::PowerCliError;
use crate::serial::Connection;
use log::{debug, info, warn};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time::sleep;

/// Firmware management interface
pub struct FirmwareManager {
    connection: Connection,
    mcumgr_port: String,
    mcumgr_baud: u32,
}

impl FirmwareManager {
    /// Create a new firmware manager
    pub fn new(connection: Connection, port: Option<String>, baud: u32) -> Self {
        Self {
            connection,
            mcumgr_port: port.unwrap_or_else(|| "/dev/ttyLP2".to_string()),
            mcumgr_baud: baud,
        }
    }

    /// List installed firmware images using mcumgr
    pub async fn list_images(&mut self) -> Result<String, PowerCliError> {
        info!("Listing firmware images using mcumgr");

        let output = Command::new("mcumgr")
            .args(&[
                "--conntype",
                "serial",
                "--connstring",
                &format!("{},baud={}", self.mcumgr_port, self.mcumgr_baud),
                "image",
                "list",
            ])
            .output()
            .map_err(|e| PowerCliError::Io(e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(PowerCliError::FirmwareError {
                message: format!("mcumgr image list failed: {}", stderr),
            });
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.to_string())
    }

    /// Get firmware slot information
    pub async fn get_info(&mut self) -> Result<String, PowerCliError> {
        info!("Getting firmware slot information");

        // Try to get image list first
        let images = self.list_images().await?;

        // Also try to get bootloader info if available
        let bootloader_info = self
            .get_bootloader_info()
            .await
            .unwrap_or_else(|_| "Bootloader info not available".to_string());

        Ok(format!(
            "=== Firmware Information ===\n\n--- Images ---\n{}\n--- Bootloader ---\n{}",
            images, bootloader_info
        ))
    }

    /// Reset PMU into bootloader mode
    pub async fn reset_to_bootloader(&mut self) -> Result<String, PowerCliError> {
        info!("Resetting PMU to bootloader mode");

        // Try to send system reset command to PMU
        // This may fail if PMU is already in bootloader mode, which is fine
        match self.send_system_reset().await {
            Ok(response) => {
                info!("System reset command sent successfully");
                debug!("Reset response: {}", response);
            }
            Err(e) => {
                warn!(
                    "System reset command failed (PMU may already be in bootloader mode): {}",
                    e
                );
            }
        }

        // Wait a bit for the reset to complete
        sleep(Duration::from_millis(2000)).await;

        // Verify we can communicate with bootloader
        match self.verify_bootloader_mode().await {
            Ok(_) => {
                info!("PMU is now in bootloader mode");
                Ok("PMU successfully reset to bootloader mode".to_string())
            }
            Err(e) => {
                warn!("Could not verify bootloader mode: {}", e);
                // Don't fail - the bootloader might be there but not responding to our test
                Ok("Reset command sent, PMU should be in bootloader mode".to_string())
            }
        }
    }

    /// Upload firmware image
    pub async fn upload_firmware(
        &mut self,
        firmware_path: &Path,
        skip_reset: bool,
    ) -> Result<String, PowerCliError> {
        println!("🚀 Starting firmware upload process...");
        println!("📁 Firmware file: {}", firmware_path.display());

        // Check if firmware file exists
        if !firmware_path.exists() {
            return Err(PowerCliError::FirmwareError {
                message: format!("Firmware file not found: {}", firmware_path.display()),
            });
        }

        let mut results = Vec::new();

        // Step 1: Reset to bootloader mode (unless skipped)
        if !skip_reset {
            println!("\n🔄 Step 1/4: Resetting PMU to bootloader mode...");
            let reset_result = self.reset_to_bootloader().await?;
            results.push(format!("✅ Reset: {}", reset_result));
            println!("   {}", reset_result);
        } else {
            println!("\n⏭️  Step 1/4: Skipping reset (assuming bootloader mode)");
            results.push("⏭️  Reset: Skipped (assuming bootloader mode)".to_string());
        }

        // Step 2: Upload firmware using mcumgr
        println!("\n📤 Step 2/4: Uploading firmware...");
        let upload_result = self.mcumgr_upload(firmware_path).await?;
        results.push(format!("✅ Upload: {}", upload_result));
        println!("   {}", upload_result);

        // Step 3: Reset PMU to run new firmware
        println!("\n🔄 Step 3/4: Resetting PMU to run new firmware...");
        let final_reset_result = self.mcumgr_reset().await?;
        results.push(format!("✅ Final Reset: {}", final_reset_result));
        println!("   {}", final_reset_result);

        // Step 4: Wait for firmware to boot with progress indication
        println!("\n⏳ Step 4/4: Waiting for firmware to boot (15 seconds)...");

        // Show countdown progress
        for i in (1..=15).rev() {
            print!("\r⏱️  Waiting for boot... {} seconds remaining", i);
            std::io::stdout().flush().unwrap();
            sleep(Duration::from_millis(1000)).await;
        }
        print!("\r✅ Boot wait completed!                        \n");

        println!("🔍 Verifying new firmware...");
        match self.verify_new_firmware().await {
            Ok(version_info) => {
                results.push(format!("✅ Verification: {}", version_info));
                println!("   ✅ {}", version_info);
            }
            Err(e) => {
                warn!("Could not verify new firmware: {}", e);
                results.push(
                    "⚠️  Verification: Could not verify new firmware (may still be booting)"
                        .to_string(),
                );
                println!("   ⚠️  Could not verify new firmware (may still be booting)");
            }
        }

        println!("\n🎉 Firmware update process completed!");
        Ok(results.join("\n"))
    }

    /// Send system reset command to PMU
    async fn send_system_reset(&mut self) -> Result<String, PowerCliError> {
        debug!("Sending system reset command to PMU");

        // Connect to PMU and send reset command
        self.connection.connect().await?;
        let response = self.connection.send_command("system reset").await?;

        Ok(response)
    }

    /// Verify PMU is in bootloader mode
    async fn verify_bootloader_mode(&mut self) -> Result<String, PowerCliError> {
        debug!("Verifying bootloader mode with mcumgr");

        let output = Command::new("mcumgr")
            .args(&[
                "--conntype",
                "serial",
                "--connstring",
                &format!("{},baud={}", self.mcumgr_port, self.mcumgr_baud),
                "echo",
                "bootloader_test",
            ])
            .output()
            .map_err(|e| PowerCliError::Io(e))?;

        if output.status.success() {
            Ok("Bootloader responding".to_string())
        } else {
            Err(PowerCliError::FirmwareError {
                message: "Bootloader not responding".to_string(),
            })
        }
    }

    /// Upload firmware using mcumgr
    async fn mcumgr_upload(&mut self, firmware_path: &Path) -> Result<String, PowerCliError> {
        info!("Uploading firmware: {}", firmware_path.display());

        // Get file size for progress indication
        let file_size = std::fs::metadata(firmware_path)
            .map_err(|e| PowerCliError::Io(e))?
            .len();

        println!(
            "📦 Starting upload of {} ({} bytes)...",
            firmware_path.file_name().unwrap().to_string_lossy(),
            file_size
        );

        let mut child = Command::new("mcumgr")
            .args(&[
                "--conntype",
                "serial",
                "--connstring",
                &format!("{},baud={}", self.mcumgr_port, self.mcumgr_baud),
                "image",
                "upload",
                firmware_path.to_str().unwrap(),
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| PowerCliError::Io(e))?;

        // Show progress while the upload is running
        let mut progress_counter = 0;
        let progress_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

        loop {
            match child.try_wait() {
                Ok(Some(status)) => {
                    // Process finished
                    print!("\r✅ Upload completed!                    \n");

                    let output = child.wait_with_output().map_err(|e| PowerCliError::Io(e))?;

                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);

                    if !status.success() {
                        return Err(PowerCliError::FirmwareError {
                            message: format!("mcumgr upload failed: {}\n{}", stderr, stdout),
                        });
                    }

                    return Ok(format!(
                        "Firmware uploaded successfully: {}",
                        firmware_path.file_name().unwrap().to_string_lossy()
                    ));
                }
                Ok(None) => {
                    // Process still running, show progress
                    let spinner = progress_chars[progress_counter % progress_chars.len()];
                    print!("\r{} Uploading firmware... Please wait", spinner);
                    std::io::stdout().flush().unwrap();
                    progress_counter += 1;

                    // Wait a bit before checking again
                    sleep(Duration::from_millis(100)).await;
                }
                Err(e) => {
                    return Err(PowerCliError::Io(e));
                }
            }
        }
    }

    /// Reset PMU using mcumgr
    async fn mcumgr_reset(&mut self) -> Result<String, PowerCliError> {
        info!("Resetting PMU using mcumgr");

        let output = Command::new("mcumgr")
            .args(&[
                "--conntype",
                "serial",
                "--connstring",
                &format!("{},baud={}", self.mcumgr_port, self.mcumgr_baud),
                "reset",
            ])
            .output()
            .map_err(|e| PowerCliError::Io(e))?;

        // mcumgr reset may not return success if the device resets immediately
        // So we don't strictly check the exit code
        let _stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !stderr.is_empty() && !stderr.contains("timeout") {
            warn!("mcumgr reset stderr: {}", stderr);
        }

        Ok("PMU reset command sent".to_string())
    }

    /// Verify new firmware is running
    async fn verify_new_firmware(&mut self) -> Result<String, PowerCliError> {
        debug!("Verifying new firmware is running");

        // Give firmware a bit more time to fully initialize
        sleep(Duration::from_millis(2000)).await;

        // Try to connect and get version
        self.connection.connect().await?;
        let response = self.connection.send_command("version").await?;

        Ok(format!(
            "New firmware version: {}",
            response.lines().next().unwrap_or("Unknown")
        ))
    }

    /// Get bootloader information
    async fn get_bootloader_info(&mut self) -> Result<String, PowerCliError> {
        debug!("Getting bootloader information");

        let output = Command::new("mcumgr")
            .args(&[
                "--conntype",
                "serial",
                "--connstring",
                &format!("{},baud={}", self.mcumgr_port, self.mcumgr_baud),
                "taskstat",
            ])
            .output()
            .map_err(|e| PowerCliError::Io(e))?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(stdout.to_string())
        } else {
            Err(PowerCliError::FirmwareError {
                message: "Could not get bootloader info".to_string(),
            })
        }
    }
}
