use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterReponse {
    name: String,
    email: String,
}
