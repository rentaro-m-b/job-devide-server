use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use diesel::Insertable;
use diesel::AsChangeset;
use diesel::Queryable;
use diesel::Selectable;
use crate::schema::users;

#[derive(Queryable, Insertable, Selectable, AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, Selectable, AsChangeset, Debug, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl NewUser {
    pub fn new(name: &str, email: &str, password_hash: &str) -> Self {
        let now = Utc::now().naive_utc();

        NewUser {
            name: name.to_string(),
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            created_at: now,
            updated_at: now
        }
    }
}

pub struct UpdateUser {
    pub name: String,
    pub email: String,
    pub updated_at: NaiveDateTime,
}

impl UpdateUser {
    pub fn new(name: &str, email: &str) -> Self {
        let now = Utc::now().naive_utc();

        UpdateUser {
            name: name.to_string(),
            email: email.to_string(),
            updated_at: now
        }
    }
}
