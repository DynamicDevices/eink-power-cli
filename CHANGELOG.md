# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-01-09

### Added
- Support for PMU firmware v2.2.0+ features
- New `board reset` command for E-Ink controller power cycling
- Enhanced command execution framework with proper error handling
- Complete CLI command structure for all PMU operations
- Version detection and system information commands
- GPIO control commands (`gpio get`, `gpio set`)
- Power control commands (`power pmic`, `power wifi`, `power disp`)
- Improved version format support (semantic versioning with build metadata)

### Changed
- Updated CLI framework to support new board control commands
- Enhanced protocol handler for board command execution
- Improved error handling and user feedback
- Updated package description to reflect PMU v2.2.0+ support

### Fixed
- Compilation errors with command cloning and borrowing
- Added missing Clone trait implementations for command enums

## [Unreleased]

### Added
- Initial project setup and repository structure
- Comprehensive implementation plan and documentation
- Cargo.toml with proper metadata and dependencies
- Commercial license and copyright notices

### Changed
- N/A

### Deprecated
- N/A

### Removed
- N/A

### Fixed
- N/A

### Security
- N/A

## [0.1.0] - 2025-01-06

### Added
- **Foundation Phase**: Complete project structure and CLI framework
- Serial communication framework with tokio-serial
- Basic CLI interface with clap derive macros
- Core system commands (version, ping, system info, reboot)
- Power control commands (PMIC, WiFi, display)
- Battery monitoring commands (LTC2959 integration ready)
- GPIO control commands with proper argument parsing
- NFC interface commands (NTA5332 integration ready)
- Human-readable and JSON output formats
- Configuration file support with TOML format
- Error handling and logging infrastructure
- **AArch64 Cross-Compilation**: Complete toolchain setup
- Cross-compilation configuration for i.MX93 target
- Deployment scripts for target board testing
- **Target Validation**: Successfully deployed and tested
- SSH deployment to i.MX93 board (fio@62.3.79.162:25)
- All CLI commands verified working on target
- Ready for Phase 2 serial communication implementation

### Technical Details
- Rust 2021 edition with modern async/await patterns
- Tokio async runtime for serial communication
- Structured error handling with thiserror
- Configuration management with config crate
- Professional CLI interface with comprehensive help system
- AArch64 binary: 2.0MB optimized release build
- Target: Linux-microPlatform Dynamic Devices Headless 4.0.20-2130-94

### Infrastructure
- Complete Git repository with commercial licensing
- Comprehensive documentation and implementation plan
- Cross-compilation toolchain for aarch64-unknown-linux-gnu
- Automated build and deployment scripts
- Integration with parent eink-microcontroller project as submodule

---

## Planned Releases

### [0.2.0] - Battery Monitoring Phase
**Target Date**: Week 3-4

#### Planned Features
- LTC2959 coulomb counter integration
- Real-time battery readings (voltage, current, charge, temperature)
- Power management statistics collection
- GPIO control interface
- Automated monitoring capabilities
- Script-friendly output formats

### [0.3.0] - Advanced Features Phase
**Target Date**: Week 5-6

#### Planned Features
- NFC controller interface (NTA5332)
- Advanced power management modes
- Batch command execution from files
- Continuous monitoring with alerts
- Enhanced automation support

### [0.4.0] - Production Ready Phase
**Target Date**: Week 7-8

#### Planned Features
- Yocto integration and Debian packaging
- Systemd service integration
- Structured logging and metrics collection
- Configuration management improvements
- Performance optimizations
- Production deployment documentation

---

## Version History Template

```
## [X.Y.Z] - YYYY-MM-DD

### Added
- New features

### Changed
- Changes to existing functionality

### Deprecated
- Soon-to-be removed features

### Removed
- Now removed features

### Fixed
- Bug fixes

### Security
- Security improvements
```

---

## Maintenance Notes

- **Versioning**: Follow semantic versioning (MAJOR.MINOR.PATCH)
- **Release Cadence**: Weekly releases during development phase
- **Documentation**: Update README.md with each release
- **Testing**: All releases require passing integration tests
- **Compatibility**: Maintain backward compatibility within major versions

## Contact

- **Maintainer**: Alex J Lennon <ajlennon@dynamicdevices.co.uk>
- **Company**: Dynamic Devices Ltd
- **Contact**: info@dynamicdevices.co.uk
