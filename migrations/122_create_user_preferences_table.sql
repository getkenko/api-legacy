CREATE TABLE user_preferences (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    theme theme_enum NOT NULL DEFAULT 'dark',
    language language_enum NOT NULL DEFAULT 'english'
)
