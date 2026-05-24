/// API client for communicating with CasGarage backend

use serde::{Deserialize, Serialize};

const API_BASE: &str = "/api/v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub message: String,
}

// TODO: Implement API client functions
