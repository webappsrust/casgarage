//! Routing system with scoped routes
//!
//! Route structure:
//! - / - Public routes (HTML)
//! - /user - User-specific routes (HTML)
//! - /admin - Admin routes (HTML)
//! - /api/v1 - Public API (JSON)
//! - /api/v1/user - User API (JSON)
//! - /api/v1/admin - Admin API (JSON)
//!
//! All routes support .txt extension for plain text responses

pub mod admin;
pub mod api;
pub mod public;
pub mod user;

use axum::Router;

use crate::web::server::AppState;

/// Build all application routes
pub fn build_routes() -> Router<AppState> {
    Router::new()
        // Public routes (HTML)
        .merge(public::routes())

        // User routes (HTML)
        .nest("/user", user::html_routes())

        // Admin routes (HTML)
        .nest("/admin", admin::html_routes())

        // API routes (JSON)
        .nest("/api/v1", api::routes())
}
