use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Router};
use base64::{engine::general_purpose::URL_SAFE, Engine};
use dioxus_native::Config;
use serde::Deserialize;
use std::{net::SocketAddr, time::Instant};
use tokio::task::block_in_place;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info, warn, Level};
use tracing_subscriber::{
    fmt::format::FmtSpan,
    prelude::*,
    EnvFilter,
};
use url::Url;

#[derive(Debug, Deserialize, PartialEq)]
struct RenderParams {
    url: String,
    w: u32,
    h: u32,
}

fn decode_base64_url(encoded: &str) -> Result<String, String> {
    debug!("Attempting to decode base64 URL: {}", encoded);
    URL_SAFE
        .decode(encoded)
        .map_err(|e| {
            error!("Base64 decoding error: {}", e);
            "Invalid base64".to_string()
        })
        .and_then(|bytes| {
            String::from_utf8(bytes).map_err(|e| {
                error!("UTF-8 decoding error: {}", e);
                "Invalid URL encoding".to_string()
            })
        })
}

fn validate_url(url_str: &str) -> Result<Url, String> {
    debug!("Validating URL: {}", url_str);
    Url::parse(url_str).map_err(|e| {
        error!("URL parsing error: {}", e);
        "Invalid URL".to_string()
    })
}

async fn render_html(url: &Url, width: u32, height: u32) -> Result<Vec<u8>, String> {
    info!("Starting HTML render for URL: {} ({}x{})", url, width, height);
    let start = Instant::now();

    let html_content = reqwest::blocking::get(url.as_str())
        .and_then(|response| {
            debug!("Received response from URL: {:?}", response.status());
            response.text()
        })
        .map_err(|e| {
            error!("Failed to fetch URL {}: {}", url, e);
            format!("Failed to fetch URL: {}", e)
        })?;

    let config = Config {
        stylesheets: Vec::new(),
        base_url: Some(url.to_string()),
    };

    let result = tokio::task::spawn_blocking(move || {
        debug!("Launching Dioxus renderer");
        dioxus_native::launch_static_html_cfg(&html_content, config);
        Ok(Vec::new())
    })
    .await
    .map_err(|e| {
        error!("Task execution failed: {}", e);
        format!("Task failed: {}", e)
    })?;

    let duration = start.elapsed();
    info!("Render completed in {:?}", duration);
    Ok(result)
}

async fn render_url(Query(params): Query<RenderParams>) -> impl IntoResponse {
    let start = Instant::now();
    info!("Received render request with params: {:?}", params);

    let result = decode_base64_url(&params.url)
        .and_then(|decoded| {
            debug!("Decoded URL: {}", decoded);
            validate_url(&decoded)
        })
        .and_then(|url| {
            info!("Starting render for URL: {}", url);
            
            let result = block_in_place(|| {
                tokio::runtime::Handle::current().block_on(render_html(&url, params.w, params.h))
            });

            let total_time = start.elapsed();
            info!("Render completed - Total time: {:?}", total_time);

            result
        });

    match result {
        Ok(png_data) => {
            info!("Successfully rendered PNG ({} bytes)", png_data.len());
            (StatusCode::OK, png_data).into_response()
        }
        Err(error) => {
            warn!("Render request failed: {}", error);
            (StatusCode::BAD_REQUEST, error).into_response()
        }
    }
}

async fn health_check() -> impl IntoResponse {
    debug!("Health check requested");
    (StatusCode::OK, "OK")
}

#[tokio::main]
async fn main() {
    // Initialize logging with detailed configuration
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("debug"))
        )
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::FULL)
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
        .init();

    info!("Initializing Titanium service");

    let app = Router::new()
        .route("/render.png", get(render_url))
        .route("/health", get(health_check))
        .layer(TraceLayer::new_for_http());

    info!("Starting server on port 3000");

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            info!("Server listening on http://{}", addr);
            if let Err(e) = axum::serve(listener, app).await {
                error!("Server error: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            error!("Failed to bind to address {}: {}", addr, e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_base64_url_valid() {
        let encoded = "aHR0cHM6Ly9nb29nbGUuY29t";
        let result = decode_base64_url(encoded);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "https://google.com");
    }

    #[test]
    fn test_decode_base64_url_invalid() {
        let encoded = "invalid@@base64";
        let result = decode_base64_url(encoded);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_url_valid() {
        let url = "https://google.com";
        let result = validate_url(url);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_url_invalid() {
        let url = "not a url";
        let result = validate_url(url);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }
}