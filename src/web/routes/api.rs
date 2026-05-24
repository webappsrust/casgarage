//! API routes (JSON responses)
//!
//! RESTful API with scoped endpoints:
//! - /api/v1 - Public API
//! - /api/v1/user - User-scoped API
//! - /api/v1/admin - Admin-scoped API

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};

use crate::web::server::AppState;

/// Main API routes
pub fn routes() -> Router<AppState> {
    Router::new()
        // Public API endpoints
        .route("/health", get(health))
        .route("/health.txt", get(health_txt))
        .route("/info", get(info))
        .route("/info.txt", get(info_txt))

        // User-scoped API
        .nest("/user", user_api_routes())

        // Admin-scoped API
        .nest("/admin", admin_api_routes())

        // Setup/first-time configuration
        .nest("/setup", setup_api_routes())
}

/// User API routes
fn user_api_routes() -> Router<AppState> {
    Router::new()
        .route("/profile", get(get_user_profile))
        .route("/buckets", get(list_user_buckets))
        .route("/keys", get(list_user_keys))
        .route("/usage", get(get_user_usage))
}

/// Admin API routes
fn admin_api_routes() -> Router<AppState> {
    Router::new()
        // Server configuration
        .route("/config", get(get_server_config).put(update_server_config))

        // User management
        .route("/users", get(list_all_users).post(create_admin_user))
        .route("/users/:id", get(get_user).delete(delete_user))

        // API key management
        .route("/keys", get(list_all_keys).post(create_api_key))
        .route("/keys/:id", get(get_key).delete(revoke_key))

        // Bucket management
        .route("/buckets", get(list_all_buckets).post(create_bucket))
        .route("/buckets/:name", get(get_bucket).delete(delete_bucket))

        // Cluster management
        .route("/cluster/status", get(cluster_status))
        .route("/cluster/nodes", get(list_nodes).post(add_node))
        .route("/cluster/nodes/:id", get(get_node).delete(remove_node))

        // Metrics
        .route("/metrics", get(get_metrics))
        .route("/metrics/prometheus", get(prometheus_metrics))

        // Logs
        .route("/logs", get(query_logs))

        // Scheduler
        .route("/scheduler/tasks", get(list_scheduled_tasks).post(create_scheduled_task))
}

/// Setup API routes
fn setup_api_routes() -> Router<AppState> {
    Router::new()
        .route("/check", get(setup_check_placeholder))
}

async fn setup_check_placeholder() -> impl IntoResponse {
    Json(serde_json::json!({"setup_required": false}))
}

// ============================================================================
// Public API Handlers
// ============================================================================

async fn health() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn health_txt() -> impl IntoResponse {
    (StatusCode::OK, [("Content-Type", "text/plain")], "OK")
}

async fn info(State(state): State<AppState>) -> impl IntoResponse {
    let db_mode = state.db.mode().await;
    let is_read_only = state.db.is_read_only().await;

    Json(serde_json::json!({
        "name": env!("CARGO_PKG_NAME"),
        "version": env!("CARGO_PKG_VERSION"),
        "description": env!("CARGO_PKG_DESCRIPTION"),
        "database_mode": format!("{:?}", db_mode),
        "read_only": is_read_only,
        "endpoints": {
            "api": "/api/v1",
            "health": "/api/v1/health",
            "info": "/api/v1/info"
        }
    }))
}

async fn info_txt(State(state): State<AppState>) -> impl IntoResponse {
    let text = format!(
        "CasGarage v{}\nS3-Compatible Object Storage\nDatabase: {:?}",
        env!("CARGO_PKG_VERSION"),
        state.db.mode().await
    );
    (StatusCode::OK, [("Content-Type", "text/plain")], text)
}

// ============================================================================
// User API Handlers (Placeholders)
// ============================================================================

async fn get_user_profile() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "not_implemented" }))
}

async fn list_user_buckets() -> impl IntoResponse {
    Json(serde_json::json!({ "buckets": [] }))
}

async fn list_user_keys() -> impl IntoResponse {
    Json(serde_json::json!({ "keys": [] }))
}

async fn get_user_usage() -> impl IntoResponse {
    Json(serde_json::json!({ "usage": { "bytes": 0, "objects": 0 } }))
}

// ============================================================================
// Admin API Handlers (Placeholders)
// ============================================================================

async fn get_server_config() -> impl IntoResponse {
    Json(serde_json::json!({ "config": {} }))
}

async fn update_server_config() -> impl IntoResponse {
    Json(serde_json::json!({ "success": true }))
}

async fn list_all_users() -> impl IntoResponse {
    Json(serde_json::json!({ "users": [] }))
}

async fn create_admin_user() -> impl IntoResponse {
    Json(serde_json::json!({ "success": false, "error": "not_implemented" }))
}

async fn get_user() -> impl IntoResponse {
    Json(serde_json::json!({ "user": null }))
}

async fn delete_user() -> impl IntoResponse {
    Json(serde_json::json!({ "success": false }))
}

async fn list_all_keys() -> impl IntoResponse {
    Json(serde_json::json!({ "keys": [] }))
}

async fn create_api_key() -> impl IntoResponse {
    Json(serde_json::json!({ "success": false }))
}

async fn get_key() -> impl IntoResponse {
    Json(serde_json::json!({ "key": null }))
}

async fn revoke_key() -> impl IntoResponse {
    Json(serde_json::json!({ "success": false }))
}

async fn list_all_buckets() -> impl IntoResponse {
    Json(serde_json::json!({ "buckets": [] }))
}

async fn create_bucket() -> impl IntoResponse {
    Json(serde_json::json!({ "success": false }))
}

async fn get_bucket() -> impl IntoResponse {
    Json(serde_json::json!({ "bucket": null }))
}

async fn delete_bucket() -> impl IntoResponse {
    Json(serde_json::json!({ "success": false }))
}

async fn cluster_status() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "single_node", "nodes": 1 }))
}

async fn list_nodes() -> impl IntoResponse {
    Json(serde_json::json!({ "nodes": [] }))
}

async fn add_node() -> impl IntoResponse {
    Json(serde_json::json!({ "success": false }))
}

async fn get_node() -> impl IntoResponse {
    Json(serde_json::json!({ "node": null }))
}

async fn remove_node() -> impl IntoResponse {
    Json(serde_json::json!({ "success": false }))
}

async fn get_metrics() -> impl IntoResponse {
    Json(serde_json::json!({ "metrics": {} }))
}

async fn prometheus_metrics() -> impl IntoResponse {
    let metrics = "# TYPE casgarage_up gauge\ncasgarage_up 1\n";
    (StatusCode::OK, [("Content-Type", "text/plain")], metrics)
}

async fn query_logs() -> impl IntoResponse {
    Json(serde_json::json!({ "logs": [] }))
}

async fn list_scheduled_tasks() -> impl IntoResponse {
    Json(serde_json::json!({ "tasks": [] }))
}

async fn create_scheduled_task() -> impl IntoResponse {
    Json(serde_json::json!({ "success": false }))
}
