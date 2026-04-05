-- Hinzufügen der Tabelle für Arbeitseinsätze (Stunden, Kilometer, Notizen)

CREATE TABLE IF NOT EXISTS einsaetze (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    auftrag_id INTEGER NOT NULL,
    datum TEXT NOT NULL,
    kilometer REAL NOT NULL DEFAULT 0.0,
    stunden REAL NOT NULL DEFAULT 0.0,
    notiz TEXT,
    FOREIGN KEY (auftrag_id) REFERENCES auftraege(id) ON DELETE CASCADE
);

-- Hinweis: Die Spalte 'termin' in 'auftraege' wird nicht mehr genutzt.
-- In SQLite löschen wir sie nicht zwingend sofort, um die Migration einfach zu halten,
-- ignorieren sie aber im Rust-Code.
