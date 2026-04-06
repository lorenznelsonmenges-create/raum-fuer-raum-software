# Software-Projekt: Achtsam Entrümpeln (Backend)

Dieses Dokument dient als zentrale Wissensbasis ("Source of Truth") für die Architektur, das Datenmodell und den Entwicklungsfortschritt der Software.

## 1. Architektur & Design-Entscheidungen

- **Backend:** Rust mit dem **Axum** Web-Framework (Port 3000).
- **Datenbank:** **SQLite** (`achtsam.db`), asynchron angebunden via `sqlx`.
- **Migrationen:** Automatische Migrationen beim Start der Anwendung (Ordner `migrations/`).
- **Error-Handling:** Zentralisiertes System in `src/error.rs` für konsistente HTTP-Statuscodes.
- **Datei-Management:** Uploads landen im Ordner `uploads/` auf der Festplatte; Pfade werden in der DB gespeichert.
- **Frontend-Anbindung:** Aktuell als reine REST-API konzipiert (bereit für Nginx/HTTPS-Reverse-Proxy).

## 2. Datenmodell (Source of Truth)

Das Datenmodell ist in `src/models.rs` definiert und wird durch SQLite-Tabellen persistiert.

### Kunde
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `i64` | Primärschlüssel (Auto-Increment) |
| `vorname` | `String` | Vorname |
| `nachname` | `String` | Nachname |
| `strasse` | `String` | Straße |
| `hausnummer`| `String` | Hausnummer |
| `plz` | `String` | Postleitzahl |
| `ort` | `String` | Stadt/Ort |
| `email` | `String` | E-Mail Adresse |
| `telefon` | `String` | Telefonnummer |
| `notizen` | `String` | Interne Kundennotizen |

### Auftrag
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `i64` | Primärschlüssel |
| `kunde_id` | `i64` | Fremdschlüssel auf `kunden` |
| `status` | `Enum` | Angefragt, Besichtigt, Durchfuehrung, Abgeschlossen, Storniert |
| `beschreibung`| `String` | Kurzbeschreibung des Auftrags |
| `basis_pauschale`| `Option<f64>`| Optionale Fixkosten-Pauschale |
| `preis_manuell`| `Option<f64>`| Manuelle Preisanpassung (Überschreibt ggf. Logik) |
| `notizen` | `String` | Interne Auftragsnotizen |

### Einsatz (Arbeitszeit & Fahrtkosten)
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `i64` | Primärschlüssel |
| `auftrag_id` | `i64` | Fremdschlüssel auf `auftraege` |
| `datum` | `String` | Datum des Einsatzes |
| `kilometer` | `f64` | Gefahrene Kilometer |
| `stunden` | `f64` | Gearbeitete Stunden |
| `notiz` | `String` | Notiz zum Einsatz |

### Datei (Uploads)
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `i64` | Primärschlüssel |
| `auftrag_id` | `i64` | Fremdschlüssel auf `auftraege` |
| `dateiname` | `String` | Originaler Name der Datei |
| `dateipfad` | `String` | Relativer Pfad im `uploads/` Ordner |
| `dateityp` | `String` | MIME-Type oder Endung |
| `hochgeladen_am`| `String` | Zeitstempel des Uploads |

### RechnungsNotiz
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `i64` | Primärschlüssel |
| `auftrag_id` | `i64` | Fremdschlüssel auf `auftraege` |
| `text` | `String` | Inhalt der Notiz |
| `auf_rechnung`| `bool` | Haken: Erscheint diese Notiz auf der finalen PDF-Rechnung? |

## 3. Status der API-Endpunkte

- [x] **Kunden:** CRUD-Operationen (Erstellen, Lesen, Liste, Update).
- [x] **Aufträge:** Erstellung, Status-Management und Update.
- [x] **Einsätze:** Dokumentation von Stunden/Kilometern.
- [x] **Uploads:** Multipart-Form Upload für Dokumente/Bilder.

## 4. Nächste Schritte

1. **Frontend:** Aufbau einer Admin-UI (Web-Interface).
2. **Login:** Absicherung der API.
3. **PDF-Export:** Automatisierte Rechnungserstellung (Aggregiert Stunden & Kilometer).

## 5. Betriebliche Hinweise

- **Hosting:** Geplant auf Hetzner-Server via Git-Deployment.
- **Email:** Finalisierung der Adressen (Platzhalter: `hallo@achtsam-entruempeln.de`).
- **Entwicklung:** `target/` und `achtsam.db*` werden ignoriert, um Tokens und Kontext zu sparen.
