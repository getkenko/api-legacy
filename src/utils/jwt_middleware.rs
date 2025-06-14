use axum::{body::Body, http::Request, middleware::Next, response::IntoResponse};
use axum_extra::extract::CookieJar;
use sqlx::PgPool;

use crate::{models::errors::{AppError, AppResult}, utils::jwt::{AccessToken, AuthToken, RefreshToken}};

// TODO: split into multiple functions to make the code more readable

fn get_access_token(cookies: &CookieJar) -> Result<Option<AccessToken>, jsonwebtoken::errors::Error> {
    let cookie = cookies.get("access_token").map(|c| c.value());
    if let Some(value) = cookie {
        let token = AccessToken::decode(value)?;
        return Ok(token);
    }

    Ok(None)
}

fn get_refresh_token(cookies: &CookieJar) -> Result<Option<RefreshToken>, jsonwebtoken::errors::Error> {
    let cookie = cookies
        .get("refresh_token")
        .map(|c| c.value());

    if let Some(value) = cookie {
        let token = RefreshToken::decode(value)?;
        return Ok(token);
    }

    Ok(None)
}

fn refresh_access_token(db: &PgPool, refresh: &RefreshToken) -> AppResult<AccessToken> {
    // fetch user data from database
    // let user = sqlx::query!(
    //     "SELECT display_name, account_state FROM users WHERE id = $1",
    //     refresh.sub,
    // )
    // .fetch_optional(db)
    // .await?;

    // check if user's account is active

    // create access token
    // let access = AccessToken::new(user_id, display_name)

    todo!()
}

pub async fn jwt_middleware(
    cookies: CookieJar,
    mut req: Request<Body>,
    next: Next,
) -> AppResult<impl IntoResponse> {
    // try to extract access token => continue
    // try to decode access token => continue
    let access = get_access_token(&cookies)?;

    // try to extract refresh token => unauthoried
    // try to decode refresh token:
    //      invalid token => unauthorized,
    //      internal => internal server error
    if access.is_none() {
        let refresh = get_refresh_token(&cookies)?.ok_or(AppError::Unathorized)?;
        
        // create new access token
        
    }

    req.extensions_mut().insert(access);

    let res = next.run(req).await;
    Ok(res)
}
