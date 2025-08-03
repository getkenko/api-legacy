pub mod auth;
pub mod meals;
pub mod products;
mod users;
pub mod sections;
pub mod nutrients;
mod fridge;

pub mod user_service {
    pub use super::users::*;
}

pub mod fridge_service {
    pub use super::fridge::*;
}