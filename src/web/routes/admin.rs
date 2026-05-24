//! Admin routes (HTML responses)
//!
//! All admin-related web pages

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::web::server::AppState;

/// Admin HTML routes
pub fn html_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_dashboard))
        .route("/dashboard", get(admin_dashboard))
        .route("/buckets", get(buckets_page))
        .route("/keys", get(keys_page))
        .route("/cluster", get(cluster_page))
        .route("/settings", get(settings_page))
        .route("/logs", get(logs_page))
        .route("/server/setup", get(server_setup_page))
}

/// Admin dashboard
async fn admin_dashboard() -> impl IntoResponse {
    Html("<h1>Admin Dashboard</h1><p>Coming soon: Real-time metrics and monitoring</p>")
}

/// Buckets management page
async fn buckets_page() -> impl IntoResponse {
    Html("<h1>Buckets</h1><p>Coming soon: Bucket management interface</p>")
}

/// API keys management page
async fn keys_page() -> impl IntoResponse {
    Html("<h1>API Tokens</h1><p>Coming soon: API token management</p>")
}

/// Cluster management page
async fn cluster_page() -> impl IntoResponse {
    Html("<h1>Cluster Status</h1><p>Coming soon: Cluster monitoring and management</p>")
}

/// Settings page
async fn settings_page() -> impl IntoResponse {
    Html("<h1>Server Settings</h1><p>Coming soon: Server configuration interface</p>")
}

/// Logs viewer page
async fn logs_page() -> impl IntoResponse {
    Html("<h1>System Logs</h1><p>Coming soon: Log viewer and search</p>")
}

/// Server setup wizard page
async fn server_setup_page() -> impl IntoResponse {
    Html("<h1>Server Setup Wizard</h1><p>Coming soon: Comprehensive server setup</p>")
}
