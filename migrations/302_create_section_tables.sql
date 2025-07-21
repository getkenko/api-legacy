-- create schemas
CREATE TABLE section_icons (
    id SERIAL PRIMARY KEY,
    emoji TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_sections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    icon_id INT,
    index INT NOT NULL,
    name TEXT NOT NULL,

    UNIQUE(user_id, index) DEFERRABLE INITIALLY DEFERRED
);

-- insert default icons
INSERT INTO section_icons (emoji) VALUES
('🍫'),('🍩'),('🍿'),('🍬'),('☕'),('🥗'),('🍔'),
('🌭'),('🍓'),('🍎'),('🥥'),('🍇'),('🍞'),('🧇'),
('🥘'),('🍝'),('🍲'),('🍛'),('🌮'),('🍗'),('🥩'),
('🥚'),('🍳');
