use actix_web::{dev::{Service, ServiceRequest, ServiceResponse}, error::ErrorUnauthorized, error::Error as AWError, HttpResponse, http::Error as HError};
use futures::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::usecase::auth_usecase::Claims;
use dotenv::dotenv;
use std::env;
use actix_web::error::ResponseError;

pub struct AuthMiddlewareService<S> {
    pub(crate) service: S
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = AWError> + 'static,
    S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = AWError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req.headers().get("Authorization").map(|v| v.to_str().unwrap_or(""));
        println!("Token: {:?}", token);
        
        if let Ok(claims) = validate_token(token.unwrap_or("")) {
            println!("Claims: {:?}", claims);
        } else {
            println!("Invalid token");
            return Box::pin(async move {
                Err(
                    ErrorUnauthorized("Unauthorized")
                )
            })
        }
        let fut = self.service.call(req);
        Box::pin(async move {
            let res: ServiceResponse<B> = fut.await?;
            Ok(res)
        })
    }
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    dotenv().ok();

    let token = token.trim_start_matches("Bearer ");
    let secret = &env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default()
    )
    .map(|data| data.claims)
}

