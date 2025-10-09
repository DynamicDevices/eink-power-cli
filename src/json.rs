/*
 * JSON Output Formatting for E-ink Power CLI
 * Copyright (c) 2025 Dynamic Devices Ltd
 * All rights reserved.
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Standard JSON response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonResponse {
    pub timestamp: DateTime<Utc>,
    pub command: String,
    pub status: String,
    pub data: Value,
    pub raw_response: Option<String>,
}

impl JsonResponse {
    #[allow(dead_code)] // May be used in future
    pub fn success(command: &str, data: Value) -> Self {
        Self {
            timestamp: Utc::now(),
            command: command.to_string(),
            status: "success".to_string(),
            data,
            raw_response: None,
        }
    }

    pub fn success_with_raw(command: &str, data: Value, raw: &str) -> Self {
        Self {
            timestamp: Utc::now(),
            command: command.to_string(),
            status: "success".to_string(),
            data,
            raw_response: Some(raw.to_string()),
        }
    }

    #[allow(dead_code)] // May be used in future
    pub fn error(command: &str, error: &str) -> Self {
        Self {
            timestamp: Utc::now(),
            command: command.to_string(),
            status: "error".to_string(),
            data: serde_json::json!({"error": error}),
            raw_response: None,
        }
    }
}

/// Battery data structure for JSON output
#[derive(Debug, Serialize, Deserialize)]
pub struct BatteryJson {
    pub voltage_mv: Option<u16>,
    pub current_ma: Option<i16>,
    pub charge_mah: Option<u16>,
    pub power_mw: Option<i32>,
    pub temperature_c: Option<f32>,
}

/// Power management statistics for JSON output
#[derive(Debug, Serialize, Deserialize)]
pub struct PowerStatsJson {
    pub sleep_cycles: Option<u32>,
    pub wake_cycles: Option<u32>,
    pub ltc2959_state: Option<String>,
    pub nfc_state: Option<String>,
    pub uart_state: Option<String>,
    pub uptime_ms: Option<u64>,
}

/// System information for JSON output
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfoJson {
    pub board: Option<String>,
    pub soc: Option<String>,
    pub version: Option<String>,
    pub build_date: Option<String>,
    pub build_type: Option<String>,
    pub uptime: Option<String>,
}

/// GPIO status for JSON output
#[derive(Debug, Serialize, Deserialize)]
pub struct GpioJson {
    pub port: String,
    pub pin: u8,
    pub value: Option<u8>,
    pub direction: Option<String>,
    pub state: Option<String>,
}

/// NFC status for JSON output
#[derive(Debug, Serialize, Deserialize)]
pub struct NfcJson {
    pub status_register: Option<String>,
    pub rf_field: Option<String>,
    pub nfc_active: Option<bool>,
    pub i2c_ready: Option<bool>,
    pub eeprom_status: Option<String>,
    pub sram_status: Option<String>,
}

/// LTC2959 data for JSON output
#[derive(Debug, Serialize, Deserialize)]
pub struct Ltc2959Json {
    pub voltage_mv: Option<u16>,
    pub current_ma: Option<i16>,
    pub charge_mah: Option<u16>,
    pub power_mw: Option<i32>,
    pub status_register: Option<String>,
    pub adc_mode: Option<String>,
    pub coulomb_counter: Option<String>,
    pub charge_complete: Option<bool>,
}

/// Parse PMU responses into structured JSON data
pub struct ResponseParser;

impl ResponseParser {
    /// Parse battery/LTC2959 response into JSON
    pub fn parse_battery_response(response: &str) -> BatteryJson {
        let mut battery = BatteryJson {
            voltage_mv: None,
            current_ma: None,
            charge_mah: None,
            power_mw: None,
            temperature_c: None,
        };

        // Parse voltage (e.g., "Voltage: 6088 mV")
        if let Some(caps) = regex::Regex::new(r"Voltage:\s*(\d+)\s*mV")
            .unwrap()
            .captures(response)
        {
            if let Ok(voltage) = caps[1].parse::<u16>() {
                battery.voltage_mv = Some(voltage);
            }
        }

        // Parse current (e.g., "Current: -170 mA")
        if let Some(caps) = regex::Regex::new(r"Current:\s*(-?\d+)\s*mA")
            .unwrap()
            .captures(response)
        {
            if let Ok(current) = caps[1].parse::<i16>() {
                battery.current_ma = Some(current);
            }
        }

        // Parse charge (e.g., "Charge: 0 mAh")
        if let Some(caps) = regex::Regex::new(r"Charge:\s*(\d+)\s*mAh")
            .unwrap()
            .captures(response)
        {
            if let Ok(charge) = caps[1].parse::<u16>() {
                battery.charge_mah = Some(charge);
            }
        }

        // Parse power (e.g., "Power: -1040 mW")
        if let Some(caps) = regex::Regex::new(r"Power:\s*(-?\d+)\s*mW")
            .unwrap()
            .captures(response)
        {
            if let Ok(power) = caps[1].parse::<i32>() {
                battery.power_mw = Some(power);
            }
        }

        battery
    }

    /// Parse system info response into JSON
    pub fn parse_system_info(response: &str) -> SystemInfoJson {
        let mut info = SystemInfoJson {
            board: None,
            soc: None,
            version: None,
            build_date: None,
            build_type: None,
            uptime: None,
        };

        // Parse board (e.g., "Board: MCXC143VFM E-Ink Power Controller")
        if let Some(caps) = regex::Regex::new(r"Board:\s*(.+)")
            .unwrap()
            .captures(response)
        {
            info.board = Some(caps[1].trim().to_string());
        }

        // Parse SoC (e.g., "SoC: NXP MCXC143VFM (ARM Cortex-M0+)")
        if let Some(caps) = regex::Regex::new(r"SoC:\s*(.+)")
            .unwrap()
            .captures(response)
        {
            info.soc = Some(caps[1].trim().to_string());
        }

        // Parse version (e.g., "Version: 2.2.0-+0fa46fb-dirty.298")
        if let Some(caps) = regex::Regex::new(r"Version:\s*(.+)")
            .unwrap()
            .captures(response)
        {
            info.version = Some(caps[1].trim().to_string());
        }

        // Parse build date (e.g., "Build: 2025-10-09 11:13:59 UTC")
        if let Some(caps) = regex::Regex::new(r"Build:\s*(.+)")
            .unwrap()
            .captures(response)
        {
            info.build_date = Some(caps[1].trim().to_string());
        }

        // Parse build type (e.g., "Build Type: Production")
        if let Some(caps) = regex::Regex::new(r"Build Type:\s*(.+)")
            .unwrap()
            .captures(response)
        {
            info.build_type = Some(caps[1].trim().to_string());
        }

        // Parse uptime (e.g., "System Uptime: 0:01:07 (67427 ms)")
        if let Some(caps) = regex::Regex::new(r"System Uptime:\s*(.+)")
            .unwrap()
            .captures(response)
        {
            info.uptime = Some(caps[1].trim().to_string());
        }

        info
    }

    /// Parse NFC status response into JSON
    pub fn parse_nfc_status(response: &str) -> NfcJson {
        let mut nfc = NfcJson {
            status_register: None,
            rf_field: None,
            nfc_active: None,
            i2c_ready: None,
            eeprom_status: None,
            sram_status: None,
        };

        // Parse status register (e.g., "NTA5332 Status: 0x02")
        if let Some(caps) = regex::Regex::new(r"NTA5332 Status:\s*(0x[0-9A-Fa-f]+)")
            .unwrap()
            .captures(response)
        {
            nfc.status_register = Some(caps[1].to_string());
        }

        // Parse RF field (e.g., "RF Field: Absent")
        if let Some(caps) = regex::Regex::new(r"RF Field:\s*(.+)")
            .unwrap()
            .captures(response)
        {
            nfc.rf_field = Some(caps[1].trim().to_string());
        }

        // Parse NFC active (e.g., "NFC Active: NO")
        if response.contains("NFC Active: YES") {
            nfc.nfc_active = Some(true);
        } else if response.contains("NFC Active: NO") {
            nfc.nfc_active = Some(false);
        }

        // Parse I2C ready (e.g., "I2C Ready: NO")
        if response.contains("I2C Ready: YES") {
            nfc.i2c_ready = Some(true);
        } else if response.contains("I2C Ready: NO") {
            nfc.i2c_ready = Some(false);
        }

        // Parse EEPROM status (e.g., "EEPROM: Ready")
        if let Some(caps) = regex::Regex::new(r"EEPROM:\s*(.+)")
            .unwrap()
            .captures(response)
        {
            nfc.eeprom_status = Some(caps[1].trim().to_string());
        }

        nfc
    }

    /// Parse LTC2959 status response into JSON
    pub fn parse_ltc2959_status(response: &str) -> Ltc2959Json {
        let mut ltc = Ltc2959Json {
            voltage_mv: None,
            current_ma: None,
            charge_mah: None,
            power_mw: None,
            status_register: None,
            adc_mode: None,
            coulomb_counter: None,
            charge_complete: None,
        };

        // Parse status register (e.g., "LTC2959 Status Register: 0x01")
        if let Some(caps) = regex::Regex::new(r"LTC2959 Status Register:\s*(0x[0-9A-Fa-f]+)")
            .unwrap()
            .captures(response)
        {
            ltc.status_register = Some(caps[1].to_string());
        }

        // Parse ADC mode (e.g., "ADC Mode: Smart Sleep")
        if let Some(caps) = regex::Regex::new(r"ADC Mode:\s*(.+)")
            .unwrap()
            .captures(response)
        {
            ltc.adc_mode = Some(caps[1].trim().to_string());
        }

        // Parse coulomb counter (e.g., "Coulomb Counter: Disabled")
        if let Some(caps) = regex::Regex::new(r"Coulomb Counter:\s*(.+)")
            .unwrap()
            .captures(response)
        {
            ltc.coulomb_counter = Some(caps[1].trim().to_string());
        }

        // Also parse any voltage/current/charge data if present
        let battery_data = Self::parse_battery_response(response);
        ltc.voltage_mv = battery_data.voltage_mv;
        ltc.current_ma = battery_data.current_ma;
        ltc.charge_mah = battery_data.charge_mah;
        ltc.power_mw = battery_data.power_mw;

        ltc
    }

    /// Parse GPIO response into JSON
    pub fn parse_gpio_response(response: &str, port: &str, pin: u8) -> GpioJson {
        let mut gpio = GpioJson {
            port: port.to_string(),
            pin,
            value: None,
            direction: None,
            state: None,
        };

        // Parse GPIO value (e.g., "GPIO A0: 1" or "Pin value: 0")
        if let Some(caps) = regex::Regex::new(r"(?:GPIO [A-Z]\d+:\s*|Pin value:\s*)([01])")
            .unwrap()
            .captures(response)
        {
            if let Ok(value) = caps[1].parse::<u8>() {
                gpio.value = Some(value);
            }
        }

        // Parse direction/state information if present
        if response.contains("INPUT") {
            gpio.direction = Some("INPUT".to_string());
        } else if response.contains("OUTPUT") {
            gpio.direction = Some("OUTPUT".to_string());
        }

        if response.contains("HIGH") {
            gpio.state = Some("HIGH".to_string());
        } else if response.contains("LOW") {
            gpio.state = Some("LOW".to_string());
        }

        gpio
    }
}
