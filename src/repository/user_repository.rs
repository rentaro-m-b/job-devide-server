use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::db::DbPool;
use crate::model::user::{NewUser, User};
use crate::schema::users;

pub struct UserRepository {
    pub pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        UserRepository{ pool }
    }

    pub fn create_user(&self, user: &NewUser) -> bool {
        let mut conn = self.pool.get().expect("Failed to get db connection from pool");
        println!("create_user");
        diesel::insert_into(users::table)
            .values(user)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .is_ok()
    }
}
