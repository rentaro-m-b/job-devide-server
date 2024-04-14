use actix_web::{web, Responder};
use crate::{controller::{auth_controller::{self, AuthController}, sample_controller::SampleController}, request::register_request::RegisterRequest, usecase::auth_usecase::{self, AuthUsecase}};

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth_usecase = AuthUsecase::new();
    let auth_controller = web::Data::new(AuthController::new(auth_usecase));
    cfg.service(
        web::scope("/sample")
            .route("", web::get().to(SampleController::sample)),
    ).service(
        web::scope("/user")
            .app_data(auth_controller.clone())
            .route("", web::post().to(register)),
    );
}

async fn register(
    auth_controller: web::Data<AuthController>,
    web::Json(req): web::Json<RegisterRequest>,
) -> impl Responder {
    auth_controller.register(actix_web::web::Json(req)).await
}
