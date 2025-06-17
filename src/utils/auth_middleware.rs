use axum::{body::Body, extract::State, http::Request, middleware::Next, response::IntoResponse};
use axum_extra::extract::CookieJar;
use sqlx::PgPool;

use crate::{database::user::find_user_by_id, models::errors::{AppError, AppResult}, routes::AppState, utils::jwt::{AccessToken, AuthToken, RefreshToken}};

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

async fn authorize_user(db: &PgPool, cookies: &CookieJar) -> AppResult<AccessToken> {
    let token = match get_access_token(&cookies)? {
        Some(t) => t,
        None => {
            let refresh = get_refresh_token(&cookies)?.ok_or(AppError::Unathorized)?;

            // TODO: remove cookies if user is None
            let user = find_user_by_id(db, &refresh.sub)
                .await?
                .ok_or(AppError::Unathorized)?;

            AccessToken::new(&refresh.sub, &user.display_name)
        }
    };

    Ok(token)
}

pub async fn auth_middleware(
    State(db): State<AppState>,
    cookies: CookieJar,
    mut req: Request<Body>,
    next: Next,
) -> AppResult<impl IntoResponse> {
    let token = authorize_user(&db, &cookies).await?;

    req.extensions_mut().insert(token);

    let res = next.run(req).await;
    Ok(res)
}
