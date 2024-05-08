use actix_web::{web, HttpMessage, HttpRequest};
use actix_web::{HttpResponse, Responder};
use crate::request::login_request::LoginRequest;
use crate::request::update_user_request::UpdateUserRequest;
use crate::usecase::auth_usecase::AuthUsecase;
use crate::request::register_request::RegisterRequest;
use crate::response::login_response::LoginResponse;
use crate::response::update_user_response::UpdateUserResponse;

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

    pub async fn update_user(&self, req: HttpRequest, web::Json(payload): web::Json<UpdateUserRequest>) -> impl Responder {
        let message;
        if let Some(user_id) = req.extensions().get::<i32>() {
            message = "Token found";
            self.auth_usecase.update(&payload.name, &payload.email, *user_id).await;
        } else {
            message = "Token not found";
        }

        HttpResponse::Ok().json(UpdateUserResponse{ message: message.to_string() })
    }

    pub async fn delete_user(&self, req: HttpRequest) -> impl Responder {
        let message;
        if let Some(user_id) = req.extensions().get::<i32>() {
            message = "Token found";
            self.auth_usecase.delete(*user_id).await;
        } else {
            message = "Token not found";
        }

        HttpResponse::Ok().json(UpdateUserResponse{ message: message.to_string() })
    }
}
