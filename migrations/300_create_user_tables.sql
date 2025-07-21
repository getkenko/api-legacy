CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username CITEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    email CITEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    avatar_url TEXT,
    account_state account_state_enum NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_details (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    sex sex_enum NOT NULL,
    weight FLOAT4 NOT NULL,
    height INT NOT NULL,
    date_of_birth DATE NOT NULL,
    idle_activity INT NOT NULL,
    workout_activity INT NOT NULL,
    diet_kind diet_kind_enum NOT NULL
);

CREATE TABLE user_preferences (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    theme theme_enum NOT NULL DEFAULT 'system',
    language language_enum NOT NULL DEFAULT 'en',
    weight_unit weight_unit_enum NOT NULL,
    height_unit height_unit_enum NOT NULL
);

CREATE TABLE user_goals (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    weight_goal weight_goal_enum NOT NULL,
    goal_diff_per_week FLOAT4 NOT NULL
);

-- or user_body_metrics?
CREATE TABLE user_nutrients (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    bmr FLOAT4 NOT NULL,
    base_tdee FLOAT4 NOT NULL,
    tdee FLOAT4 NOT NULL,
    -- target macros that user needs to hit everyday
    protein_target INT NOT NULL,
    fat_target INT NOT NULL,
    carb_target INT NOT NULL,
    -- macros distribution
    protein_dist INT,
    fat_dist INT,
    carb_dist INT
);

CREATE TABLE user_metrics (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    origin user_origin_enum NOT NULL
);

CREATE TABLE user_meal_sections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    index INT NOT NULL,
    label TEXT NOT NULL,

    UNIQUE(user_id, index) DEFERRABLE INITIALLY DEFERRED
);

CREATE TABLE user_meals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    section_id UUID NOT NULL REFERENCES user_meal_sections(id) ON DELETE CASCADE,
    date DATE NOT NULL
);

