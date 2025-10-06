#!/bin/bash
# E-ink Power CLI - Development Helper Script
# Copyright (c) 2025 Dynamic Devices Ltd
# All rights reserved.

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Project info
PROJECT_NAME="eink-power-cli"
DOCKER_IMAGE="$PROJECT_NAME-dev"

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

show_help() {
    cat << EOF
E-ink Power CLI - Development Helper Script

Usage: $0 [COMMAND] [OPTIONS]

COMMANDS:
    setup           Set up development environment
    build           Build the project (native)
    build-arm64     Build for ARM64 target
    build-all       Build for all targets
    test            Run tests
    test-ci         Run CI-style tests
    lint            Run linting (clippy + fmt)
    clean           Clean build artifacts
    docker-build    Build Docker development image
    docker-dev      Start development container
    docker-serial   Start development container with serial access
    docker-ci       Run CI pipeline in container
    docker-docs     Start documentation server
    deploy          Deploy to target device
    monitor         Monitor serial output
    release         Create release build and artifacts
    help            Show this help message

OPTIONS:
    --target TARGET     Specify build target (default: aarch64-unknown-linux-gnu)
    --device DEVICE     Specify serial device (default: /dev/ttyUSB0)
    --host HOST         Specify deployment host (default: fio@62.3.79.162)
    --port PORT         Specify SSH port (default: 25)
    --verbose           Enable verbose output
    --quiet             Suppress non-essential output

EXAMPLES:
    $0 setup                    # Set up development environment
    $0 build-arm64              # Build for ARM64
    $0 test                     # Run all tests
    $0 docker-dev               # Start development container
    $0 deploy --host user@host  # Deploy to custom host
    $0 monitor --device /dev/ttyACM0  # Monitor different serial device

EOF
}

# Default values
TARGET="aarch64-unknown-linux-gnu"
DEVICE="/dev/ttyUSB0"
HOST="fio@62.3.79.162"
PORT="25"
VERBOSE=false
QUIET=false

# Parse command line arguments
COMMAND=""
while [[ $# -gt 0 ]]; do
    case $1 in
        --target)
            TARGET="$2"
            shift 2
            ;;
        --device)
            DEVICE="$2"
            shift 2
            ;;
        --host)
            HOST="$2"
            shift 2
            ;;
        --port)
            PORT="$2"
            shift 2
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --quiet)
            QUIET=true
            shift
            ;;
        --help|-h)
            show_help
            exit 0
            ;;
        *)
            if [[ -z "$COMMAND" ]]; then
                COMMAND="$1"
            else
                log_error "Unknown option: $1"
                exit 1
            fi
            shift
            ;;
    esac
done

# Set verbose flags
if [[ "$VERBOSE" == "true" ]]; then
    set -x
    CARGO_VERBOSE="--verbose"
else
    CARGO_VERBOSE=""
fi

if [[ "$QUIET" == "true" ]]; then
    CARGO_QUIET="--quiet"
else
    CARGO_QUIET=""
fi

# Command implementations
setup() {
    log_info "Setting up development environment..."
    
    # Check for required tools
    command -v docker >/dev/null 2>&1 || { log_error "Docker is required but not installed."; exit 1; }
    command -v docker-compose >/dev/null 2>&1 || { log_error "Docker Compose is required but not installed."; exit 1; }
    
    # Build Docker image
    log_info "Building Docker development image..."
    docker build -t "$DOCKER_IMAGE" .
    
    # Create cache volumes
    docker volume create "${PROJECT_NAME}-cargo-cache" >/dev/null 2>&1 || true
    docker volume create "${PROJECT_NAME}-target-cache" >/dev/null 2>&1 || true
    
    log_success "Development environment setup complete!"
    log_info "Use '$0 docker-dev' to start the development container"
}

build_native() {
    log_info "Building native binary..."
    cargo build --release $CARGO_VERBOSE $CARGO_QUIET
    log_success "Native build complete: target/release/$PROJECT_NAME"
}

build_arm64() {
    log_info "Building ARM64 binary..."
    cargo build --release --target "$TARGET" $CARGO_VERBOSE $CARGO_QUIET
    log_success "ARM64 build complete: target/$TARGET/release/$PROJECT_NAME"
}

build_all() {
    log_info "Building for all targets..."
    
    targets=("x86_64-unknown-linux-gnu" "aarch64-unknown-linux-gnu" "aarch64-unknown-linux-musl")
    
    for target in "${targets[@]}"; do
        log_info "Building for $target..."
        rustup target add "$target" >/dev/null 2>&1 || true
        cargo build --release --target "$target" $CARGO_VERBOSE $CARGO_QUIET
    done
    
    log_success "All targets built successfully!"
}

run_tests() {
    log_info "Running tests..."
    cargo test --all-features $CARGO_VERBOSE $CARGO_QUIET
    log_success "All tests passed!"
}

run_ci_tests() {
    log_info "Running CI-style tests..."
    
    log_info "1. Checking code formatting..."
    cargo fmt --all -- --check
    
    log_info "2. Running Clippy lints..."
    cargo clippy --all-targets --all-features -- -D warnings
    
    log_info "3. Running tests..."
    cargo test --verbose --all-features
    
    log_info "4. Building release binary..."
    cargo build --release --target "$TARGET"
    
    log_info "5. Running security audit..."
    cargo audit || log_warning "Security audit found issues (check output above)"
    
    log_success "CI pipeline completed successfully!"
}

