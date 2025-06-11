CREATE TABLE users_deletion_history (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    reason TEXT,
    date TIMESTAMPTZ NOT NULL DEFAULT NOW()
)