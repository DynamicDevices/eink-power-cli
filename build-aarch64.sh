#!/bin/bash
#
# Build script for AArch64 cross-compilation
#

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_status "Building eink-power-cli for AArch64 (i.MX93)"

# Build for AArch64 target
PKG_CONFIG_ALLOW_CROSS=1 cargo build --target aarch64-unknown-linux-gnu --release

# Show binary information
BINARY_PATH="target/aarch64-unknown-linux-gnu/release/eink-power-cli"
if [ -f "$BINARY_PATH" ]; then
    print_success "Build completed successfully!"
    echo ""
    echo "Binary Information:"
    echo "  Path: $BINARY_PATH"
    echo "  Size: $(ls -lh $BINARY_PATH | awk '{print $5}')"
    echo "  Type: $(file $BINARY_PATH)"
    echo ""
    echo "Deploy to target:"
    echo "  ./deploy.sh [target_ip] [target_user]"
else
    echo "Build failed - binary not found"
    exit 1
fi
