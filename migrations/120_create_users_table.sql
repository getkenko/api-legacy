CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username CITEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    email CITEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    is_male BOOL NOT NULL,
    date_of_birth DATE NOT NULL,
    avatar_url TEXT,
    account_state account_state_enum NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)
