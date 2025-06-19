CREATE TABLE meal_items (
    id SERIAL PRIMARY KEY,
    meal_id UUID NOT NULL REFERENCES user_meals(id) ON DELETE CASCADE,
    meal_product_id UUID NOT NULL REFERENCES meal_products(id) ON DELETE CASCADE,
    quantity INT NOT NULL CHECK (quantity > 0)
)
