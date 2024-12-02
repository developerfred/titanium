# Builder stage
FROM rustlang/rust:nightly AS builder

ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH=/usr/local/cargo/bin:$PATH
ENV RUST_LOG=debug
ENV RUST_BACKTRACE=full
ENV RUST_LIB_BACKTRACE=1

RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libgdk3.0-cil \
    libgdk-pixbuf2.0-dev \
    libglib2.0-dev \
    cmake \
    libxdo-dev \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Create logs directory
RUN mkdir -p /app/logs

COPY Cargo.toml Cargo.lock* ./

RUN mkdir src && \
    echo 'fn main() { println!("dummy") }' > src/main.rs && \
    cargo build --release && \
    cargo build --tests && \
    rm -rf src target/release/deps/titanium*

COPY src src/
COPY rust-toolchain.toml .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release && \
    cargo test --no-run && \
    cp target/release/titanium /app/titanium

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    libgtk-3-0 \
    libwebkit2gtk-4.0-37 \
    libayatana-appindicator3-1 \
    librsvg2-2 \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-x \
    libgdk3.0-cil \
    libgdk-pixbuf2.0-0 \
    curl \
    libxdo3 \
    && rm -rf /var/lib/apt/lists/*

# Create logs directory and set permissions
RUN mkdir -p /app/logs && \
    chown -R nobody:nogroup /app/logs

COPY --from=builder /app/titanium /usr/local/bin/titanium

ENV RUST_LOG=debug
ENV RUST_BACKTRACE=full
ENV RUST_LIB_BACKTRACE=1

# Add logging script
COPY <<'EOF' /usr/local/bin/start.sh
#!/bin/bash
set -e

# Redirect stdout/stderr to both console and file
exec &> >(tee -a "/app/logs/titanium.log")

# Start the application
exec titanium
EOF

RUN chmod +x /usr/local/bin/start.sh

COPY <<'EOF' /usr/local/bin/healthcheck.sh
#!/bin/bash
curl -f http://localhost:3000/health || exit 1
EOF

RUN chmod +x /usr/local/bin/healthcheck.sh

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD ["/usr/local/bin/healthcheck.sh"]

VOLUME ["/app/logs"]
EXPOSE 3000

CMD ["/usr/local/bin/start.sh"]