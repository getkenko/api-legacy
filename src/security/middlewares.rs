use std::net::SocketAddr;

use axum::{body::Body, extract::{ConnectInfo, State}, http::{header::AUTHORIZATION, Request}, middleware::Next, response::IntoResponse};

use crate::{models::errors::{AppError, AppResult}, routes::AppState, security::jwt::Token};

// TODO: move to config file
const MAX_REQUESTS_PER_MINUTE: u32 = 100;

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
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> AppResult<impl IntoResponse> {
    // TODO: create unique identifier based on something more than IP address-
    //-so VPN users wont get rate limited by activity of others (use xxHash for keys)

    // create unique identifier
    let ip = addr.ip().to_string();
    let key = format!("ratelimit_{ip}");

    // increase request count for identifier
    let req_min = state.cache.increment_requests(&key).await?;

    // if req/min is above the threshold then return rate limit
    if req_min > MAX_REQUESTS_PER_MINUTE {
        return Err(AppError::RateLimit);
    }

    let res = next.run(req).await;
    Ok(res)
}