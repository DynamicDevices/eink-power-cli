# AArch64 Cross-Compilation for i.MX93

## Quick Start

### 1. Build for AArch64
```bash
cd tools/eink-power-cli
./build-aarch64.sh
```

### 2. Deploy to Target Board
```bash
# Deploy to default IP (192.168.1.100) as root
./deploy.sh

# Deploy to specific IP and user
./deploy.sh 192.168.1.50 root
./deploy.sh your-board-ip your-username
```

### 3. Test on Target
```bash
# SSH to target board
ssh root@192.168.1.100

# Test the CLI
eink-power-cli --version
eink-power-cli --help
eink-power-cli version

# Test with power controller (if connected)
eink-power-cli --device /dev/ttyUSB0 version
eink-power-cli --device /dev/ttyUSB0 ping
```

## Technical Details

### Cross-Compilation Setup
- **Target**: `aarch64-unknown-linux-gnu` (GNU/Linux AArch64)
- **Toolchain**: `aarch64-linux-gnu-gcc`
- **Binary Size**: ~2.0MB (stripped, optimized)
- **Dependencies**: No external system dependencies (libudev avoided)

### Build Configuration
```toml
# .cargo/config.toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

### Deployment Process
1. **Cross-compile** for AArch64 target
2. **Copy binary** to target via SCP
3. **Set permissions** and install to `/usr/local/bin/`
4. **Test functionality** with version and help commands
5. **Check serial devices** available on target

## Troubleshooting

### Build Issues
- **libudev error**: Fixed by disabling serialport default features
- **Cross-compiler not found**: Install with `sudo apt install gcc-aarch64-linux-gnu`
- **Target not installed**: Run `rustup target add aarch64-unknown-linux-gnu`

### Deployment Issues
- **SSH connection failed**: Check target IP and SSH access
- **Permission denied**: Ensure SSH key authentication or password access
- **Binary won't run**: Verify target is AArch64 Linux (not ARM32)

### Runtime Issues
- **Serial device not found**: Check `/dev/ttyUSB*` or `/dev/ttyACM*` devices
- **Permission denied on serial**: Add user to `dialout` group: `usermod -a -G dialout username`
- **Command not found**: Ensure `/usr/local/bin` is in PATH

## Next Steps

Once deployed successfully, you can:

1. **Connect power controller** via USB serial
2. **Test communication** with MCXC143VFM controller
3. **Implement Phase 2** - actual command execution
4. **Replace Python scripts** with Rust CLI tool

## Files Created

- `build-aarch64.sh` - Cross-compilation build script
- `deploy.sh` - Target deployment script  
- `.cargo/config.toml` - Cross-compilation configuration
- `target/aarch64-unknown-linux-gnu/release/eink-power-cli` - AArch64 binary
