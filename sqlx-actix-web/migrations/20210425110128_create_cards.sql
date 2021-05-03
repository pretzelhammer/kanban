-- create_cards.sql
CREATE TYPE STATUS AS ENUM ('todo', 'doing', 'done');

CREATE TABLE IF NOT EXISTS cards (
    id BIGSERIAL PRIMARY KEY,
    board_id BIGINT NOT NULL,
    description TEXT NOT NULL,
    status STATUS NOT NULL DEFAULT 'todo',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (CURRENT_TIMESTAMP AT TIME ZONE 'utc'),
    CONSTRAINT board_fk
        FOREIGN KEY (board_id)
        REFERENCES boards(id)
        ON DELETE CASCADE
);

-- seed db with some dummy data
INSERT INTO cards
(board_id, description, status)
VALUES
(1, 'Test card 1', 'todo'),
(1, 'Test card 2', 'doing'),
(1, 'Test card 3', 'done'),
(2, 'Test card 4', 'todo'),
(2, 'Test card 5', 'todo'),
(3, 'Test card 6', 'done'),
(3, 'Test card 7', 'done');