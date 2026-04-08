# Datenbank-Schema (Source of Truth)

Diese Datei beschreibt den aktuellen Stand der SQLite-Datenbank (`achtsam.db`). 
Alle Änderungen MUSS ein Agent über neue `.sql`-Dateien in `migrations/` vornehmen und danach dieses Dokument aktualisieren.

## Tabellenübersicht

### kunden
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `INTEGER` | Primärschlüssel (Auto-Increment) |
| `vorname` | `TEXT` | Vorname (Pflicht) |
| `nachname` | `TEXT` | Nachname (Pflicht) |
| `strasse` | `TEXT` | Straße |
| `hausnummer` | `TEXT` | Hausnummer |
| `plz` | `TEXT` | Postleitzahl |
| `ort` | `TEXT` | Stadt/Ort (vorher: `stadt`) |
| `email` | `TEXT` | E-Mail Adresse |
| `telefon` | `TEXT` | Telefonnummer |
| `notizen` | `TEXT` | Interne Kundennotizen |

### auftraege
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `INTEGER` | Primärschlüssel |
| `kunde_id` | `INTEGER` | Fremdschlüssel auf `kunden` (Cascade Delete) |
| `status` | `TEXT` | Status (Enum-like: AnfrageLaeuft, InBearbeitung...) |
| `beschreibung` | `TEXT` | Kurzbeschreibung |
| `basis_pauschale` | `REAL` | Optionale Fixkosten-Pauschale |
| `stundensatz` | `REAL` | Aktueller Stundensatz (Default: 45.0) |
| `kilometer_satz` | `REAL` | Aktueller Kilometersatz (Default: 0.5) |
| `notizen` | `TEXT` | Interne Auftragsnotizen |

### einsaetze
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `INTEGER` | Primärschlüssel |
| `auftrag_id` | `INTEGER` | Fremdschlüssel auf `auftraege` |
| `datum` | `TEXT` | Datum des Einsatzes |
| `kilometer` | `REAL` | Gefahrene Kilometer |
| `stunden` | `REAL` | Gearbeitete Stunden |
| `notiz` | `TEXT` | Notiz zum Einsatz |
| `typ` | `TEXT` | `ARBEIT` oder `FAHRT` |
| `signatur_pfad` | `TEXT` | Pfad zum Signaturbild |

### dateien
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `INTEGER` | Primärschlüssel |
| `auftrag_id` | `INTEGER` | Fremdschlüssel auf `auftraege` |
| `dateiname` | `TEXT` | Originalname |
| `dateipfad` | `TEXT` | Relativer Pfad in `uploads/` |
| `dateityp` | `TEXT` | MIME-Type |
| `hochgeladen_am` | `DATETIME` | Zeitstempel (Default: CURRENT_TIMESTAMP) |
| `kategorie` | `TEXT` | DATENSCHUTZ, VERTRAG, SONSTIGES, etc. |

### rechnungen
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `INTEGER` | Primärschlüssel |
| `auftrag_id` | `INTEGER` | Fremdschlüssel auf `auftraege` |
| `rechnungs_nummer` | `TEXT` | Eindeutige Rechnungsnummer (Unique) |
| `datum` | `TEXT` | Datum der Erstellung |
| `gesamt_netto` | `REAL` | Summe Netto |
| `gesamt_brutto` | `REAL` | Summe Brutto |
| `pdf_pfad` | `TEXT` | Pfad zur generierten PDF |
| `status` | `TEXT` | OFFEN, BEZAHLT, etc. |

### rechnungs_notizen
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `INTEGER` | Primärschlüssel |
| `auftrag_id` | `INTEGER` | Fremdschlüssel auf `auftraege` |
| `text` | `TEXT` | Textinhalt |
| `auf_rechnung` | `BOOLEAN` | Soll auf dem PDF erscheinen (0 oder 1) |
