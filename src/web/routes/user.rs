//! User routes (HTML responses)
//!
//! User-specific web pages and functionality

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::web::server::AppState;

/// User HTML routes
pub fn html_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(user_dashboard))
        .route("/dashboard", get(user_dashboard))
        .route("/profile", get(user_profile))
        .route("/keys", get(user_keys))
        .route("/buckets", get(user_buckets))
}

/// User dashboard
async fn user_dashboard() -> impl IntoResponse {
    Html("<h1>My Dashboard</h1><p>Coming soon: User dashboard with usage stats</p>")
}

/// User profile page
async fn user_profile() -> impl IntoResponse {
    Html("<h1>My Profile</h1><p>Coming soon: Profile management</p>")
}

/// User API keys
async fn user_keys() -> impl IntoResponse {
    Html("<h1>My API Tokens</h1><p>Coming soon: Manage your API tokens</p>")
}

/// User buckets
async fn user_buckets() -> impl IntoResponse {
    Html("<h1>My Buckets</h1><p>Coming soon: Your S3 buckets</p>")
}
