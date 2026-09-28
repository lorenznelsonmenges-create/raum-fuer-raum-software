-- Migration: Kategorien auf Dienstleistung & Nachbereitung umstellen, Unterkategorie ergänzen
ALTER TABLE einsaetze ADD COLUMN unterkategorie TEXT;

-- Vorhandene Einträge aktualisieren
UPDATE einsaetze SET typ = 'DIENSTLEISTUNG' WHERE typ = 'ARBEIT_VOR_ORT' OR typ = 'ARBEIT';
UPDATE einsaetze SET typ = 'NACHBEREITUNG' WHERE typ = 'ARBEIT_VORBEREITUNG';
