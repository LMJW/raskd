CREATE TABLE tasks (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    tasktype TEXT NOT NULL
);

CREATE TABLE timeslots(
    id INTEGER,
    start TEXT NOT NULL,
    end TEXT,
    duration INTEGER,
    CONSTRAINT fk_id
        FOREIGN KEY (id)
        REFERENCES tasks(id)
);