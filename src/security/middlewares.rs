use std::net::SocketAddr;

use axum::{body::Body, extract::{ConnectInfo, State}, http::{header::AUTHORIZATION, Request}, middleware::Next, response::IntoResponse};
use chrono::{Duration, Utc};

use crate::{config::CONFIG, database::user_repo, models::errors::{AppError, AppResult, ValidationError}, routes::AppState, security::jwt::Token};

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
        .map_err(|_| ValidationError::InvalidToken)?;

    // strip bearer prefix
    let token_str = auth_header
        .strip_prefix("Bearer ")
        .ok_or(ValidationError::InvalidAuthHeader)?;

    // try to decode token from data
    let token = Token::decode(token_str)?
        .ok_or(AppError::Unathorized)?;

    // check if token is linked to valid user
    let last_check = state.cache.user_last_check(token.sub).await?;
    if last_check + Duration::seconds(CONFIG.user_check_interval) <= Utc::now() {
        let user_exists = user_repo::check_user_exists(&state.db, token.sub).await?;
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
    let ip = addr.ip().to_string();
    let req_min = state.cache.increment_requests(&ip).await?;

    // if req/min is above the threshold then return rate limit
    if req_min > CONFIG.rate_limit.max_requests {
        return Err(AppError::RateLimit);
    }

    let res = next.run(req).await;
    Ok(res)
}