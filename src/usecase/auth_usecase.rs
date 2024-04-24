use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use crate::{
    model::user::{NewUser, User},
    repository::user_repository::UserRepository
};

pub struct AuthUsecase {
    user_repository: UserRepository
}

impl AuthUsecase {
    pub fn new(user_repository: UserRepository) -> Self {
        AuthUsecase{ 
            user_repository
        }
    }

    pub async fn register(&self, name: &str, email: &str, password: &str) -> bool {
        let hashed_password = self.hash_password(password).await;
        let new_user = NewUser::new(name, email, &hashed_password);
        self.user_repository.create_user(&new_user)
    }

    pub async fn hash_password(&self, password: &str) -> String {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let hashed_result = argon2.hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string());

        // have not written error handling
        match hashed_result {
            Ok(hashed_password) => hashed_password,
            Err(e) => {
                "".to_string()
            }
        }
    }

    pub fn verify_password(&self, password: &str, password_hash: &str) -> bool {
        let argon2 = Argon2::default();
        let parsed_result = PasswordHash::new(password_hash);
        
        match parsed_result {
            Ok(parsed_hash) => argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok(),
            Err(e) => {
                false
            }
        }
    }
}
