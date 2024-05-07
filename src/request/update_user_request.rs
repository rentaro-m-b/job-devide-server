use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserRequest {
    pub name: String,
    pub email: String
}
