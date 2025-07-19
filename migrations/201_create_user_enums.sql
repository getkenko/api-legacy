-- users
CREATE TYPE account_state_enum AS ENUM
('active', 'suspended', 'deleted', 'inactive');



-- user_details
CREATE TYPE sex_enum AS ENUM
('male', 'female');

CREATE TYPE diet_kind_enum AS ENUM
('vegetarian', 'vegan', 'pescatarian', 'ketogenic', 'classic');



-- user_preferences
CREATE TYPE theme_enum AS ENUM
('system', 'dark', 'light');

CREATE TYPE language_enum AS ENUM
('en', 'pl');

CREATE TYPE weight_unit_enum AS ENUM
('kg', 'lb');

CREATE TYPE height_unit_enum AS ENUM
('cm', 'ft_in');



-- user_goals
CREATE TYPE weight_goal_enum AS ENUM
('gain', 'lose', 'maintain');



-- user_metrics
CREATE TYPE user_origin_enum AS ENUM
('instagram', 'tiktok', 'x', 'twitch', 'facebook', 'youtube', 'other');