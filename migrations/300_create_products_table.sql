CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    barcode BIGINT NOT NULL,
    brand TEXT,
    ingredients TEXT,
    unit unit_enum NOT NULL DEFAULT 'grams',
    quantity INT NOT NULL CHECK (quantity >= 0),
    calories INT NOT NULL CHECK (calories >= 0),
    proteins INT NOT NULL CHECK (proteins >= 0),
    fats INT NOT NULL CHECK (fats >= 0),
    carbohydrates INT NOT NULl CHECK (carbohydrates >= 0),

    search_vector tsvector GENERATED ALWAYS AS (to_tsvector('english', coalesce(name, '') || ' ' || coalesce(ingredients, ''))) STORED
)
