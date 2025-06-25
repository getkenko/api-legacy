CREATE TABLE user_goals (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    weight_goal weight_goal_enum NOT NULL,
    goal_diff_per_week FLOAT4 NOT NULL
)