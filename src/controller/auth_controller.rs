use actix_web::{web, HttpResponse, Responder};
use crate::model::user::User;
use crate::usecase::auth_usecase::AuthUsecase;
use crate::request::register_request::RegisterRequest;

pub struct AuthController {
    auth_usecase: AuthUsecase
}

impl AuthController {
    pub fn new(auth_usecase: AuthUsecase) -> Self {
        AuthController{ auth_usecase }
    }

    // こちらユーザを引数に放り込んだほうが良いだろう
    pub async fn register(
        &self,
        web::Json(req): web::Json<RegisterRequest>,
    ) -> impl Responder {
        let hashed_password = match self.auth_usecase.hash_password(&req.password) {
            Ok(hashed_password) => hashed_password,
            Err(e) => {
                return HttpResponse::InternalServerError().finish();
            }
        };

        let id = 1;
        let user = User::new(id, &req.name, &req.email, &hashed_password);

        HttpResponse::Ok().json(user)
    }

    pub fn login(&self, password: &str, password_hash: &str) -> Result<bool, argon2::password_hash::Error> {
        let is_valid = self.auth_usecase.verify_password(password, password_hash)?;

        Ok(is_valid)
    }
}
