-- Migration: Sicherstellen, dass die Tabelle 'users' existiert und 'admin' sowie 'stefanie' mit validen Bcrypt-Hashes hinterlegt sind.
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'ADMIN'
);

-- admin (Passwort: achtsam2024)
INSERT INTO users (username, password_hash, role)
VALUES ('admin', '$2b$12$s4nMxf4upx/0DKHOmhaU7OPAV5COxzxrFzZrrpMrtSuoQjVquHhDC', 'ADMIN')
ON CONFLICT(username) DO UPDATE SET password_hash = excluded.password_hash;

-- stefanie (Passwort: stefanie2026!)
INSERT INTO users (username, password_hash, role)
VALUES ('stefanie', '$2b$12$9d6Dnia1UiZWxh.gCh7kWes4aKswOazaXfeUGSq5/RPppGrnUyb.e', 'ADMIN')
ON CONFLICT(username) DO UPDATE SET password_hash = excluded.password_hash;
