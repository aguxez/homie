CREATE TABLE client_keys (
    id INTEGER NOT NULL PRIMARY KEY,
    key TEXT NOT NULL,
    is_active INTEGER NOT NULL CHECK (is_active IN (0, 1))
);