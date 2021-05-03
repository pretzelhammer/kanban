require('dotenv').config();

const pgp = require('pg-promise')();
const pool = pgp(process.env.DATABASE_URL);

const createBoards = `
create table if not exists boards (
    id bigserial primary key,
    name text not null,
    created_at timestamp with time zone not null default (current_timestamp at time zone 'utc')
);
`;

const seedBoards = `
insert into boards
(name)
values
('Test board 1'),
('Test board 2'),
('Test board 3');
`;

const createStatusEnum = `
create type status_enum as enum ('todo', 'doing', 'done');
`;

const createCards = `
create table if not exists cards (
    id bigserial primary key,
    board_id int not null,
    description text not null,
    status status_enum not null default 'todo',
    created_at timestamp with time zone not null default (current_timestamp at time zone 'utc'),
    constraint board_fk
        foreign key (board_id)
        references boards(id)
        on delete cascade
);
`;

const seedCards = `
insert into cards
(board_id, description, status)
values
(1, 'Test card 1', 'todo'),
(1, 'Test card 2', 'doing'),
(1, 'Test card 3', 'done'),
(2, 'Test card 4', 'todo'),
(2, 'Test card 5', 'todo'),
(3, 'Test card 6', 'done'),
(3, 'Test card 7', 'done');
`;

const createTokens = `
create table if not exists tokens (
    id text primary key,
    expired_at timestamp with time zone not null
);
`;

const seedTokens = `
insert into tokens
(id, expired_at)
values
('LET_ME_IN', (current_timestamp + interval '15 minutes') at time zone 'utc');
`;

(async () => {
    await pool.result(createBoards);
    await pool.result(seedBoards);
    await pool.result(createStatusEnum);
    await pool.result(createCards);
    await pool.result(seedCards);
    await pool.result(createTokens);
    await pool.result(seedTokens);
    pool.$pool.end();
})();
