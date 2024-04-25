use actix_web::{dev::{Service, ServiceRequest, ServiceResponse}, Error};
use futures::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;

pub struct AuthMiddlewareService<S> {
    pub(crate) service: S
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req.headers().get("Authorization").map(|v| v.to_str().unwrap_or(""));
        println!("Token: {:?}", token);
        if let Ok(claims) = Claims::validate_token(token.unwrap_or("")) {
            println!("Claims: {:?}", claims);
        } else {
            println!("Invalid token");
        }
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize
}

impl Claims {
    pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token = token.trim_start_matches("Bearer ");
        decode::<Claims>(
            token,
            &DecodingKey::from_secret("your_secret_key".as_ref()),
            &Validation::default()
        )
        .map(|data| data.claims)
    }
}
