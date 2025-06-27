CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    barcode INT NOT NULL,
    ingredients TEXT NOT NULL,
    calories INT NOT NULL CHECK (calories >= 0),
    proteins INT NOT NULL CHECK (proteins >= 0),
    fats INT NOT NULL CHECK (fats >= 0),
    carbohydrates INT NOT NULl CHECK (carbohydrates >= 0),

    search_vector tsvector -- so i can work on searching products without having to migrate data yet
)
