CREATE TABLE user_meal_sections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    index INT NOT NULL,
    label TEXT NOT NULL,

    UNIQUE(user_id, index)
)
