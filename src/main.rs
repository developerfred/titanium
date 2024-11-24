use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::get, Router};
use base64::{engine::general_purpose::URL_SAFE, Engine};
use dioxus_native::Config;
use serde::Deserialize;
use std::{net::SocketAddr, time::Instant};
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use url::Url;

#[derive(Debug, Deserialize, PartialEq)]
struct RenderParams {
    url: String,
    w: u32,
    h: u32,
}

fn decode_base64_url(encoded: &str) -> Result<String, String> {
    URL_SAFE
        .decode(encoded)
        .map_err(|_| "Invalid base64".to_string())
        .and_then(|bytes| String::from_utf8(bytes).map_err(|_| "Invalid URL encoding".to_string()))
}

fn validate_url(url_str: &str) -> Result<Url, String> {
    Url::parse(url_str).map_err(|_| "Invalid URL".to_string())
}

async fn render_html(url: &Url) -> Result<Vec<u8>, String> {
    let config = Config {
        stylesheets: Vec::new(),
        base_url: Some(url.to_string()),
        ..Default::default()
    };

    let html_content = reqwest::blocking::get(url.as_str())
        .and_then(|response| response.text())
        .map_err(|e| format!("Failed to fetch URL: {}", e))?;

    tokio::task::spawn_blocking(move || {
        dioxus_native::launch_static_html_cfg(&html_content, config)
    })
    .await
    .map_err(|e| format!("Render failed: {}", e))?
}

async fn render_url(Query(params): Query<RenderParams>) -> impl IntoResponse {
    let start = Instant::now();

    let result = decode_base64_url(&params.url)
        .and_then(|decoded| validate_url(&decoded))
        .and_then(|url| {
            info!("Starting render for URL: {}", url);

            let load_start = Instant::now();
            let result = tokio::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(render_html(&url))
            });

            let total_time = start.elapsed();
            info!("Render completed - Total time: {:?}", total_time);

            result
        });

    match result {
        Ok(png_data) => (StatusCode::OK, png_data).into_response(),
        Err(error) => (StatusCode::BAD_REQUEST, error).into_response(),
    }
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .with_max_level(Level::INFO)
        .init();

    let app = Router::new()
        .route("/render.png", get(render_url))
        .route("/health", get(health_check))
        .layer(TraceLayer::new_for_http());

    info!("Starting server on port 3000");

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
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

    #[test]
    fn test_render_params_deserialization() {
        let json = r#"{"url":"aHR0cHM6Ly9nb29nbGUuY29t","w":800,"h":600}"#;
        let params: RenderParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.url, "aHR0cHM6Ly9nb29nbGUuY29t");
        assert_eq!(params.w, 800);
        assert_eq!(params.h, 600);
    }
}
