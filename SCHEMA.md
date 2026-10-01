# Datenbank-Schema (Source of Truth)

Diese Datei beschreibt den aktuellen Stand der SQLite-Datenbank (`achtsam.db`), wie er sich aus allen Dateien in `migrations/` ergibt (Stand: Migration `20261002000000_add_beleg_zu_buchungen.sql`).
Alle Änderungen MUSS ein Agent über neue `.sql`-Dateien in `migrations/` vornehmen und danach dieses Dokument aktualisieren.

**Konventionen**
- Fremdschlüssel sind aktiv (`foreign_keys(true)` in `database.rs`).
- Geldbeträge: Alt-Tabellen speichern `REAL` in Euro, `buchungen` speichert `INTEGER` in Cent. Im Rust-Code ist beides der Domain Primitive `Euro(i64)` (Cent).
- Die Session-Tabelle von `tower-sessions-sqlx-store` wird beim Start separat angelegt (`session_store.migrate()`) und ist hier nicht aufgeführt.

## Tabellenübersicht

### kunden
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `INTEGER` | Primärschlüssel (Auto-Increment) |
| `vorname` | `TEXT NOT NULL` | Vorname |
| `nachname` | `TEXT NOT NULL` | Nachname |
| `strasse` | `TEXT` | Straße |
| `hausnummer` | `TEXT` | Hausnummer |
| `plz` | `TEXT` | Postleitzahl |
| `ort` | `TEXT` | Stadt/Ort (vorher: `stadt`) |
| `email` | `TEXT` | E-Mail-Adresse |
| `telefon` | `TEXT` | Telefonnummer |
| `notizen` | `TEXT` | Interne Kundennotizen |

### auftraege
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `INTEGER` | Primärschlüssel |
| `kunde_id` | `INTEGER NOT NULL` | FK → `kunden(id)`, `ON DELETE CASCADE` |
| `status` | `TEXT NOT NULL` | `AnfrageLaeuft`, `InBearbeitung`, `Abgeschlossen`, `Storniert` |
| `beschreibung` | `TEXT` | Kurzbeschreibung |
| `basis_pauschale` | `REAL` | Optionale Fixkosten-Pauschale (Euro) |
| `preis_manuell` | `REAL` | **Veraltet** – existiert noch, wird im Code nicht mehr genutzt |
| `notizen` | `TEXT` | Interne Auftragsnotizen |
| `stundensatz` | `REAL` | Stundensatz Dienstleistung (Default 45.0) |
| `kilometer_satz` | `REAL` | Kilometersatz (Default 0.50) |
| `created_by` | `TEXT` | Benutzername des Erstellers |
| `stundensatz_nachbereitung` | `REAL NOT NULL` | Stundensatz Nachbereitung (Default 0.0 → Code fällt auf `stundensatz` zurück) |

### einsaetze
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `INTEGER` | Primärschlüssel |
| `auftrag_id` | `INTEGER NOT NULL` | FK → `auftraege(id)`, `ON DELETE CASCADE` |
| `datum` | `TEXT NOT NULL` | Datum des Einsatzes |
| `kilometer` | `REAL NOT NULL` | Gefahrene Kilometer (Default 0.0) |
| `stunden` | `REAL NOT NULL` | Gearbeitete Stunden (Default 0.0) |
| `notiz` | `TEXT` | Interne Notiz |
| `typ` | `TEXT NOT NULL` | `DIENSTLEISTUNG`, `NACHBEREITUNG` oder `KILOMETER` (Altwerte werden im Code gemappt) |
| `signatur_pfad` | `TEXT` | Pfad zum Signaturbild |
| `unterkategorie` | `TEXT` | z.B. Praktische Unterstützung, Beratung, Mediation, Auswertung, Verteilung |

### dateien
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `INTEGER` | Primärschlüssel |
| `auftrag_id` | `INTEGER NOT NULL` | FK → `auftraege(id)`, `ON DELETE CASCADE` |
| `dateiname` | `TEXT NOT NULL` | Originalname |
| `dateipfad` | `TEXT NOT NULL` | Relativer Pfad in `uploads/` |
| `dateityp` | `TEXT NOT NULL` | MIME-Type |
| `hochgeladen_am` | `DATETIME` | Zeitstempel (Default `CURRENT_TIMESTAMP`) |
| `kategorie` | `TEXT NOT NULL` | `DATENSCHUTZ`, `VERTRAG`, `SONSTIGES` (Default), `SIGNATUR`, `RECHNUNG` |

