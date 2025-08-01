mod user;
mod user_nutrients;
mod user_fridge_products;
mod product;
mod meal;
mod section;

pub mod user_repo {
    pub use super::user::*;
}

pub mod user_nutrients_repo {
    pub use super::user_nutrients::*;
}

pub mod fridge_repo {
    pub use super::user_fridge_products::*;
}

pub mod product_repo {
    pub use super::product::*;
}

pub mod meal_repo {
    pub use super::meal::*;
}

pub mod section_repo {
    pub use super::section::*;
}