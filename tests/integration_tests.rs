use axum::http::{StatusCode, Request};
use tower::ServiceExt;
use base64::{engine::general_purpose::URL_SAFE, Engine};
use std::str::from_utf8;

#[tokio::test]
async fn test_health_endpoint() {
    let app = url_to_png_service::create_app();

    let response = app
        .oneshot(Request::builder().uri("/health").body(()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_render_endpoint_invalid_url() {
    let app = url_to_png_service::create_app();
    
    let invalid_url = "not_valid_base64";
    
    let response = app
        .oneshot(
            Request::builder()
                .uri(&format!("/render.png?url={}&w=800&h=600", invalid_url))
                .body(())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_render_endpoint_valid_url() {
    let app = url_to_png_service::create_app();
    
    let url = "https://example.com";
    let encoded_url = URL_SAFE.encode(url.as_bytes());
    
    let response = app
        .oneshot(
            Request::builder()
                .uri(&format!("/render.png?url={}&w=800&h=600", encoded_url))
                .body(())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}