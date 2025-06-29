use std::collections::HashMap;

use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{database::{meal::{check_meal_item_exists, delete_meal_item, fetch_user_meal_product_count, fetch_user_meals_products, insert_meal_product}, meal_section::check_meal_section_exists, product::check_product_exists}, models::{database::meal::InsertMealProduct, dto::meals::{AddMealProductRequest, MealMacroResponse, QuickAddMealProductRequest, UserMealProductView}, errors::{AppError, AppResult}}};

const USER_MEAL_PRODUCT_LIMIT: i64 = 100;

pub async fn calculate_meal_day_macro(db: &PgPool, user_id: Uuid, date: NaiveDate) -> AppResult<MealMacroResponse> {
    let products = fetch_user_meals_products(db, user_id, date).await?;
    let day_macro = MealMacroResponse::from_meals_products(&products);
    Ok(day_macro)
}

pub async fn get_user_meals_for_date(db: &PgPool, user_id: Uuid, date: NaiveDate) -> AppResult<HashMap<Uuid, Vec<UserMealProductView>>> {
    let mut meals: HashMap<Uuid, Vec<UserMealProductView>> = HashMap::new();

    let meals_products = fetch_user_meals_products(db, user_id, date).await?;

    for meal_product in meals_products.iter() {
        let meal_product_view = UserMealProductView::from(meal_product);

        if let Some(section) = meals.get_mut(&meal_product.section_id) { // already exists
            section.push(meal_product_view);
        } else { // create new key-value pair
            meals.insert(meal_product.section_id, vec![meal_product_view]);
        }
    }

    Ok(meals)
}

// swaglord: im out of names vro, route handlers and services have the same function names
// same for database query wrappers omggg
async fn validate_product_exists(db: &PgPool, product_id: Uuid) -> AppResult<()> {
    let exists = check_product_exists(db, product_id).await?;
    if !exists {
        return Err(AppError::ProductNotFound);
    }

    Ok(())
}

async fn check_section_exists(db: &PgPool, user_id: Uuid, section_id: Uuid) -> AppResult<()> {
    let section_exists = check_meal_section_exists(db, user_id, section_id).await?;
    if !section_exists {
        return Err(AppError::MealSectionNotFound);
    }

    Ok(())
}

async fn check_product_limit(db: &PgPool, user_id: Uuid, date: NaiveDate) -> AppResult<()> {
    let count = fetch_user_meal_product_count(db, user_id, date).await?;
    if count >= USER_MEAL_PRODUCT_LIMIT {
        return Err(AppError::MealProductLimitReached);
    }

    Ok(())
}

pub async fn add_meal_product_for_date(
    db: &PgPool,
    user_id: Uuid,
    date: NaiveDate,
    product: AddMealProductRequest,
) -> AppResult<()> {
    check_section_exists(db, user_id, product.section_id).await?;
    validate_product_exists(db, product.product_id).await?;
    check_product_limit(db, user_id, date).await?;

    let insert = InsertMealProduct::from(product);
    insert_meal_product(db, user_id, date, insert).await?;
    Ok(())
}

pub async fn quick_add_meal_product_for_date(
    db: &PgPool,
    user_id: Uuid,
    date: NaiveDate,
    product: QuickAddMealProductRequest,
) -> AppResult<()> {
    check_section_exists(db, user_id, product.section_id).await?;
    check_product_limit(db, user_id, date).await?;

    let insert = InsertMealProduct::from(product);
    insert_meal_product(db, user_id, date, insert).await?;
    Ok(())
}

pub async fn delete_meal_product(db: &PgPool, product_id: Uuid) -> AppResult<()> {
    if !check_meal_item_exists(db, product_id).await? {
        return Err(AppError::MealProductNotFound);
    }

    delete_meal_item(db, product_id).await?;

    Ok(())
}