run_lint() {
    log_info "Running linting..."
    
    log_info "Checking code formatting..."
    cargo fmt --all -- --check
    
    log_info "Running Clippy..."
    cargo clippy --all-targets --all-features -- -D warnings
    
    log_success "Linting complete!"
}

clean_build() {
    log_info "Cleaning build artifacts..."
    cargo clean
    log_success "Build artifacts cleaned!"
}

docker_build() {
    log_info "Building Docker image..."
    docker build -t "$DOCKER_IMAGE" .
    log_success "Docker image built: $DOCKER_IMAGE"
}

docker_dev() {
    log_info "Starting development container..."
    docker-compose up dev
}

docker_serial() {
    log_info "Starting development container with serial access..."
    log_warning "Make sure $DEVICE exists and you have permission to access it"
    docker-compose up dev-serial
}

docker_ci() {
    log_info "Running CI pipeline in container..."
    docker-compose up ci
}

docker_docs() {
    log_info "Starting documentation server..."
    log_info "Documentation will be available at http://localhost:8000"
    docker-compose up docs
}

deploy() {
    log_info "Deploying to target device..."
    
    # Build for target
    build_arm64
    
    # Deploy using existing script
    if [[ -f "./deploy-target.sh" ]]; then
        ./deploy-target.sh
    else
        log_error "deploy-target.sh script not found"
        exit 1
    fi
    
    log_success "Deployment complete!"
}

monitor_serial() {
    log_info "Monitoring serial output on $DEVICE..."
    
    if [[ ! -e "$DEVICE" ]]; then
        log_error "Serial device $DEVICE not found"
        exit 1
    fi
    
    log_info "Press Ctrl+C to exit monitoring"
    log_info "Device: $DEVICE, Baud: 115200"
    
    # Use minicom if available, otherwise try screen
    if command -v minicom >/dev/null 2>&1; then
        minicom -D "$DEVICE" -b 115200
    elif command -v screen >/dev/null 2>&1; then
        screen "$DEVICE" 115200
    else
        log_error "Neither minicom nor screen found. Install one of them for serial monitoring."
        exit 1
    fi
}

create_release() {
    log_info "Creating release build and artifacts..."
    
    # Build all targets
    build_all
    
    # Create release directory
    RELEASE_DIR="release-$(date +%Y%m%d-%H%M%S)"
    mkdir -p "$RELEASE_DIR"
    
    # Copy binaries
    targets=("x86_64-unknown-linux-gnu" "aarch64-unknown-linux-gnu" "aarch64-unknown-linux-musl")
    
    for target in "${targets[@]}"; do
        if [[ -f "target/$target/release/$PROJECT_NAME" ]]; then
            cp "target/$target/release/$PROJECT_NAME" "$RELEASE_DIR/$PROJECT_NAME-$target"
            
            # Create checksums
            cd "$RELEASE_DIR"
            sha256sum "$PROJECT_NAME-$target" > "$PROJECT_NAME-$target.sha256"
            md5sum "$PROJECT_NAME-$target" > "$PROJECT_NAME-$target.md5"
            cd ..
        fi
    done
    
    # Create release info
    cat > "$RELEASE_DIR/RELEASE_INFO.txt" << EOF
E-ink Power CLI Release
=======================
Build Date: $(date)
Git Commit: $(git rev-parse HEAD 2>/dev/null || echo "unknown")
Git Branch: $(git branch --show-current 2>/dev/null || echo "unknown")
Rust Version: $(rustc --version)

Binaries:
- $PROJECT_NAME-x86_64-unknown-linux-gnu (Linux x86_64)
- $PROJECT_NAME-aarch64-unknown-linux-gnu (Linux ARM64)
- $PROJECT_NAME-aarch64-unknown-linux-musl (Linux ARM64 - static)

Checksums:
- SHA256: *.sha256 files
- MD5: *.md5 files
EOF
    
    log_success "Release artifacts created in $RELEASE_DIR/"
}

# Main command dispatcher
case "$COMMAND" in
    setup)
        setup
        ;;
    build)
        build_native
        ;;
    build-arm64)
        build_arm64
        ;;
    build-all)
        build_all
        ;;
    test)
        run_tests
        ;;
    test-ci)
        run_ci_tests
        ;;
    lint)
        run_lint
        ;;
    clean)
        clean_build
        ;;
    docker-build)
        docker_build
        ;;
    docker-dev)
        docker_dev
        ;;
    docker-serial)
        docker_serial
        ;;
    docker-ci)
        docker_ci
        ;;
    docker-docs)
        docker_docs
        ;;
    deploy)
        deploy
        ;;
    monitor)
        monitor_serial
        ;;
    release)
        create_release
        ;;
    help|"")
        show_help
        ;;
    *)
        log_error "Unknown command: $COMMAND"
        log_info "Use '$0 help' to see available commands"
        exit 1
        ;;
esac
