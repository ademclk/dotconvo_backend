use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterInput {
    pub username: String,
    pub email: String,
    pub password: String
}