# E-ink Power CLI

Command-line interface for communicating with the MCXC143VFM/MCXC144VFM E-ink power management controller over serial UART.

**✨ Version 2.3.0 - Enhanced with comprehensive firmware management capabilities!**

[![CI/CD Pipeline](https://github.com/DynamicDevices/eink-power-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/DynamicDevices/eink-power-cli/actions/workflows/ci.yml)
[![Maintenance](https://github.com/DynamicDevices/eink-power-cli/actions/workflows/maintenance.yml/badge.svg)](https://github.com/DynamicDevices/eink-power-cli/actions/workflows/maintenance.yml)
[![Release](https://img.shields.io/github/v/release/DynamicDevices/eink-power-cli?include_prereleases)](https://github.com/DynamicDevices/eink-power-cli/releases)
[![License: Commercial](https://img.shields.io/badge/License-Commercial-red.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-Linux-green.svg)](https://www.kernel.org)
[![Targets](https://img.shields.io/badge/targets-x86__64%20%7C%20ARM64-brightgreen.svg)](#cross-compilation-for-arm64)

## Overview

The E-ink Power CLI is a Rust-based command-line tool designed to communicate with the MCXC143VFM/MCXC144VFM power management controller. It provides:

- **Power Management**: Control PMIC, WiFi, and display power rails
- **Battery Monitoring**: Real-time LTC2959 coulomb counter readings
- **System Control**: GPIO manipulation, system information, and diagnostics
- **🆕 Firmware Management**: Upload firmware via mcumgr with progress indication (v2.3.0)
- **Board Control**: E-Ink controller reset and power cycling
- **Automation**: JSON/CSV output formats and batch operations

## What's New in v2.3.0

- **🚀 Firmware Management**: Complete firmware upload with mcumgr integration
- **📊 Progress Indication**: Real-time upload progress and boot countdown
- **🔄 Automated Process**: Reset → Upload → Reboot → Verify in one command
- **📋 Firmware Info**: List installed images and bootloader status
- **🔧 Improved GPIO Commands**: Enhanced `gpio get` and `gpio set` functionality
- **⚡ Better Power Control**: Refined power management commands for all rails
- **🚀 Full Command Framework**: Complete implementation of all planned CLI commands
- **🎯 Version Alignment**: CLI version now matches PMU firmware version for clarity

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

# Reset the E-Ink controller board (NEW in v2.2.0+)
eink-power-cli board reset

# Control GPIO pins
eink-power-cli gpio get gpioa 5
eink-power-cli gpio set gpiob 3 1

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

### Cross-Compilation for ARM64

```bash
# Using the provided script (recommended)
./build-aarch64.sh

# Manual cross-compilation
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
```

### Development Helper Script

The project includes a comprehensive development script (`./dev.sh`) for common tasks:

```bash
# Set up development environment with Docker
./dev.sh setup

# Build for different targets
./dev.sh build           # Native build
./dev.sh build-arm64     # ARM64 build
./dev.sh build-all       # All targets

# Testing and quality assurance
./dev.sh test            # Run tests
./dev.sh test-ci         # Run full CI pipeline
./dev.sh lint            # Code quality checks

# Docker development
./dev.sh docker-dev      # Start development container
./dev.sh docker-serial   # Container with serial access
./dev.sh docker-ci       # Run CI in container

# Deployment and monitoring
./dev.sh deploy          # Deploy to target device
./dev.sh monitor         # Monitor serial output
./dev.sh release         # Create release artifacts
```

## CI/CD Pipeline

This project includes a comprehensive CI/CD pipeline with GitHub Actions:

### 🔄 Continuous Integration Features

- **Multi-target builds**: x86_64, ARM64
- **Code quality enforcement**: Formatting, linting (Clippy), security audit
- **Comprehensive testing**: Unit tests, integration tests, documentation tests
- **Docker-based builds**: Consistent, reproducible environment using `rust:1.81-bullseye`
- **Artifact archiving**: Pre-built binaries with checksums for all platforms

### 📦 Automated Release Process

- **Tagged releases**: Automatic binary builds and GitHub releases on version tags
- **Multi-platform artifacts**: 
  - `eink-power-cli-linux-x64` (x86_64)
  - `eink-power-cli-linux-arm64` (ARM64)
- **Integrity verification**: SHA256 and MD5 checksums for all binaries
- **Release notes**: Auto-generated with build information and installation instructions

### 🛠️ Development Workflows

#### Main CI Pipeline (`.github/workflows/ci.yml`)
- **Triggers**: Push to `main`/`develop`, pull requests, version tags
- **Jobs**: Test & Quality → Security Audit → Multi-target Build → Release (on tags)
- **Docker**: Uses official Rust container with cross-compilation tools
- **Caching**: Cargo registry and build artifacts for faster builds

#### Maintenance Pipeline (`.github/workflows/maintenance.yml`)
- **Scheduled**: Weekly dependency updates and security monitoring
- **Dependency updates**: Automated PRs for patch version updates
- **Security monitoring**: Automatic issue creation for vulnerabilities
- **Code metrics**: Regular quality analysis and reporting

### 🔍 Quality Assurance

#### Automated Checks
```bash
# Code formatting (enforced)
cargo fmt --all -- --check

# Linting with strict warnings
cargo clippy --all-targets --all-features -- -D warnings

# Security vulnerability scanning
cargo audit

# Documentation generation
cargo doc --no-deps --document-private-items
```

#### Local CI Simulation
```bash
# Run the full CI pipeline locally
./dev.sh test-ci

# Or using Docker
docker-compose up ci
```

### 🐳 Docker Development Environment

The project includes a complete Docker setup for consistent development:

#### Development Container (`Dockerfile`)
- **Base**: `rust:1.81-bullseye` with cross-compilation tools
- **Tools**: Clippy, rustfmt, cargo-audit, cargo-bloat, etc.
- **Cross-compilation**: Pre-configured for ARM64 targets
- **Non-root user**: Security-focused development environment

#### Docker Compose Services (`docker-compose.yml`)
- **`dev`**: Main development environment
- **`dev-serial`**: Development with serial device access
- **`ci`**: Local CI pipeline simulation
- **`docs`**: Documentation server on port 8000

#### Usage Examples
```bash
# Build and start development environment
docker-compose up dev

# Development with hardware access
docker-compose up dev-serial

# Run CI pipeline locally
docker-compose up ci

# Start documentation server
docker-compose up docs
```

### 📊 Build Artifacts

Each successful build produces:
- **Binaries**: Optimized release builds for all targets
- **Checksums**: SHA256 and MD5 verification files
- **Build info**: Commit hash, build date, Rust version
- **Documentation**: Generated API docs
- **Test reports**: Coverage and test results

### 🔧 Development Best Practices

#### Code Quality
- **Formatting**: Enforced with `rustfmt`
- **Linting**: Strict Clippy rules with zero warnings policy
- **Testing**: Comprehensive unit and integration test coverage
- **Documentation**: All public APIs must be documented

#### Security
- **Dependency auditing**: Weekly vulnerability scans
- **Minimal dependencies**: Only essential crates included
- **Static analysis**: Clippy security lints enabled
- **Container security**: Non-root development user

#### Performance
- **Release builds**: LTO and strip enabled for minimal binary size
- **Target optimization**: Specific builds for each platform
- **Caching**: Aggressive caching of dependencies and build artifacts

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
