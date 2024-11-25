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


RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    cargo build --tests && \
    rm -rf src


COPY src src/


RUN cargo build --release && \
    cargo test --no-run


FROM debian:bookworm-slim


RUN apt-get update && \
    apt-get install -y \
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

EXPOSE 3000

CMD ["titanium"]