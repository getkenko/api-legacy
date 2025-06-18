CREATE TABLE user_details (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    is_male BOOL NOT NULL,
    weight FLOAT4 NOT NULL,
    height INTEGER NOT NULL,
    date_of_birth DATE NOT NULL
)
