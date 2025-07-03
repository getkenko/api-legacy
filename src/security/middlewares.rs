use std::net::SocketAddr;

use axum::{body::Body, extract::{ConnectInfo, State}, http::{header::AUTHORIZATION, Request}, middleware::Next, response::IntoResponse};
use chrono::{Duration, Utc};

use crate::{database::user::check_user_exists, models::errors::{AppError, AppResult}, routes::AppState, security::jwt::Token};

// TODO: move to config file
const MAX_REQUESTS_PER_MINUTE: u32 = 100;
const USER_CHECK_INTERVAL: Duration = Duration::minutes(3);

pub async fn auth_middleware(
    State(state): State<AppState>,
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

    // check if token is linked to valid user
    let last_check = state.cache.user_last_check(token.sub).await?;
    tracing::info!("before user check");
    if last_check + USER_CHECK_INTERVAL <= Utc::now() {
        tracing::info!("user check");
        let user_exists = check_user_exists(&state.db, token.sub).await?;
        if !user_exists {
            return Err(AppError::Unathorized);
        }

        state.cache.update_user_last_check(token.sub).await?;
    }

    // insert token and forward request
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

    let ip = addr.ip().to_string();
    let req_min = state.cache.increment_requests(&ip).await?;

    // if req/min is above the threshold then return rate limit
    if req_min > MAX_REQUESTS_PER_MINUTE {
        return Err(AppError::RateLimit);
    }

    let res = next.run(req).await;
    Ok(res)
}