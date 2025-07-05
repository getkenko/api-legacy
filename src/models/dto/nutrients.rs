use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateMacrosDistribution {
    pub protein: i32,
    pub fat: i32,
    pub carb: i32,
}

#[derive(Deserialize)]
pub struct UpdateMacrosTarget {
    pub protein: i32,
    pub fat: i32,
    pub carb: i32,
}