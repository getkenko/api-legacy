CREATE TABLE meal_products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    meal_id UUID NOT NULL REFERENCES user_meals(id) ON DELETE CASCADE,

    kind meal_product_kind_enum NOT NULL,
    quantity INT NOT NULL CHECK (quantity >= 0),

    -- from_database variant
    product_id UUID REFERENCES products(id) ON DELETE CASCADE,

    -- quick_add variant
    unit unit_enum,
    name TEXT,
    calories INT CHECK (calories >= 0),
    proteins INT CHECK (proteins >= 0),
    fats INT CHECK (fats >= 0),
    carbohydrates INT CHECK (carbohydrates >= 0),

    CHECK (
        (
            kind = 'from_database'
            AND product_id IS NOT NULL
        )
        OR
        (
            kind = 'quick_add'
            AND name IS NOT NULL AND calories IS NOT NULL
            AND proteins IS NOT NULL AND fats IS NOT NULL
            AND carbohydrates IS NOT NULL AND unit IS NOT NULL
        )
    )
)
