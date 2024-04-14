use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

// とりあえず、このようにユーザを定義する。idはusizeで良いよな？uusizeとかないのかな？まあ、何が良いのかわかっていないのだけど
// これをなんとなしに登録処理に突っ込もうとしていたけど、リクエストを受け取るデータの構造体を定義してそれを使うのが基本だよな
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: usize,
    name: String,
    email: String,
    password_hash: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl User {
    pub fn new(id: usize, name: &str, email: &str, password_hash: &str) -> Self {
        let now = Utc::now().naive_utc();

        User {
            id: id,
            name: name.to_string(),
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            created_at: now,
            updated_at: now
        }
    }
}
