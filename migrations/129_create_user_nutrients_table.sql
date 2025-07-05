-- swaglord: or user_body_metrics?
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
)