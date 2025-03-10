FROM rustlang/rust:nightly AS builder

RUN apt-get update && \
    apt-get install -y \
        pkg-config \
        libssl-dev \
        libgtk-3-dev \
        libwebkit2gtk-4.0-dev \
        libayatana-appindicator3-dev \
        librsvg2-dev \
        cmake \
        libxdo-dev && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock* ./

COPY src src/

ARG RUST_LOG=info
ENV RUST_LOG=${RUST_LOG}

RUN echo "Building application with RUST_LOG=${RUST_LOG}" && \
    cargo build --release && \
    cargo test --no-run

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y \
        bash \
        ca-certificates \
        libwebkit2gtk-4.0-37 \
        libjavascriptcoregtk-4.0-18 \
        libsoup2.4-1 \
        gstreamer1.0-plugins-base \
        gstreamer1.0-plugins-good \
        gstreamer1.0-x \
        libgtk-3-0 \
        curl \
        librsvg2-2 \
        libxdo3 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/titanium /usr/local/bin/

ENV RUST_LOG=info

COPY <<'EOF' /usr/local/bin/start.sh
#!/bin/bash
echo "Starting Titanium server with RUST_LOG=${RUST_LOG}"
exec titanium
EOF

RUN chmod +x /usr/local/bin/start.sh

EXPOSE 3000

CMD ["/usr/local/bin/start.sh"]