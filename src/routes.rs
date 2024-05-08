use actix_web::{web::{self, Payload}, HttpRequest};
use crate::{
    controller::{
        auth_controller::AuthController,
        sample_controller::SampleController
    }, db::get_connection_pool, middleware::{auth_middleware::AuthMiddleware, sample_middleware::SayHi}, repository::user_repository::UserRepository, request::{login_request::LoginRequest, register_request::RegisterRequest, update_user_request::UpdateUserRequest}, usecase::auth_usecase::AuthUsecase
};

pub fn config(cfg: &mut web::ServiceConfig) {
    let pool = get_connection_pool();
    let user_repository = UserRepository::new(pool.clone());
    let auth_usecase = AuthUsecase::new(user_repository);
    let auth_controller = web::Data::new(AuthController::new(auth_usecase));
    cfg.service(
        web::scope("/sample")
            .wrap(SayHi)
            .wrap(AuthMiddleware)
            .route("", web::get().to(SampleController::sample)),
    )
    .service(
        web::scope("/user")
            .app_data(auth_controller.clone())  // Clone auth_controller Data for this scope
            .route("register", web::post().to(|data: web::Data<AuthController>, req: web::Json<RegisterRequest>| async move {
                data.register(req).await
            }))
            .route("login", web::post().to(|data: web::Data<AuthController>, req: web::Json<LoginRequest>| async move {
                data.login(req).await
            }))
            .service(
                web::scope("")
                .wrap(AuthMiddleware)
                .route("update", web::put().to(|data: web::Data<AuthController>, req: HttpRequest, payload: web::Json<UpdateUserRequest>| async move {
                    data.update_user(req, payload).await
                }))
                .route("delete", web::delete().to(|data: web::Data<AuthController>, req: HttpRequest| async move {
                    data.delete_user(req).await
                }))
            )
    );
}
