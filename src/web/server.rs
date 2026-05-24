//! Web server implementation
//!
//! Provides the main HTTP/HTTPS server with graceful shutdown and signal handling

use anyhow::{Context, Result};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, TraceLayer},
    compression::CompressionLayer,
};
use tracing::{error, info, warn};

use crate::config::Config;
use crate::db::Database;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: Database,
}

/// Start the web server
pub async fn start(config: Config, db: Database) -> Result<()> {
    info!("🚀 Starting web server...");

    let state = AppState {
        config: Arc::new(config.clone()),
        db: db.clone(),
    };

    // Build application with routes
    let app = build_app(state);

    // Get bind address
    let bind_addr = config.http_bind_address();

    info!("🌐 Binding to {}", bind_addr);

    // Create TCP listener
    let listener = TcpListener::bind(bind_addr)
        .await
        .context("Failed to bind server")?;

    info!("✓ Server listening on {}", config.display_url());
    info!("");
    info!("Press Ctrl+C to stop");
    info!("");

    // Start server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Server error")?;

    info!("👋 Server stopped gracefully");

    Ok(())
}

/// Build the application router
fn build_app(state: AppState) -> Router {
    Router::new()
        // Root route
        .route("/", get(root_handler))

        // Health check
        .route("/health", get(health_handler))
        .route("/healthz", get(health_handler))
        .route("/readyz", get(readiness_handler))

        // Setup/first user routes
        .merge(crate::auth::first_user::routes())

        // API routes
        .nest("/api/v1", api_routes())

        // Fallback for 404
        .fallback(not_found_handler)

        // Middleware
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())

        // Add state
        .with_state(state)
}

/// API routes
fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(api_health_handler))
        .route("/info", get(api_info_handler))
        // More routes will be added as we implement them
}

/// Root handler
async fn root_handler(State(_state): State<AppState>) -> impl IntoResponse {
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CasGarage - S3-Compatible Object Storage</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            max-width: 800px;
            margin: 50px auto;
            padding: 20px;
            background: #f5f5f5;
        }}
        .container {{
            background: white;
            padding: 40px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }}
        h1 {{ color: #333; margin-bottom: 10px; }}
        .version {{ color: #666; font-size: 14px; }}
        .status {{ color: #27ae60; margin: 20px 0; }}
        .links {{ margin-top: 30px; }}
        .links a {{
            display: inline-block;
            margin-right: 20px;
            color: #3498db;
            text-decoration: none;
        }}
        .links a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="container">
        <h1>🚗 CasGarage</h1>
        <p class="version">v{} - S3-Compatible Object Storage</p>
        <p class="status">✓ Server is running</p>
        <p>Self-hosted S3-compatible object storage with integrated administrative web UI.</p>
        <div class="links">
            <a href="/api/v1/health">API Health</a>
            <a href="/api/v1/info">Server Info</a>
            <a href="https://casgarage.readthedocs.io" target="_blank">Documentation</a>
            <a href="https://github.com/casapps/casgarage" target="_blank">GitHub</a>
        </div>
    </div>
</body>
</html>"#,
        env!("CARGO_PKG_VERSION")
    );

    (StatusCode::OK, [("Content-Type", "text/html")], html)
}

/// Health check handler (simple)
async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

/// Readiness handler (checks if server can accept requests)
async fn readiness_handler(State(state): State<AppState>) -> impl IntoResponse {
    // Check database
    match state.db.health_check().await {
        Ok(health) if health.is_healthy() => {
            (StatusCode::OK, "ready")
        }
        Ok(_) => {
            (StatusCode::SERVICE_UNAVAILABLE, "not ready - database unhealthy")
        }
        Err(e) => {
            error!("Health check error: {}", e);
            (StatusCode::SERVICE_UNAVAILABLE, "not ready - health check failed")
        }
    }
}

/// API health check handler (detailed)
async fn api_health_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.db.health_check().await {
        Ok(health) => {
            let response = HealthResponse {
                status: if health.is_healthy() { "healthy".to_string() } else { "degraded".to_string() },
                version: env!("CARGO_PKG_VERSION").to_string(),
                database: DatabaseStatus {
                    mode: format!("{:?}", health.mode),
                    sqlite_ok: health.sqlite_ok,
                    external_ok: health.external_ok,
                },
                disk_space_mb: health.disk_space_mb,
            };

            let status = if health.is_healthy() {
                StatusCode::OK
            } else {
                StatusCode::SERVICE_UNAVAILABLE
            };

            (status, Json(response))
        }
        Err(e) => {
            error!("Health check failed: {}", e);
            let response = HealthResponse {
                status: "error".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                database: DatabaseStatus {
                    mode: "unknown".to_string(),
                    sqlite_ok: false,
                    external_ok: false,
                },
                disk_space_mb: 0,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

/// Server info handler
async fn api_info_handler(State(state): State<AppState>) -> impl IntoResponse {
    let _db_stats = state.db.stats().await;
    let db_mode = state.db.mode().await;
    let is_read_only = state.db.is_read_only().await;

    let info = ServerInfo {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        description: env!("CARGO_PKG_DESCRIPTION").to_string(),
        uptime_seconds: 0, // TODO: Track uptime
        database_mode: format!("{:?}", db_mode),
        read_only: is_read_only,
        endpoints: EndpointInfo {
            api: "/api/v1".to_string(),
            health: "/api/v1/health".to_string(),
            info: "/api/v1/info".to_string(),
        },
    };

    (StatusCode::OK, Json(info))
}

/// Not found handler
async fn not_found_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}

/// Health check response
#[derive(Debug, Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    version: String,
    database: DatabaseStatus,
    disk_space_mb: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct DatabaseStatus {
    mode: String,
    sqlite_ok: bool,
    external_ok: bool,
}

/// Server info response
#[derive(Debug, Serialize, Deserialize)]
struct ServerInfo {
    name: String,
    version: String,
    description: String,
    uptime_seconds: u64,
    database_mode: String,
    read_only: bool,
    endpoints: EndpointInfo,
}

#[derive(Debug, Serialize, Deserialize)]
struct EndpointInfo {
    api: String,
    health: String,
    info: String,
}

/// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C signal");
        }
        _ = terminate => {
            info!("Received SIGTERM signal");
        }
    }

    info!("🛑 Initiating graceful shutdown...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_response_serialization() {
        let response = HealthResponse {
            status: "healthy".to_string(),
            version: "0.1.0".to_string(),
            database: DatabaseStatus {
                mode: "Normal".to_string(),
                sqlite_ok: true,
                external_ok: false,
            },
            disk_space_mb: 1000,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("healthy"));
    }
}
