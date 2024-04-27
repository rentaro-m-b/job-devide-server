use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error};
use futures::future::{ready, Ready};
use crate::middleware::service::auth_middleware_service::AuthMiddlewareService;

// ミドルウェアの処理には2つのステップがある。
// 1.ミドルウェアの初期化。
// 次の「サービス」をパラメータとして呼び出す。
// 2.ミドルウェアの呼び出しメソッドが「通常のリクエスト」で呼び出される。
pub struct AuthMiddleware;

// ミドルウェアのファクトリは `Transform` 型である。
// `S` - 次のサービスのタイプ。
// `B` - レスポンスボディの型。
impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
} 


