-- Tabelle für Datei-Metadaten (Verträge, Bilder, etc.)

CREATE TABLE IF NOT EXISTS dateien (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    auftrag_id INTEGER NOT NULL,
    dateiname TEXT NOT NULL,
    dateipfad TEXT NOT NULL,
    dateityp TEXT NOT NULL,
    hochgeladen_am DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (auftrag_id) REFERENCES auftraege(id) ON DELETE CASCADE
);
