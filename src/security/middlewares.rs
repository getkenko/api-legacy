use axum::{body::Body, http::{header::AUTHORIZATION, Request}, middleware::Next, response::IntoResponse};

use crate::{models::errors::{AppError, AppResult}, security::jwt::Token};

pub async fn auth_middleware(
    mut req: Request<Body>,
    next: Next,
) -> AppResult<impl IntoResponse> {
    // try to extract authorization header data
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .map(|h| h.to_str())
        .ok_or(AppError::Unathorized)?
        .map_err(|_| AppError::TokenInvalidSymbols)?;

    // strip bearer prefix
    let token_str = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::InvalidAuthFormat)?;

    // try to decode token from data
    let token = Token::decode(token_str)?
        .ok_or(AppError::Unathorized)?;

    // insert token and resume request
    req.extensions_mut().insert(token);
    let res = next.run(req).await;
    
    Ok(res)
}

// RATE LIMIT MIDDLEWARE
pub async fn rate_limit_middleware(
    req: Request<Body>,
    next: Next,
) -> AppResult<impl IntoResponse> {
    // create user unique id using MAC + IP
    // (using just IP will make the API unreachable for VPN users)
    // increase requests in last 60 seconds in redis
    // if req / min is above the threshold then return rate limited error

    let res = next.run(req).await;
    Ok(res)
}