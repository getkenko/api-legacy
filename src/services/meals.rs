use std::collections::HashMap;

use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{database::{meal::{check_meal_item_exists, delete_meal_item, fetch_user_meal_product_count, fetch_user_meals_products, insert_meal_product}, meal_section::check_meal_section_exists, product::check_product_exists}, models::{database::meal::InsertMealProduct, dto::meals::{AddMealProductRequest, Macros, MealDayMacrosResponse, QuickAddMealProductRequest, UserMealProductView}, errors::{AppError, AppResult}}, utils::validation::validate_meal_date};

const USER_MEAL_PRODUCT_LIMIT: i64 = 100;

pub async fn calc_meal_day_macros(db: &PgPool, user_id: Uuid, date: NaiveDate) -> AppResult<MealDayMacrosResponse> {
    // check if date is today or past days
    validate_meal_date(date)?;

    let products = fetch_user_meals_products(db, user_id, date).await?;

    let mut per_section: HashMap<Uuid, Macros> = HashMap::new();
    let mut total = Macros::default();

    for product in products {
        total.add(&product);

        if let Some(macros) = per_section.get_mut(&product.section_id) {
            macros.add(&product);
        } else {
            let mut macros = Macros::default();
            macros.add(&product);
            per_section.insert(product.section_id, macros);
        }
    }

    Ok(MealDayMacrosResponse { per_section, total })
}

pub async fn get_user_meals_for_date(db: &PgPool, user_id: Uuid, date: NaiveDate) -> AppResult<HashMap<Uuid, Vec<UserMealProductView>>> {
    validate_meal_date(date)?;

    let mut meals: HashMap<Uuid, Vec<UserMealProductView>> = HashMap::new();

    let meals_products = fetch_user_meals_products(db, user_id, date).await?;

    for meal_product in meals_products.into_iter() {
        let section_id = meal_product.section_id;
        let meal_product_view = UserMealProductView::from(meal_product);

        if let Some(section) = meals.get_mut(&section_id) { // already exists
            section.push(meal_product_view);
        } else { // create new key-value pair
            meals.insert(section_id, vec![meal_product_view]);
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
) -> AppResult<UserMealProductView> {
    validate_meal_date(date)?;
    check_section_exists(db, user_id, product.section_id).await?;
    validate_product_exists(db, product.product_id).await?;
    check_product_limit(db, user_id, date).await?;

    let insert = InsertMealProduct::from(product);
    let product = insert_meal_product(db, user_id, date, insert).await?;
    let view = UserMealProductView::from(product);
    Ok(view)
}

pub async fn quick_add_meal_product_for_date(
    db: &PgPool,
    user_id: Uuid,
    date: NaiveDate,
    product: QuickAddMealProductRequest,
) -> AppResult<UserMealProductView> {
    validate_meal_date(date)?;
    check_section_exists(db, user_id, product.section_id).await?;
    check_product_limit(db, user_id, date).await?;

    // name must be non-empty, macros must be >= 0, quantity needs to be >= 0
    if product.name.is_empty() {
        return Err(AppError::MealProductEmptyName);
    } else if product.calories < 0 || product.proteins < 0 || product.fats < 0 || product.carbs < 0 {
        return Err(AppError::MealProductNegativeMacros);
    } else if product.quantity <= 0 {
        return Err(AppError::MealProductInvalidQuantity);
    }

    let insert = InsertMealProduct::from(product);
    let product = insert_meal_product(db, user_id, date, insert).await?;
    let view = UserMealProductView::from(product);
    Ok(view)
}

pub async fn delete_meal_product(db: &PgPool, product_id: Uuid) -> AppResult<()> {
    if !check_meal_item_exists(db, product_id).await? {
        return Err(AppError::MealProductNotFound);
    }

    delete_meal_item(db, product_id).await?;

    Ok(())
}
