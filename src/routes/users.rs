use axum::{extract::State, middleware, routing::get, Extension, Json, Router};

use crate::{database::user::fetch_user_info, models::{dto::UserInfo, errors::AppResult}, utils::{auth_middleware::auth_middleware, jwt::AccessToken}};

use super::AppState;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/me", get(user_info))

        .layer(middleware::from_fn_with_state(state, auth_middleware))
}

async fn user_info(
    State(db): State<AppState>,
    token: Extension<AccessToken>,
) -> AppResult<Json<UserInfo>> {
    let info = fetch_user_info(&db, &token.sub).await?;
    Ok(Json(info))
}
