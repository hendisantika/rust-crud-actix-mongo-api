use serde::{Deserialize, Serialize};

use crate::users::models::{Role, Tokens};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}