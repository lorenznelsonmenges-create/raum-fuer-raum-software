-- Migration: Stundensatz für Nachbereitung in einstellungen und auftraege ergänzen
ALTER TABLE einstellungen ADD COLUMN stundensatz_nachbereitung REAL NOT NULL DEFAULT 45.0;
UPDATE einstellungen SET stundensatz_nachbereitung = COALESCE(stundensatz, 45.0);

ALTER TABLE auftraege ADD COLUMN stundensatz_nachbereitung REAL NOT NULL DEFAULT 0.0;
UPDATE auftraege SET stundensatz_nachbereitung = COALESCE(stundensatz, 0.0);
