-- Migration: Beleg-Datei zu jeder Buchung (Belegpflicht)
--
-- Die Pflicht selbst wird in der Anwendung durchgesetzt (neue Buchungen und jede
-- Änderung brauchen einen Beleg). Die Spalten sind nullable, weil bereits
-- existierende Buchungen noch keinen Beleg haben.
-- CHECK-Constraints als zweite Schicht (Defense in Depth): Belege liegen nur im
-- Ordner uploads/belege/ und haben nur einen der drei erlaubten Typen.
ALTER TABLE buchungen ADD COLUMN beleg_pfad TEXT
    CHECK(beleg_pfad IS NULL OR (beleg_pfad LIKE 'uploads/belege/%' AND beleg_pfad NOT LIKE '%..%'));
ALTER TABLE buchungen ADD COLUMN beleg_dateiname TEXT
    CHECK(beleg_dateiname IS NULL OR length(beleg_dateiname) BETWEEN 1 AND 200);
ALTER TABLE buchungen ADD COLUMN beleg_typ TEXT
    CHECK(beleg_typ IS NULL OR beleg_typ IN ('application/pdf', 'image/jpeg', 'image/png'));
