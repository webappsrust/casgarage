//! First user registration flow
//!
//! Handles the initial setup when no admin users exist

use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use super::{create_user, has_admin_users, UserRole};
use crate::web::server::AppState;

/// First user registration routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/setup/register", get(registration_page))
        .route("/api/v1/setup/register", post(register_first_user))
        .route("/api/v1/setup/check", get(check_setup_status))
}

/// Check if setup is required
async fn check_setup_status(State(state): State<AppState>) -> impl IntoResponse {
    match has_admin_users(state.db.sqlite()).await {
        Ok(has_users) => {
            let response = SetupStatus {
                setup_required: !has_users,
                message: if has_users {
                    "Setup complete".to_string()
                } else {
                    "First user registration required".to_string()
                },
            };
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let response = SetupStatus {
                setup_required: true,
                message: format!("Error checking setup status: {}", e),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

/// Registration page HTML
async fn registration_page(State(state): State<AppState>) -> impl IntoResponse {
    // Check if setup already complete
    match has_admin_users(state.db.sqlite()).await {
        Ok(true) => {
            return Html("<h1>Setup already complete</h1><p>Please <a href=\"/login\">login</a>.</p>".to_string());
        }
        Ok(false) => {}
        Err(_) => {
            return Html("<h1>Error</h1><p>Database error occurred</p>".to_string());
        }
    }

    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CasGarage - First User Registration</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 20px;
        }
        .container {
            background: white;
            border-radius: 12px;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
            padding: 40px;
            max-width: 480px;
            width: 100%;
        }
        h1 {
            color: #333;
            margin-bottom: 10px;
            font-size: 28px;
        }
        .subtitle {
            color: #666;
            margin-bottom: 30px;
            font-size: 14px;
        }
        .form-group {
            margin-bottom: 20px;
        }
        label {
            display: block;
            margin-bottom: 8px;
            color: #333;
            font-weight: 500;
        }
        input {
            width: 100%;
            padding: 12px;
            border: 2px solid #e0e0e0;
            border-radius: 6px;
            font-size: 14px;
            transition: border-color 0.3s;
        }
        input:focus {
            outline: none;
            border-color: #667eea;
        }
        button {
            width: 100%;
            padding: 14px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            border-radius: 6px;
            font-size: 16px;
            font-weight: 600;
            cursor: pointer;
            transition: transform 0.2s;
        }
        button:hover {
            transform: translateY(-2px);
        }
        button:active {
            transform: translateY(0);
        }
        .error {
            background: #fee;
            border: 1px solid #fcc;
            color: #c33;
            padding: 12px;
            border-radius: 6px;
            margin-bottom: 20px;
            display: none;
        }
        .success {
            background: #efe;
            border: 1px solid #cfc;
            color: #3c3;
            padding: 12px;
            border-radius: 6px;
            margin-bottom: 20px;
            display: none;
        }
        .note {
            background: #f0f7ff;
            border-left: 4px solid #667eea;
            padding: 12px;
            margin-top: 20px;
            font-size: 13px;
            color: #555;
        }
        @media (max-width: 720px) {
            .container { padding: 30px 20px; }
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🚗 Welcome to CasGarage</h1>
        <p class="subtitle">First User Registration</p>

        <div id="error" class="error"></div>
        <div id="success" class="success"></div>

        <form id="registerForm">
            <div class="form-group">
                <label for="username">Username</label>
                <input type="text" id="username" name="username" required minlength="3" autocomplete="username">
            </div>

            <div class="form-group">
                <label for="email">Email (optional)</label>
                <input type="email" id="email" name="email" autocomplete="email">
            </div>

            <div class="form-group">
                <label for="password">Password</label>
                <input type="password" id="password" name="password" required minlength="8" autocomplete="new-password">
            </div>

            <div class="form-group">
                <label for="confirm_password">Confirm Password</label>
                <input type="password" id="confirm_password" name="confirm_password" required minlength="8" autocomplete="new-password">
            </div>

            <button type="submit">Create First User</button>
        </form>

        <div class="note">
            <strong>Note:</strong> This account will be used to create the <strong>administrator</strong> account in the next step.
        </div>
    </div>

    <script>
        document.getElementById('registerForm').addEventListener('submit', async (e) => {
            e.preventDefault();

            const formData = new FormData(e.target);
            const data = Object.fromEntries(formData);

            // Validate passwords match
            if (data.password !== data.confirm_password) {
                showError('Passwords do not match');
                return;
            }

            // Clear messages
            hideMessages();

            try {
                const response = await fetch('/api/v1/setup/register', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        username: data.username,
                        email: data.email || null,
                        password: data.password
                    })
                });

                const result = await response.json();

                if (response.ok) {
                    showSuccess('Registration successful! Redirecting to administrator setup...');
                    setTimeout(() => {
                        window.location.href = '/setup/admin';
                    }, 2000);
                } else {
                    showError(result.error || 'Registration failed');
                }
            } catch (err) {
                showError('Network error: ' + err.message);
            }
        });

        function showError(message) {
            const errorDiv = document.getElementById('error');
            errorDiv.textContent = message;
            errorDiv.style.display = 'block';
        }

        function showSuccess(message) {
            const successDiv = document.getElementById('success');
            successDiv.textContent = message;
            successDiv.style.display = 'block';
        }

        function hideMessages() {
            document.getElementById('error').style.display = 'none';
            document.getElementById('success').style.display = 'none';
        }
    </script>
</body>
</html>"#;

    Html(html.to_string())
}

/// Register first user API endpoint
async fn register_first_user(
    State(state): State<AppState>,
    Json(payload): Json<RegistrationRequest>,
) -> impl IntoResponse {
    // Check if admin users already exist
    match has_admin_users(state.db.sqlite()).await {
        Ok(true) => {
            return (
                StatusCode::CONFLICT,
                Json(RegistrationResponse {
                    success: false,
                    message: "Setup already complete".to_string(),
                    user_id: None,
                }),
            );
        }
        Ok(false) => {}
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(RegistrationResponse {
                    success: false,
                    message: format!("Database error: {}", e),
                    user_id: None,
                }),
            );
        }
    }

    // Validate input
    if payload.username.len() < 3 {
        return (
            StatusCode::BAD_REQUEST,
            Json(RegistrationResponse {
                success: false,
                message: "Username must be at least 3 characters".to_string(),
                user_id: None,
            }),
        );
    }

    if payload.password.len() < 8 {
        return (
            StatusCode::BAD_REQUEST,
            Json(RegistrationResponse {
                success: false,
                message: "Password must be at least 8 characters".to_string(),
                user_id: None,
            }),
        );
    }

    // Create first user with Admin role
    match create_user(
        state.db.sqlite(),
        &payload.username,
        &payload.password,
        payload.email.as_deref(),
        UserRole::Admin,
    )
    .await
    {
        Ok(user) => {
            info!("✓ First user created: {}", user.username);

            (
                StatusCode::CREATED,
                Json(RegistrationResponse {
                    success: true,
                    message: "User created successfully".to_string(),
                    user_id: Some(user.id),
                }),
            )
        }
        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(RegistrationResponse {
                    success: false,
                    message: format!("Failed to create user: {}", e),
                    user_id: None,
                }),
            )
        }
    }
}

/// Registration request
#[derive(Debug, Deserialize)]
struct RegistrationRequest {
    username: String,
    email: Option<String>,
    password: String,
}

/// Registration response
#[derive(Debug, Serialize)]
struct RegistrationResponse {
    success: bool,
    message: String,
    user_id: Option<i64>,
}

/// Setup status response
#[derive(Debug, Serialize)]
struct SetupStatus {
    setup_required: bool,
    message: String,
}
