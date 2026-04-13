CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'ADMIN'
);

-- Default user for initial login (password: achtsam2024)
-- Hash generated for: achtsam2024
INSERT INTO users (username, password_hash, role)
VALUES ('admin', '$2b$12$6P29FhZ/9.uX098e9XqgHe6GkE1/yP5vU8/fO1L8f.7G.z7.0.8', 'ADMIN');
