# Titanium - URL to PNG Service 🖼️

Titanium is a high-performance web service that converts web pages to PNG images using Rust and Dioxus. It provides a simple HTTP API for rendering web pages as screenshots.

## Features 🚀

- Fast and efficient web page rendering
- Base64 URL encoding for safe URL transmission
- Configurable image dimensions
- Health check endpoint
- Docker support
- Comprehensive test suite

## Prerequisites 📋

Before running the service, ensure you have the following installed:

- Rust (nightly toolchain)
- Docker (optional)
- The following system dependencies:
  - pkg-config
  - libssl-dev
  - libgtk-3-dev
  - libwebkit2gtk-4.0-dev
  - libayatana-appindicator3-dev
  - librsvg2-dev
  - cmake

## Installation 🛠️

### Using Docker

```bash
docker-compose up -d
```

### Manual Installation

1. Clone the repository:
```bash
git clone https://github.com/developerfred/titanium.git
cd titanium
```

2. Install dependencies:
```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install pkg-config libssl-dev libgtk-3-dev libwebkit2gtk-4.0-dev \
    libayatana-appindicator3-dev librsvg2-dev cmake
```

3. Build and run:
```bash
cargo build --release
./target/release/titanium
```

## Usage 💡

The service exposes two main endpoints:

### 1. Render URL to PNG
```
GET /render.png?url={base64_encoded_url}&w={width}&h={height}
```

Parameters:
- `url`: Base64-encoded URL of the webpage to render
- `w`: Width of the output image
- `h`: Height of the output image

Example:
```bash
# Convert https://example.com to base64 first
curl "http://localhost:3000/render.png?url=aHR0cHM6Ly9leGFtcGxlLmNvbQ==&w=800&h=600" > screenshot.png
```

### 2. Health Check
```
GET /health
```

Returns "OK" if the service is running properly.

## Development 🔧

### Running Tests

Using Docker:
```bash
./test.sh
```

Manually:
```bash
cargo test --all-features
```

### Running Locally

```bash
cargo run
```

The service will start on `http://localhost:3000`

## Contributing 🤝# Titanium - URL to PNG Service 🖼️

Titanium is a high-performance web service that converts web pages to PNG images using Rust and Dioxus. It provides a simple HTTP API for rendering web pages as screenshots.

## Features 🚀

- Fast and efficient web page rendering
- Base64 URL encoding for safe URL transmission
- Configurable image dimensions
- Health check endpoint
- Docker support
- Comprehensive test suite

## Prerequisites 📋

Before running the service, ensure you have the following installed:

- Rust (nightly toolchain)
- Docker (optional)
- The following system dependencies:
  - pkg-config
  - libssl-dev
  - libgtk-3-dev
  - libwebkit2gtk-4.0-dev
  - libayatana-appindicator3-dev
  - librsvg2-dev
  - cmake

## Installation 🛠️

### Using Docker

```bash
docker-compose up -d
```

### Manual Installation

1. Clone the repository:
```bash
git clone https://github.com/developerfred/titanium.git
cd titanium
```

2. Install dependencies:
```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install pkg-config libssl-dev libgtk-3-dev libwebkit2gtk-4.0-dev \
    libayatana-appindicator3-dev librsvg2-dev cmake
```

3. Build and run:
```bash
cargo build --release
./target/release/titanium
```

## Usage 💡

The service exposes two main endpoints:

### 1. Render URL to PNG
```
GET /render.png?url={base64_encoded_url}&w={width}&h={height}
```

Parameters:
- `url`: Base64-encoded URL of the webpage to render
- `w`: Width of the output image
- `h`: Height of the output image

Example:
```bash
# Convert https://example.com to base64 first
curl "http://localhost:3000/render.png?url=aHR0cHM6Ly9leGFtcGxlLmNvbQ==&w=800&h=600" > screenshot.png
```

### 2. Health Check
```
GET /health
```

Returns "OK" if the service is running properly.

## Development 🔧

### Running Tests

Using Docker:
```bash
./test.sh
```

Manually:
```bash
cargo test --all-features
```

### Running Locally

```bash
cargo run
```

The service will start on `http://localhost:3000`

## Contributing 🤝

1. Fork the repository
2. Create your feature branch (`git checkout -b feat/TitaniumFeature`)
3. Run the tests (`cargo test`)
4. Commit your changes (`git commit -m 'Add some TitaniumFeature'`)
5. Push to the branch (`git push origin feature/TitaniumFeature`)
6. Open a Pull Request


