CREATE TABLE user_preferences (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    theme theme_enum NOT NULL DEFAULT 'system',
    language language_enum NOT NULL DEFAULT 'en',
    weight_unit weight_unit_enum NOT NULL,
    height_unit height_unit_enum NOT NULL
)
