use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use crate::{
    model::user::{NewUser, User},
    repository::user_repository::UserRepository, schema::users::password_hash
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

    pub async fn register(&self, name: &str, email: &str, password: &str) {
        let user_password_hash = self.hash_password(password);
        let new_user = NewUser::new(name, email, &user_password_hash);
        self.user_repository.create_user(&new_user);
    }

    fn hash_password(&self, password: &str) -> String {
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

    pub fn verify_password(&self, user_email: &str, password: &str) -> String {
        let option_user = self.user_repository.find_user_by_email(user_email);
        let user_password_hash = match option_user {
            Some(user) => user.password_hash,
            None => return "".to_string()
        };

        let argon2 = Argon2::default();
        let parsed_result = match PasswordHash::new(user_password_hash.as_str()) {
            Ok(parsed_hash) => argon2.verify_password(password.as_bytes(), &parsed_hash),
            Err(e) => {
                return "".to_string()
            }
        };

        let create_token_result = match parsed_result {
            Ok(_) => self.create_token(user_email),
            Err(_) => return "".to_string()
        };

        match create_token_result {
            Ok(token) => token,
            Err(_) => "".to_string()
        }
    }

    fn create_token(&self, user_email: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let iat = chrono::Utc::now();
        let exp = iat + chrono::Duration::days(2);
        let claims = Claims {
            iat: iat.timestamp() as usize,
            exp: exp.timestamp() as usize,
            email: user_email.to_owned()
        };

        encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
    email: String
}
