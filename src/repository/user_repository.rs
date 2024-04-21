use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::db::DbPool;
use crate::model::user::User;
use crate::schema::users;

pub struct UserRepository {
    pub pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        UserRepository{ pool }
    }

    pub fn create_user(&self, user: &User) -> bool {
        let mut conn = self.pool.get().expect("Failed to get db connection from pool");

        diesel::insert_into(users::table)
            .values((
                users::name.eq(&user.name),
                users::email.eq(&user.email),
                users::password_hash.eq(&user.password_hash)
            ))
            .execute(&mut conn)
            .is_ok()
    }
}
