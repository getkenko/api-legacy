use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{patch, post}, Extension, Json, Router};
use uuid::Uuid;

use crate::{models::{dto::fridge::{CreateFridgeProductDto, FridgeProductDto, UpdateFridgeProductDto}, errors::AppResult}, routes::AppState, security::jwt::Token, services::fridge_service};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(add_product).get(get_products))
        .route("/{id}", patch(update_product).delete(delete_product))
}

async fn add_product(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Json(product): Json<CreateFridgeProductDto>,
) -> AppResult<impl IntoResponse> {
    let product = fridge_service::add_product(&state.db, token.sub, product).await?;
    let product_view = FridgeProductDto::from(product);
    Ok((StatusCode::CREATED, Json(product_view)))
}

async fn get_products(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
) -> AppResult<Json<Vec<FridgeProductDto>>> {
    let products = fridge_service::get_products(&state.db, token.sub).await?;
    let product_views = products.into_iter().map(|p| p.into()).collect::<Vec<_>>();
    Ok(Json(product_views))
}

async fn update_product(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Path(product_id): Path<Uuid>,
    Json(update): Json<UpdateFridgeProductDto>,
) -> AppResult<Json<FridgeProductDto>> {
    let updated_product = fridge_service::update_product(&state.db, token.sub, product_id, update).await?;
    Ok(Json(updated_product.into()))
}

async fn delete_product(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Path(product_id): Path<Uuid>,
) -> AppResult<StatusCode> {
    fridge_service::delete_product(&state.db, token.sub, product_id).await?;
    Ok(StatusCode::NO_CONTENT)
}