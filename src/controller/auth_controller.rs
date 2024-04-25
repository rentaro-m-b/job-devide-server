use actix_web::web;
use actix_web::{HttpResponse, Responder};
use crate::request::login_request::LoginRequest;
use crate::usecase::auth_usecase::AuthUsecase;
use crate::request::register_request::RegisterRequest;
use crate::response::login_response::LoginResponse;

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
        self.auth_usecase.register(&req.name, &req.email, &req.password).await;
        HttpResponse::Ok().json(req)
    }

    pub async fn login(&self, web::Json(req): web::Json<LoginRequest>) -> impl Responder {
        let token = self.auth_usecase.verify_password(&req.email, &req.password);

        HttpResponse::Ok().json(LoginResponse{ token: token })
    }
}
