CREATE TABLE meal_products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type meal_product_type_enum NOT NULL,

    -- from_database variant
    product_id UUID REFERENCES products(id) ON DELETE CASCADE,

    -- quick_add variant
    label TEXT,
    calories INT,
    proteins INT,
    fats INT,
    carbohydrates INT
)