### rechnungen
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `INTEGER` | Primärschlüssel |
| `auftrag_id` | `INTEGER NOT NULL` | FK → `auftraege(id)`, `ON DELETE CASCADE` |
| `rechnungs_nummer` | `TEXT NOT NULL UNIQUE` | Format `R` + 6 Ziffern, fortlaufend |
| `datum` | `TEXT NOT NULL` | Datum der Erstellung |
| `gesamt_netto` | `REAL NOT NULL` | Summe Netto (Euro) |
| `gesamt_brutto` | `REAL NOT NULL` | Summe Brutto (Euro) |
| `pdf_pfad` | `TEXT NOT NULL` | Pfad zur generierten PDF |
| `status` | `TEXT NOT NULL` | Default `OFFEN` (Code setzt `Offen`) |

### rechnungs_notizen
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `INTEGER` | Primärschlüssel |
| `auftrag_id` | `INTEGER NOT NULL` | FK → `auftraege(id)`, `ON DELETE CASCADE` |
| `text` | `TEXT NOT NULL` | Textinhalt |
| `auf_rechnung` | `BOOLEAN NOT NULL` | Soll auf dem PDF erscheinen (0 oder 1, Default 0) |

### einstellungen
Singleton-Tabelle (`CHECK (id = 1)`).
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `INTEGER` | Primärschlüssel, immer 1 |
| `stundensatz` | `REAL NOT NULL` | Standard-Stundensatz Dienstleistung (Default 45.0) |
| `kilometer_satz` | `REAL NOT NULL` | Standard-Kilometersatz (Default 0.5) |
| `stundensatz_nachbereitung` | `REAL NOT NULL` | Standard-Stundensatz Nachbereitung (Default 45.0) |

### users
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `INTEGER` | Primärschlüssel |
| `username` | `TEXT NOT NULL UNIQUE` | Benutzername (`admin`, `stefanie`) |
| `password_hash` | `TEXT NOT NULL` | Bcrypt-Hash |
| `role` | `TEXT NOT NULL` | Default `ADMIN` (wird nicht ausgewertet) |

### buchungen (Buchhaltung)
Primäre Validierung über Domain Primitives in `src/domain.rs`; die CHECK-Constraints sind eine zweite Schicht (Defense in Depth). Beleg-Spalten sind nullable (Altbuchungen); die **Belegpflicht** setzt die Anwendung durch.
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `INTEGER` | Primärschlüssel (Auto-Increment) |
| `buchungs_typ` | `TEXT NOT NULL` | `CHECK IN ('einnahme', 'ausgabe')` |
| `betrag_cent` | `INTEGER NOT NULL` | Betrag in **Cent**, `CHECK > 0 AND <= 1000000000` (10 Mio. €) |
| `kategorie` | `TEXT NOT NULL` | `CHECK length 1–100` |
| `beschreibung` | `TEXT NOT NULL` | `CHECK length <= 500` (darf leer sein) |
| `datum` | `TEXT NOT NULL` | ISO `YYYY-MM-DD`, `CHECK(date(julianday(datum)) IS datum)` – nur real existierende Daten |
| `auftrag_id` | `INTEGER` | FK → `auftraege(id)`, `ON DELETE SET NULL` |
| `beleg_referenz` | `TEXT` | Belegnummer, `CHECK NULL oder length 1–100` |
| `beleg_pfad` | `TEXT` | Beleg-Datei `uploads/belege/<uuid>.<pdf\|jpg\|png>`, `CHECK LIKE 'uploads/belege/%'`, kein `..` |
| `beleg_dateiname` | `TEXT` | Ursprünglicher Dateiname (nur Anzeige), `CHECK length 1–200` |
| `beleg_typ` | `TEXT` | `CHECK IN ('application/pdf', 'image/jpeg', 'image/png')` |
| `created_at` | `TEXT NOT NULL` | Default `datetime('now')` (UTC) |
| `created_by` | `INTEGER NOT NULL` | FK → `users(id)`, serverseitig aus der Session gesetzt |

## Indizes
| Index | Tabelle(Spalte) |
| :--- | :--- |
| `idx_auftraege_kunde_id` | `auftraege(kunde_id)` |
| `idx_auftraege_status` | `auftraege(status)` |
| `idx_einsaetze_auftrag_id` | `einsaetze(auftrag_id)` |
| `idx_dateien_auftrag_id` | `dateien(auftrag_id)` |
| `idx_rechnungen_auftrag_id` | `rechnungen(auftrag_id)` |
| `idx_buchungen_datum` | `buchungen(datum)` |
| `idx_buchungen_typ` | `buchungen(buchungs_typ)` |
| `idx_buchungen_created_by` | `buchungen(created_by)` |
