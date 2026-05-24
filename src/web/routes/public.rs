//! Public routes (accessible without authentication)

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::web::server::AppState;

/// Public routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(index_page))
        .route("/login", get(login_page))
        .route("/robots.txt", get(robots_txt))
        .route("/.well-known/security.txt", get(security_txt))
}

/// Index/landing page
async fn index_page() -> impl IntoResponse {
    // For now, use embedded HTML directly
    // Will be replaced with proper template rendering
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CasGarage</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 20px;
        }
        .container {
            background: white;
            border-radius: 16px;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
            padding: 60px;
            max-width: 800px;
            width: 90%;
            text-align: center;
        }
        @media (max-width: 720px) {
            .container { padding: 30px 20px; width: 98%; }
        }
        h1 {
            font-size: 48px;
            margin-bottom: 10px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
        }
        .btn {
            display: inline-block;
            padding: 14px 30px;
            margin: 10px;
            border-radius: 8px;
            text-decoration: none;
            font-weight: 600;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🚗 CasGarage</h1>
        <p>S3-Compatible Object Storage</p>
        <br>
        <a href="/setup/register" class="btn">Get Started</a>
        <a href="/api/v1/health" class="btn">Health Check</a>
    </div>
</body>
</html>"#;
    Html(html)
}

/// Login page
async fn login_page() -> impl IntoResponse {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Login - CasGarage</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
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
            max-width: 400px;
            width: 100%;
        }
        h1 { color: #333; margin-bottom: 10px; }
        .subtitle { color: #666; margin-bottom: 30px; font-size: 14px; }
        .form-group { margin-bottom: 20px; }
        label { display: block; margin-bottom: 8px; color: #333; font-weight: 500; }
        input {
            width: 100%;
            padding: 12px;
            border: 2px solid #e0e0e0;
            border-radius: 6px;
            font-size: 14px;
        }
        input:focus { outline: none; border-color: #667eea; }
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
        }
        button:hover { transform: translateY(-2px); }
        .error { background: #fee; border: 1px solid #fcc; color: #c33; padding: 12px; border-radius: 6px; margin-bottom: 20px; display: none; }
        @media (max-width: 720px) { .container { padding: 30px 20px; } }
    </style>
</head>
<body>
    <div class="container">
        <h1>🔐 Login</h1>
        <p class="subtitle">CasGarage Admin Access</p>
        <div id="error" class="error"></div>
        <form id="loginForm">
            <div class="form-group">
                <label for="username">Username</label>
                <input type="text" id="username" name="username" required autofocus>
            </div>
            <div class="form-group">
                <label for="password">Password</label>
                <input type="password" id="password" name="password" required>
            </div>
            <button type="submit">Login</button>
        </form>
    </div>
    <script>
        document.getElementById('loginForm').addEventListener('submit', async (e) => {
            e.preventDefault();
            const data = new FormData(e.target);
            // TODO: Implement login API call
            document.getElementById('error').textContent = 'Login functionality coming soon';
            document.getElementById('error').style.display = 'block';
        });
    </script>
</body>
</html>"#;

    Html(html)
}

/// robots.txt (configurable via admin UI in full implementation)
async fn robots_txt() -> impl IntoResponse {
    let robots = "User-agent: *\nDisallow: /admin/\nDisallow: /api/\n";
    (StatusCode::OK, [("Content-Type", "text/plain")], robots)
}

/// security.txt
async fn security_txt() -> impl IntoResponse {
    let security = "Contact: mailto:casjay@yahoo.com\n\
                    Preferred-Languages: en\n\
                    Canonical: https://casgarage.readthedocs.io/.well-known/security.txt\n";
    (StatusCode::OK, [("Content-Type", "text/plain")], security)
}
