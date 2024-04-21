use actix_web::{web, Responder, HttpRequest};
use crate::{
    controller::{
        auth_controller::AuthController,
        sample_controller::SampleController
    },
    db::get_connection_pool,
    repository::user_repository::UserRepository,
    request::register_request::RegisterRequest,
    usecase::auth_usecase::AuthUsecase
};

pub fn config(cfg: &mut web::ServiceConfig) {
    let pool = get_connection_pool();
    let user_repository = UserRepository::new(pool.clone());
    let auth_usecase = AuthUsecase::new(user_repository);
    let auth_controller = web::Data::new(AuthController::new(auth_usecase));
    cfg.service(
        web::scope("/sample")
            .route("", web::get().to(SampleController::sample)),
    )
    .service(
        web::scope("/user")
            .app_data(auth_controller.clone())  // Clone auth_controller Data for this scope
            .route("", web::post().to(|data: web::Data<AuthController>, req: web::Json<RegisterRequest>| async move {
                data.register(req).await
            })),
    );
}
