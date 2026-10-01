-- Migration: Buchhaltungsmodul – Tabelle `buchungen` (Einnahmen & Ausgaben)
--
-- Secure by Design – Defense in Depth:
-- Die primäre Validierung erfolgt durch die Domain Primitives in `src/domain.rs`.
-- Die CHECK-Constraints hier sind eine zusätzliche Sicherheitsschicht, falls
-- jemals Daten an der Anwendung vorbei in die Datenbank gelangen.
CREATE TABLE IF NOT EXISTS buchungen (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    buchungs_typ TEXT NOT NULL CHECK(buchungs_typ IN ('einnahme', 'ausgabe')),
    betrag_cent INTEGER NOT NULL CHECK(betrag_cent > 0 AND betrag_cent <= 1000000000),
    kategorie TEXT NOT NULL CHECK(length(kategorie) BETWEEN 1 AND 100),
    beschreibung TEXT NOT NULL CHECK(length(beschreibung) <= 500),
    -- julianday() normalisiert (2026-02-30 → 2026-03-02), date() formatiert zurück:
    -- nur wenn das Ergebnis identisch ist, war es ein echtes ISO-Datum (YYYY-MM-DD).
    -- (Ein bloßes date(datum) reicht nicht: SQLite 3.44 gibt 2026-02-30 unverändert zurück.)
    datum TEXT NOT NULL CHECK(date(julianday(datum)) IS datum),
    auftrag_id INTEGER,
    beleg_referenz TEXT CHECK(beleg_referenz IS NULL OR length(beleg_referenz) BETWEEN 1 AND 100),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    created_by INTEGER NOT NULL,
    FOREIGN KEY (auftrag_id) REFERENCES auftraege(id) ON DELETE SET NULL,
    FOREIGN KEY (created_by) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_buchungen_datum ON buchungen(datum);
CREATE INDEX IF NOT EXISTS idx_buchungen_typ ON buchungen(buchungs_typ);
CREATE INDEX IF NOT EXISTS idx_buchungen_created_by ON buchungen(created_by);
