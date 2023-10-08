CREATE TABLE event_handlers (
    slug VARCHAR NOT NULL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    python_code TEXT NOT NULL
);
