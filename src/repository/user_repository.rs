use diesel::prelude::*;
use crate::db::DbPool;
use crate::model::user::{NewUser, User};
use crate::schema::users;
use crate::schema::users::dsl::*;

pub struct UserRepository {
    pub pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        UserRepository{ pool }
    }

    pub fn create_user(&self, user: &NewUser) {
        let mut conn = self.pool.get().expect("Failed to get db connection from pool");
        println!("create_user");
        let _ = diesel::insert_into(users::table)
            .values(user)
            .execute(&mut conn);
    }

    pub fn find_user_by_email(&self, user_email: &str) -> Option<User> {
        let mut conn = self.pool.get().expect("Failed to get db connection from pool");
        users.filter(email.eq(user_email))
            .first::<User>(&mut conn)
            .ok()
    }
}
