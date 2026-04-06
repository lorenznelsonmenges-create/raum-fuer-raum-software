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
| `status` | `Enum` | Angefragt, Besichtigt, Durchfuehrung, Archiviert, Storniert |
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

1. [x] **Layout-Anpassung:** Der aktuell dunkelgrüne Reiter für die Rechnungserstellung (rechts) wurde als eigene Sektion unter die anderen Sektionen verschoben. Die Arbeitsstunden und Fahrten nutzen nun ein 2-Spalten-Grid zur besseren Platznutzung.
2. [x] **Bug-Fix & Edit-Logik:** Die Bearbeitung von Arbeitsstunden und Kilometern wurde korrigiert (getrennte Routen für Create/Update), sodass keine Duplikate mehr entstehen.
3. [x] **Lösch-Funktion:** Arbeitsstunden und Fahrten können nun über einen Papierkorb-Button gelöscht werden.
4. [x] **UI-Polishing:** "Abgeschlossen" wurde in "Archiviert" umbenannt. Personendaten sind nun in der Auftragsansicht ausklappbar eingebunden.
5. [ ] **Dokumenten-Feedback:** Nach dem Hochladen eines erforderlichen Dokuments (z.B. Vertrag) soll dies visuell deutlicher hervorgehoben werden (der aktuelle Upload-Prozess ist noch etwas kontraintuitiv).
6. [ ] **PDF-Rechnungserstellung:** Stunden, Kilometer und Preise zusammenfassen und als PDF generieren.
7. [ ] **Frontend:** Weiterer Ausbau der Admin-UI.
8. [ ] **Login:** Absicherung der API.

## 5. Betriebliche Hinweise

- **Hosting:** Geplant auf Hetzner-Server via Git-Deployment.
- **Email:** Finalisierung der Adressen (Platzhalter: `hallo@achtsam-entruempeln.de`).
- **Entwicklung:** `target/` und `achtsam.db*` werden ignoriert, um Tokens und Kontext zu sparen.
