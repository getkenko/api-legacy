CREATE TABLE user_metrics (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    origin user_origin_enum NOT NULL
)