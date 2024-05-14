use actix_web::{dev::ServiceRequest, Error};
use actix_web::HttpMessage;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, DecodingKey, Validation};
use futures::future::{ready, Ready};
use std::env;
use crate::models::users::Claims;

pub fn jwt_middleware(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Ready<Result<ServiceRequest, (Error, ServiceRequest)>> {
    let token = credentials.token();
    let secret = env::var("JWT_SECRET").unwrap();
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::default();

    match decode::<Claims>(&token, &decoding_key, &validation) {
        Ok(token_data) => {
            req.extensions_mut().insert(token_data.claims);
            ready(Ok(req))
        }
        Err(_) => ready(Err((actix_web::error::ErrorUnauthorized("Invalid token"), req))),
    }
}
