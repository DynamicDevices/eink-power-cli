#!/bin/bash
#
# Deploy eink-power-cli to i.MX93 target board
# Usage: ./deploy.sh [target_ip] [target_user]
#

set -e

# Configuration
TARGET_IP="${1:-192.168.1.100}"  # Default IP, override with first argument
TARGET_USER="${2:-root}"         # Default user, override with second argument  
BINARY_PATH="target/aarch64-unknown-linux-gnu/release/eink-power-cli"
TARGET_PATH="/usr/local/bin/eink-power-cli"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if binary exists
if [ ! -f "$BINARY_PATH" ]; then
    print_error "AArch64 binary not found at $BINARY_PATH"
    print_status "Run: cargo build --target aarch64-unknown-linux-gnu --release"
    exit 1
fi

print_status "Deploying eink-power-cli to i.MX93 target board"
print_status "Target: ${TARGET_USER}@${TARGET_IP}"
print_status "Binary: $(ls -lh $BINARY_PATH | awk '{print $5}') AArch64 executable"

# Copy binary to target
print_status "Copying binary to target..."
scp "$BINARY_PATH" "${TARGET_USER}@${TARGET_IP}:${TARGET_PATH}"

# Make executable and test
print_status "Setting permissions and testing..."
ssh "${TARGET_USER}@${TARGET_IP}" << 'EOF'
    chmod +x /usr/local/bin/eink-power-cli
    
    echo "=== Binary Information ==="
    file /usr/local/bin/eink-power-cli
    ls -la /usr/local/bin/eink-power-cli
    
    echo ""
    echo "=== Version Test ==="
    /usr/local/bin/eink-power-cli --version
    
    echo ""
    echo "=== Quick Command Test ==="
    /usr/local/bin/eink-power-cli version
    
    echo ""
    echo "=== Available Serial Devices ==="
    ls -la /dev/tty* | grep -E "(USB|ACM)" || echo "No USB/ACM serial devices found"
EOF

if [ $? -eq 0 ]; then
    print_success "Deployment successful!"
    echo ""
    print_status "Next steps:"
    echo "  1. SSH to target: ssh ${TARGET_USER}@${TARGET_IP}"
    echo "  2. Test commands: eink-power-cli --help"
    echo "  3. Connect to power controller: eink-power-cli --device /dev/ttyUSB0 version"
    echo ""
    print_warning "Note: Ensure the MCXC143VFM power controller is connected via USB serial"
else
    print_error "Deployment failed!"
    exit 1
fi
