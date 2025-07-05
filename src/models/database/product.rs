use uuid::Uuid;

pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub barcode: i64,
    pub ingredients: String,
    pub calories: i32,
    pub proteins: i32,
    pub fats: i32,
    pub carbohydrates: i32,
}