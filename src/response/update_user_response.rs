use serde::Serialize;

#[derive(Serialize)]
pub struct UpdateUserResponse {
    pub message: String,
}
