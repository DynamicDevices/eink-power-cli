# E-ink Power CLI

Command-line interface for communicating with the MCXC143VFM E-ink power management controller over serial UART.

[![License: Commercial](https://img.shields.io/badge/License-Commercial-red.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-Linux-green.svg)](https://www.kernel.org)

## Overview

The E-ink Power CLI is a Rust-based command-line tool designed to communicate with the MCXC143VFM power management controller. It provides a simple, extensible interface for:

- **Power Management**: Control PMIC, WiFi, and display power rails
- **Battery Monitoring**: Real-time LTC2959 coulomb counter readings
- **System Control**: GPIO manipulation, system information, and diagnostics
- **NFC Interface**: NTA5332 controller status and field detection
- **Automation**: Script-friendly output formats and batch operations

## Quick Start

### Installation

```bash
# Clone the repository
git clone git@github.com:DynamicDevices/eink-power-cli.git
cd eink-power-cli

# Build the project
cargo build --release

# Install locally
cargo install --path .
```

### Basic Usage

```bash
# Check controller version
eink-power-cli version

# Read battery status
eink-power-cli battery read

# Control power rails
eink-power-cli power pmic on
eink-power-cli power wifi off

# Get system information
eink-power-cli system info

# Monitor continuously
eink-power-cli monitor --interval 30s
```

## Features

### ✅ **Phase 1: Foundation (v0.1.0)**
- Serial communication with MCXC143VFM controller
- Basic system commands (version, ping, info, reboot)
- Power control (PMIC, WiFi, display)
- Human-readable and JSON output formats

### 🔄 **Phase 2: Battery Monitoring (v0.2.0)**
- LTC2959 coulomb counter integration
- Real-time battery readings (voltage, current, charge, temperature)
- Power management statistics
- GPIO control interface

### 📋 **Phase 3: Advanced Features (v0.3.0)**
- NFC controller interface (NTA5332)
- Advanced power management modes
- Batch command execution
- Continuous monitoring with alerts

### 🚀 **Phase 4: Production Ready (v0.4.0)**
- Yocto integration and packaging
- Systemd service support
- Structured logging and metrics
- Configuration management

## Command Reference

### System Commands
```bash
eink-power-cli version                    # Controller firmware version
eink-power-cli ping                       # Connectivity test
eink-power-cli system info                # System information
eink-power-cli system reboot              # Restart controller
```

### Power Management
```bash
eink-power-cli power pmic on|off          # Control main PMIC
eink-power-cli power wifi on|off          # Control WiFi module
eink-power-cli power disp on|off          # Control display
eink-power-cli pm stats                   # Power management statistics
eink-power-cli pm sleep [timeout]         # Enter deep sleep
```

### Battery Monitoring
```bash
eink-power-cli battery read               # Read all measurements
eink-power-cli battery status             # Battery status
eink-power-cli battery enable|disable     # Enable/disable monitoring
```

### GPIO Control
```bash
eink-power-cli gpio get <port> <pin>      # Read GPIO state
eink-power-cli gpio set <port> <pin> <val> # Set GPIO state
```

### NFC Interface
```bash
eink-power-cli nfc status                 # NFC controller status
eink-power-cli nfc info                   # Device information
eink-power-cli nfc field-detect           # Check field detection
```

## Configuration

Create a configuration file at `~/.config/eink-power-cli/config.toml`:

```toml
[connection]
device = "/dev/ttyUSB0"
baud_rate = 115200
timeout = 3

[output]
format = "human"  # human, json, csv
timestamps = true
colors = true

[monitoring]
default_interval = 30
alert_thresholds = { voltage_low = 3200, temperature_high = 60 }

[logging]
level = "info"
file = "/var/log/eink-power-cli.log"
```

## Output Formats

### Human-Readable (Default)
```
📊 LTC2959 Measurements:
   🔋 Voltage: 3850 mV
   ⚡ Current: 125 mA
   🔋 Charge: 2450 mAh
   🌡️  Temperature: 23°C
```

### JSON Format
```bash
eink-power-cli --format json battery read
```
```json
{
  "timestamp": "2025-01-06T10:30:00Z",
  "command": "battery_read",
  "status": "success",
  "data": {
    "voltage_mv": 3850,
    "current_ma": 125,
    "charge_mah": 2450,
    "temperature_c": 23
  }
}
```

## Integration Examples

### Shell Scripts
```bash
#!/bin/bash
# Power-on sequence
eink-power-cli power pmic on
sleep 2
eink-power-cli power wifi on
eink-power-cli power disp on

# Check battery health
BATTERY=$(eink-power-cli --format json battery read)
VOLTAGE=$(echo $BATTERY | jq -r '.data.voltage_mv')
if [ $VOLTAGE -lt 3200 ]; then
    echo "Low battery warning: ${VOLTAGE}mV"
fi
```

### Python Integration
```python
import subprocess
import json

def get_battery_status():
    result = subprocess.run([
        'eink-power-cli', '--format', 'json', 'battery', 'read'
    ], capture_output=True, text=True)
    
    if result.returncode == 0:
        return json.loads(result.stdout)
    else:
        raise Exception(f"Command failed: {result.stderr}")

battery = get_battery_status()
print(f"Battery voltage: {battery['data']['voltage_mv']}mV")
```

### Systemd Service
```ini
[Unit]
Description=E-ink Power Monitor
After=network.target

[Service]
Type=simple
User=root
ExecStart=/usr/local/bin/eink-power-cli monitor --continuous --interval 60s
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

## Development

### Building from Source
```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- version
```

### Testing
```bash
# Unit tests
cargo test

# Integration tests (requires hardware)
cargo test --test integration_tests

# Mock tests (no hardware required)
cargo test --test mock_serial
```

## Hardware Requirements

- **Target Platform**: Yocto Foundries.io Linux Microplatform (i.MX93)
- **Serial Connection**: UART at 115200 baud (typically `/dev/ttyUSB0`)
- **Controller**: MCXC143VFM with E-ink power management firmware
- **Minimum Rust Version**: 1.70+

## Troubleshooting

### Common Issues

**Serial port not found**:
```bash
# Check available ports
ls /dev/ttyUSB*
# or
eink-power-cli --device /dev/ttyACM0 version
```

**Permission denied**:
```bash
# Add user to dialout group
sudo usermod -a -G dialout $USER
# Log out and back in
```

**Command timeout**:
```bash
# Increase timeout
eink-power-cli --timeout 10 battery read
```

**Controller not responding**:
```bash
# Check connection
eink-power-cli ping
# Check system status
eink-power-cli system info
```

## Support

- **Maintainer**: Alex J Lennon <ajlennon@dynamicdevices.co.uk>
- **Company**: Dynamic Devices Ltd
- **Contact**: info@dynamicdevices.co.uk
- **Issues**: [GitHub Issues](https://github.com/DynamicDevices/eink-power-cli/issues)
- **Documentation**: [Project Wiki](https://github.com/DynamicDevices/eink-power-cli/wiki)

## License

Copyright (c) 2025 Dynamic Devices Ltd. All rights reserved.

This software is proprietary and confidential. See [LICENSE](LICENSE) for full terms.

## Related Projects

- [eink-microcontroller](https://github.com/DynamicDevices/eink-microcontroller) - MCXC143VFM firmware
- [Yocto Foundries.io](https://foundries.io/) - Linux Microplatform
