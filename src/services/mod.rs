pub mod auth;
pub mod meals;
pub mod products;
pub mod users;
pub mod sections;
pub mod nutrients;
mod fridge;

pub mod fridge_service {
    pub use super::fridge::*;
}