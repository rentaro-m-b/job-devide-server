use actix_web::web;
use actix_web::{HttpResponse, Responder};
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
        web::Json(req): web::Json<RegisterRequest>
    ) -> impl Responder {
        let result = self.auth_usecase.register(&req.name, &req.email, &req.password).await;
        println!("register {}", result);
        HttpResponse::Ok().json(req)
        
        // match result {
        //     Ok(user) => HttpResponse::Ok().json(user),
        //     Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
        // }
    }

    pub fn login(&self, password: &str, password_hash: &str) -> Result<bool, argon2::password_hash::Error> {
        let is_valid = self.auth_usecase.verify_password(password, password_hash);

        Ok(is_valid)
    }
}
