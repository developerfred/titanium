FROM rustlang/rust:nightly AS builder


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

WORKDIR /usr/src/app


COPY Cargo.toml Cargo.lock* ./


RUN mkdir src && \
    echo "fn main() { println!(\"Initializing build...\"); }" > src/main.rs && \
    cargo build --release && \
    cargo build --tests && \
    rm -rf src


COPY src src/
COPY rust-toolchain.toml .


ARG RUST_LOG=info
ENV RUST_LOG=${RUST_LOG}

RUN echo "Building with RUST_LOG=${RUST_LOG}" && \
    cargo build --release && \
    cargo test --no-run


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


COPY --from=builder /usr/src/app/target/release/titanium /usr/local/bin/


ENV RUST_LOG=info


COPY <<'EOF' /usr/local/bin/healthcheck.sh
#!/bin/bash
curl -f http://localhost:3000/health || exit 1
EOF

RUN chmod +x /usr/local/bin/healthcheck.sh

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD ["/usr/local/bin/healthcheck.sh"]

EXPOSE 3000

CMD ["titanium"]