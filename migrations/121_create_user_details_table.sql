CREATE TABLE user_details (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    sex sex_enum NOT NULL,
    weight FLOAT4 NOT NULL,
    height INT NOT NULL,
    date_of_birth DATE NOT NULL,
    idle_activity INT NOT NULL,
    workout_activity INT NOT NULL,
    diet_kind diet_kind_enum NOT NULL
)
