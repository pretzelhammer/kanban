-- create_boards.sql
CREATE TABLE IF NOT EXISTS boards (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'utc')
);

-- seed db with some dummy data
INSERT INTO boards
(name)
VALUES
('Test board 1'),
('Test board 2'),
('Test board 3');
