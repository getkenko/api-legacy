ALTER TABLE products
ADD COLUMN unit unit_enum NOT NULL DEFAULT 'grams',
ADD COLUMN quantity INT NOT NULL CHECK(quantity >= 0)