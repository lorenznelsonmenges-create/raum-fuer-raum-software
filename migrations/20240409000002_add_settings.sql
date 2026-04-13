CREATE TABLE einstellungen (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    stundensatz REAL NOT NULL DEFAULT 45.0,
    kilometer_satz REAL NOT NULL DEFAULT 0.5
);

INSERT INTO einstellungen (id, stundensatz, kilometer_satz) VALUES (1, 45.0, 0.5);
