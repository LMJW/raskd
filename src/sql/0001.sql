CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    tasktype TEXT NOT NULL,
    start TEXT NOT NULL,
    stop TEXT,
    --- after pause and restart, we will store another entry of the same task
    --  but with increment of parts
    parts INTEGER NOT NULL
);