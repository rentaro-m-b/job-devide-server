use diesel::prelude::*;
use diesel::result::{Error as DieselError, DatabaseErrorKind};
use crate::db::DbPool;
use crate::model::user::{NewUser, UpdateUser, User};
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
        // emailの重複があったら何かメッセージを出したいな〜
        let mut conn = self.pool.get().expect("Failed to get db connection from pool");
        println!("create_user");

        // トランザクションの開始
        let transaction_result = conn.transaction::<_, DieselError, _>(|transaction_conn| {
            // `insert_into` でユーザーを挿入
            diesel::insert_into(users::table)
                .values(user)
                .execute(transaction_conn)
                .map_err(|e| match e {
                    // ユニーク制約違反のエラーメッセージを返す
                    DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                        DieselError::RollbackTransaction
                    }
                    _ => e,
                })
        });

        // トランザクション全体の結果に基づいてメッセージを生成
        match transaction_result {
            Ok(_) => println!("transaction ok"),
            Err(DieselError::RollbackTransaction) => println!("Email already exists."),
            Err(_) => println!("Failed to create user.")
        };
    }

    pub fn update_user(&self, user: &UpdateUser, user_id: i32) {
        let mut conn = self.pool.get().expect("failde to get db connection from pool");
        println!("update_user");

        // トランザクションの開始
        let transaction_result = conn.transaction::<_, DieselError, _>(|transaction_conn| {
            let target = users.filter(id.eq(user_id));

            diesel::update(target)
                .set((name.eq(user.name.as_str()), email.eq(user.email.as_str())))
                .execute(transaction_conn)
                .map_err(|e| match e {
                    // ユニーク制約違反のエラーメッセージを返す
                    DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                        DieselError::RollbackTransaction
                    }
                    _ => e,
                })
        });
    
        // トランザクション全体の結果に基づいてメッセージを生成
        match transaction_result {
            Ok(_) => println!("transaction ok"),
            Err(DieselError::RollbackTransaction) => println!("Email already exists."),
            Err(_) => println!("Failed to update user.")
        };
    }

    pub fn delete_user(&self, user_id: i32) {
        let mut conn = self.pool.get().expect("failde to get db connection from pool");
        
        // トランザクションの開始
        let transaction_result = conn.transaction::<_, DieselError, _>(|transaction_conn| {
            let target = users.filter(id.eq(user_id));

            let rows_affected = diesel::delete(target)
                .execute(transaction_conn)?;

            if rows_affected == 0 {
                Err(DieselError::NotFound)
            } else {
                Ok(())
            }
        });
    
        // トランザクション全体の結果に基づいてメッセージを生成
        match transaction_result {
            Ok(_) => println!("transaction ok"),
            Err(DieselError::NotFound) => println!("User not found."),
            Err(_) => println!("Failed to delete user.")
        };
    }

    pub fn find_user_by_email(&self, user_email: &str) -> Option<User> {
        let mut conn = self.pool.get().expect("Failed to get db connection from pool");
        users.filter(email.eq(user_email))
            .first::<User>(&mut conn)
            .ok()
    }

    
}
