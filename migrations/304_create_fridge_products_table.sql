CREATE TABLE user_fridge_products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    quantity INT NOT NULL CHECK (quantity >= 0),
    expiration DATE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)