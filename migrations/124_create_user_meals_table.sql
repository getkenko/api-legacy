CREATE TABLE user_meals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    section_id UUID NOT NULL REFERENCES user_meal_sections(id) ON DELETE CASCADE,
    date DATE NOT NULL
)
