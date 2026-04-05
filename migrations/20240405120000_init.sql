-- Initialisierung der Tabellen für Achtsam Entrümpeln

CREATE TABLE IF NOT EXISTS kunden (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    vorname TEXT NOT NULL,
    nachname TEXT NOT NULL,
    strasse TEXT,
    hausnummer TEXT,
    plz TEXT,
    stadt TEXT,
    email TEXT,
    telefon TEXT,
    notizen TEXT
);

CREATE TABLE IF NOT EXISTS auftraege (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    kunde_id INTEGER NOT NULL,
    status TEXT NOT NULL,
    beschreibung TEXT,
    basis_pauschale REAL, -- Optional
    preis_manuell REAL,    -- Neu: Manuelle Preisanpassung
    notizen TEXT,
    FOREIGN KEY (kunde_id) REFERENCES kunden(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS rechnungs_notizen (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    auftrag_id INTEGER NOT NULL,
    text TEXT NOT NULL,
    auf_rechnung BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (auftrag_id) REFERENCES auftraege(id) ON DELETE CASCADE
);
