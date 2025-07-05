use axum::{extract::State, middleware, routing::post, Extension, Json, Router};

use crate::{models::{dto::nutrients::{UpdateMacrosDistribution, UpdateMacrosTarget}, errors::AppResult}, routes::AppState, security::{jwt::Token, middlewares::auth_middleware}, services::nutrients::{update_user_macros_distribution, update_user_macros_target}};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/distribution", post(update_distribution))
        .route("/target", post(update_target))
        .layer(middleware::from_fn_with_state(state, auth_middleware))
}

async fn update_distribution(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Json(dist): Json<UpdateMacrosDistribution>,
) -> AppResult<()> {
    update_user_macros_distribution(&state.db, token.sub, dist).await
}

async fn update_target(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Json(target): Json<UpdateMacrosTarget>,
) -> AppResult<()> {
    update_user_macros_target(&state.db, token.sub, target).await
}