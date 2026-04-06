-- Migration: Rechnungen-Tabelle hinzufügen
CREATE TABLE IF NOT EXISTS rechnungen (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    auftrag_id INTEGER NOT NULL,
    rechnungs_nummer TEXT NOT NULL UNIQUE,
    datum TEXT NOT NULL,
    gesamt_netto REAL NOT NULL DEFAULT 0.0,
    gesamt_brutto REAL NOT NULL DEFAULT 0.0,
    pdf_pfad TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'OFFEN',
    FOREIGN KEY (auftrag_id) REFERENCES auftraege(id) ON DELETE CASCADE
);
