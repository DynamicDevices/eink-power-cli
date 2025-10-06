#!/bin/bash
#
# Deploy to the specific i.MX93 target board
# Target: 62.3.79.162:25 user fio
#

set -e

BINARY_PATH="target/aarch64-unknown-linux-gnu/release/eink-power-cli"
TARGET_HOST="fio@62.3.79.162"
TARGET_PORT="25"

echo "🚀 Deploying to i.MX93 Jaguar E-ink Board"
echo "   Target: $TARGET_HOST:$TARGET_PORT"

# Copy binary
echo "📦 Copying binary..."
scp -P $TARGET_PORT $BINARY_PATH $TARGET_HOST:~/bin/eink-power-cli

# Set permissions and test
echo "✅ Setting permissions and testing..."
ssh -p $TARGET_PORT $TARGET_HOST << 'EOF'
    chmod +x ~/bin/eink-power-cli
    echo "=== Deployment Complete ==="
    echo "Binary: $(ls -lh ~/bin/eink-power-cli | awk '{print $5}')"
    echo "Version: $(~/bin/eink-power-cli --version)"
    echo ""
    echo "Usage:"
    echo "  ~/bin/eink-power-cli --help"
    echo "  ~/bin/eink-power-cli version"
    echo "  ~/bin/eink-power-cli --device /dev/ttyUSB0 ping"
EOF

echo "🎉 Deployment successful!"
