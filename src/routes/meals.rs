use axum::{extract::{Path, State}, http::StatusCode, middleware, routing::{delete, get, post}, Extension, Json, Router};
use chrono::NaiveDate;
use serde::Serialize;
use uuid::Uuid;

use crate::{database::{meal::{self, add_meal_product, check_meal_item_exists, delete_meal_item, fetch_meal_product, fetch_user_meal_products_for_date, fetch_user_meal_section_exists, fetch_user_meal_sections}, product::fetch_product_by_id}, models::{database::{AddMealProduct, MealProductKind}, dto::{AddProduct, MealDayMacro, QuickAddProduct, UserMealSectionView}, errors::{AppError, AppResult}}, utils::{auth_middleware::auth_middleware, jwt::AccessToken}};

use super::AppState;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/sections", get(user_sections))
        .route("/products/{product_id}", delete(delete_meal_product))
        .route("/{date}", get(user_meals))
        .route("/{date}/macro", get(meal_macro))
        .route("/{date}/products", post(add_product))
        .route("/{date}/products/quick", post(quick_add_product)) // should we instead use query parameter in /products?

        .layer(middleware::from_fn_with_state(state, auth_middleware))
}

async fn meal_macro(
    State(db): State<AppState>,
    Extension(token): Extension<AccessToken>,
    Path(date): Path<NaiveDate>,
) -> AppResult<Json<MealDayMacro>> {
    let mut macro_sum = MealDayMacro::default();
    let products = fetch_user_meal_products_for_date(&db, &token.sub, date).await?;

    for (product, quantity) in products {
        match product.kind {
            MealProductKind::QuickAdd =>
                // database already makes sure that these fields are set when kind is quick add
                macro_sum.add_raw(product.calories.unwrap(), product.proteins.unwrap(), product.fats.unwrap(), product.carbohydrates.unwrap()),
            MealProductKind::FromDatabase => {
                let product = fetch_product_by_id(&db, product.product_id.unwrap()).await?;
                macro_sum.add_product(&product, quantity);
            }
        }
    }

    Ok(Json(macro_sum))
}

// TODO: move it to the models module and use BETTER name
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UserMealsProducts {
    product_id: Option<Uuid>,
    name: String,
    calories: i32,
    proteins: i32,
    fats: i32,
    carbohydrates: i32,
}

impl UserMealsProducts {
    fn new(product_id: Option<Uuid>, name: String, calories: i32, proteins: i32, fats: i32, carbohydrates: i32) -> Self {
        Self {
            product_id,
            name,
            calories,
            proteins,
            fats,
            carbohydrates,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UserMeals {
    section_id: Uuid,
    products: Vec<UserMealsProducts>,
}

async fn user_meals(
    State(db): State<AppState>,
    Extension(token): Extension<AccessToken>,
    Path(date): Path<NaiveDate>,
) -> AppResult<Json<Vec<UserMeals>>> {
    let mut res = vec![];

    // fetch user_meals for this date
    let meals = sqlx::query!(
        "SELECT id, section_id FROM user_meals WHERE user_id = $1 AND date = $2",
        token.sub, date,
    )
    .fetch_all(&db)
    .await?;

    for meal in meals {
        let mut products = vec![];

        // fetch meal items for this section
        let items = sqlx::query!(
            "SELECT meal_product_id, quantity FROM meal_items WHERE meal_id = $1",
            meal.id,
        )
        .fetch_all(&db)
        .await?;

        for item in items {
            let mp = fetch_meal_product(&db, &item.meal_product_id).await?;

            let product = match mp.kind {
                MealProductKind::QuickAdd => UserMealsProducts::new(None, mp.name.unwrap(), mp.calories.unwrap(), mp.proteins.unwrap(), mp.fats.unwrap(), mp.carbohydrates.unwrap()),
                MealProductKind::FromDatabase => {
                    let p = fetch_product_by_id(&db, mp.product_id.unwrap()).await?;
                    UserMealsProducts::new(Some(p.id), p.name, p.calories, p.proteins, p.fats, p.carbohydrates)
                }
            };

            products.push(product);
        }

        res.push(UserMeals { section_id: meal.section_id, products });
    }

    Ok(Json(res))
}

// sections
async fn user_sections(
    State(db): State<AppState>,
    Extension(token): Extension<AccessToken>,
) -> AppResult<Json<Vec<UserMealSectionView>>> {
    let sections = fetch_user_meal_sections(&db, &token.sub).await?;
    Ok(Json(sections))
}

// products
async fn add_product(
    State(db): State<AppState>,
    Extension(token): Extension<AccessToken>,
    Path(date): Path<NaiveDate>,
    Json(product): Json<AddProduct>,
) -> AppResult<StatusCode> {
    let section_exists = fetch_user_meal_section_exists(&db, &product.section_id).await?;
    if !section_exists {
        return Err(AppError::MealSectionNotFound);
    }

    let add_product = AddMealProduct::from_database(date, product.section_id, product.quantity, product.product_id);
    add_meal_product(&db, &token.sub, add_product).await?;

    Ok(StatusCode::CREATED)
}

async fn quick_add_product(
    State(db): State<AppState>,
    Extension(token): Extension<AccessToken>,
    Path(date): Path<NaiveDate>,
    Json(product): Json<QuickAddProduct>,
) -> AppResult<StatusCode> {
    let section_exists = fetch_user_meal_section_exists(&db, &product.section_id).await?;
    if !section_exists {
        return Err(AppError::MealSectionNotFound);
    }

    let add_product = AddMealProduct::quick_add(date, product.section_id, product.quantity, product.name, product.calories, product.proteins, product.fats, product.carbohydrates);
    add_meal_product(&db, &token.sub, add_product).await?;

    Ok(StatusCode::CREATED)
}

async fn delete_meal_product(
    State(db): State<AppState>,
    Path(meal_product_id): Path<Uuid>,
) -> AppResult<StatusCode> {
    if !check_meal_item_exists(&db, &meal_product_id).await? {
        return Err(AppError::MealProductNotFound);
    }

    delete_meal_item(&db, &meal_product_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
