# E-ink Power CLI - Development Container
# This Dockerfile provides a consistent development environment
# for building and testing the eink-power-cli application

FROM rust:1.81-bullseye

# Metadata
LABEL maintainer="Alex J Lennon <ajlennon@dynamicdevices.co.uk>"
LABEL description="Development container for E-ink Power CLI"
LABEL version="1.0"

# Install system dependencies
RUN apt-get update && apt-get install -y \
    # Build tools
    build-essential \
    pkg-config \
    cmake \
    # Cross-compilation tools
    gcc-aarch64-linux-gnu \
    libc6-dev-arm64-cross \
    # System libraries
    libudev-dev \
    libssl-dev \
    # Development tools
    git \
    curl \
    wget \
    vim \
    # Analysis tools
    cloc \
    valgrind \
    strace \
    # Network tools (for testing serial communication)
    socat \
    minicom \
    # Clean up
    && rm -rf /var/lib/apt/lists/*

# Install Rust components
RUN rustup component add \
    clippy \
    rustfmt \
    rust-src \
    rust-analysis

# Install cross-compilation targets
RUN rustup target add \
    x86_64-unknown-linux-gnu \
    aarch64-unknown-linux-gnu \
    aarch64-unknown-linux-musl

# Install useful Cargo tools
RUN cargo install \
    cargo-edit \
    cargo-outdated \
    cargo-audit \
    cargo-bloat \
    cargo-watch \
    cargo-expand

# Set up cross-compilation environment
ENV CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
ENV CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++
ENV AR_aarch64_unknown_linux_gnu=aarch64-linux-gnu-ar
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc

# Create development user (non-root for security)
RUN useradd -m -s /bin/bash developer && \
    usermod -aG sudo developer && \
    echo "developer ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Set up development workspace
WORKDIR /workspace
RUN chown developer:developer /workspace

# Switch to development user
USER developer

# Set up shell environment
RUN echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc && \
    echo 'export RUST_BACKTRACE=1' >> ~/.bashrc && \
    echo 'export CARGO_TERM_COLOR=always' >> ~/.bashrc

# Create helpful aliases
RUN echo 'alias ll="ls -la"' >> ~/.bashrc && \
    echo 'alias build-native="cargo build --release"' >> ~/.bashrc && \
    echo 'alias build-arm64="cargo build --release --target aarch64-unknown-linux-gnu"' >> ~/.bashrc && \
    echo 'alias test-all="cargo test --all-features"' >> ~/.bashrc && \
    echo 'alias lint="cargo clippy --all-targets --all-features"' >> ~/.bashrc && \
    echo 'alias fmt="cargo fmt --all"' >> ~/.bashrc

# Set default command
CMD ["/bin/bash"]

# Usage instructions (as comments for reference)
# Build the development container:
#   docker build -t eink-power-cli-dev .
#
# Run the development container:
#   docker run -it --rm -v $(pwd):/workspace eink-power-cli-dev
#
# For serial device access (when testing):
#   docker run -it --rm --device=/dev/ttyUSB0 -v $(pwd):/workspace eink-power-cli-dev
#
# Build the project inside container:
#   cargo build --release --target aarch64-unknown-linux-gnu
#
# Run tests:
#   cargo test --all-features
