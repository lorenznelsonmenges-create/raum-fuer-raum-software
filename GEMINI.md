# Software-Projekt: Achtsam Entrümpeln (Backend)

**GESCHÜTZTE DATEI:** Diese Datei darf ausschließlich geändert werden,
> wenn der Nutzer dies in der aktuellen Konversation explizit erlaubt hat.
> Kein Agent darf diese Datei eigenständig bearbeiten.

Dieses Dokument dient als zentrale Wissensbasis ("Source of Truth") für die Architektur, das Datenmodell und den Entwicklungsfortschritt der Software.

## 1. Architektur & Design-Entscheidungen

- **Backend:** Rust mit dem **Axum** Web-Framework (Port 3000).
- **Datenbank:** **SQLite** (`achtsam.db`), asynchron angebunden via `sqlx`.
- **Migrationen:** Automatische Migrationen beim Start der Anwendung (Ordner `migrations/`).
- **Error-Handling:** Zentralisiertes System in `src/error.rs` für konsistente HTTP-Statuscodes.
- **Datei-Management:** Uploads landen im Ordner `uploads/` auf der Festplatte; Pfade werden in der DB gespeichert.
- **Frontend-Anbindung:** Aktuell als reine REST-API konzipiert (bereit für Nginx/HTTPS-Reverse-Proxy).
- **Templating:** Handlebars 6.x (nicht Tera – trotz anderslautender älterer Notizen).
- **PDF-Generierung:** `headless_chrome` 1.x via HTML → PDF.

## 2. Datenmodell (Source of Truth)

⚠️ **SYNCHRONISIERUNGS-PFLICHT:** Dieses Datenmodell MUSS mit `src/models.rs`
übereinstimmen. Wenn ein Agent `src/models.rs` ändert (Felder hinzufügt,
entfernt oder umbenennt), MUSS er danach mit expliziter Nutzer-Erlaubnis
diesen Abschnitt aktualisieren. Kein Merge ohne aktuelle Doku.

### Kunde
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `i64` | Primärschlüssel (Auto-Increment) |
| `vorname` | `String` | Vorname (Pflichtfeld) |
| `nachname` | `String` | Nachname (Pflichtfeld) |
| `strasse` | `Option<String>` | Straße |
| `hausnummer` | `Option<String>` | Hausnummer |
| `plz` | `Option<String>` | Postleitzahl |
| `ort` | `Option<String>` | Stadt/Ort |
| `email` | `Option<String>` | E-Mail Adresse |
| `telefon` | `Option<String>` | Telefonnummer |
| `notizen` | `Option<String>` | Interne Kundennotizen |

### Auftrag
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `i64` | Primärschlüssel |
| `kunde_id` | `i64` | Fremdschlüssel auf `kunden` |
| `status` | `Enum` | AnfrageLaeuft, InBearbeitung, Abgeschlossen, Storniert |
| `beschreibung` | `String` | Kurzbeschreibung des Auftrags |
| `basis_pauschale` | `Option<f64>` | Optionale Fixkosten-Pauschale |
| `stundensatz` | `f64` | Stundensatz (Default: 45.00) |
| `kilometer_satz` | `f64` | Kilometersatz (Default: 0.50) |
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
| `typ` | `String` | ARBEIT oder FAHRT |
| `signatur_pfad` | `Option<String>` | Pfad zum Signaturbild (digital vor Ort) |

### Datei (Uploads)
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `i64` | Primärschlüssel |
| `auftrag_id` | `i64` | Fremdschlüssel auf `auftraege` |
| `dateiname` | `String` | Originaler Name der Datei |
| `dateipfad` | `String` | Relativer Pfad im `uploads/` Ordner |
| `dateityp` | `String` | MIME-Type oder Endung |
| `hochgeladen_am` | `String` | Zeitstempel des Uploads |
| `kategorie` | `String` | DATENSCHUTZ, VERTRAG, SONSTIGES, SIGNATUR, RECHNUNG |

### RechnungNotiz
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `i64` | Primärschlüssel |
| `auftrag_id` | `i64` | Fremdschlüssel auf `auftraege` |
| `text` | `String` | Inhalt der Notiz |
| `auf_rechnung` | `bool` | Haken: Erscheint diese Notiz auf der finalen PDF-Rechnung? |

## 3. Status der API-Endpunkte

- [x] **Kunden:** CRUD-Operationen (Erstellen, Lesen, Liste, Update, Löschen).
- [x] **Aufträge:** Erstellung, Status-Management und Update.
- [x] **Einsätze:** Dokumentation von Stunden/Kilometern + Digitale Signatur.
- [x] **Uploads:** Multipart-Form Upload für Dokumente/Bilder + Drag & Drop Support.
- [x] **Email:** Platzhalter-Endpunkt für den Stundennachweis-Versand.

## 4. Nächste Schritte

1. [ ] **Dokumenten-Feedback:** Visuelle Hervorhebung nach erfolgreichem Upload.
2. [ ] **PDF-Rechnungserstellung:** Finalisierung des Designs und Einbindung der Vorlagen.
3. [ ] **Frontend:** Weiterer Ausbau der Admin-UI.
4. [ ] **Login:** Absicherung der API.
5. [ ] **Testing:** Einführung automatisierter Tests (cargo test).

## 5. Betriebliche Hinweise

- **Hosting:** Geplant auf Hetzner-Server via Git-Deployment.
- **Email:** Finalisierung der Adressen (Platzhalter: `hallo@achtsam-entruempeln.de`).
- **Entwicklung:** `target/`, `achtsam.db*`, `uploads/` und `Cargo.lock` werden ignoriert.
- **PDF-Generierung:** `headless_chrome` (Crate) benötigt Chromium auf dem Server.
  Bei Deployment via Git: `apt install chromium-browser` im Setup-Script sicherstellen.

## 6. Quality & Validation (Globale Checkliste)

Dieser Abschnitt gilt als **Gesetz** für den Haupt-Agenten und alle Sub-Agenten:

### Zero-Ping-Pong & Architektur-Disziplin
1. **DRY (Don't Repeat Yourself)**: Bevor du neuen Code schreibst, MUSS eine Suche im bestehenden Verzeichnis erfolgen.
2. **SSOT (Single Source of Truth)**: Nutze die dafür vorgesehenen zentralen Dateien exklusiv.
3. **Zero-Ping-Pong**: Führe vor dem Abschluss JEDER Aufgabe `cargo check` oder `cargo build` aus.
4. **Design-Disziplin**: Jede neue Seite, jedes Feature und jede UI-Anpassung MUSS sich strikt am Design Guide in `DESIGN.md` orientieren (Farben, Abstände, Typografie).

### Git-Disziplin (gilt für alle Agenten)
- Nach jeder abgeschlossenen Aufgabe MUSS der Orchestrator einen Commit vorschlagen.
- Commit nur wenn `cargo check` oder `cargo build` erfolgreich war.
- Commit-Message beschreibt was geändert wurde, nicht was der Prompt war.
- Der Nutzer bestätigt den Commit explizit – kein Agent pusht eigenständig.
- Bei größeren Features: `git checkout -b feature/<name>` vor dem Start.
  Merge zurück auf `main` erst nach erfolgreichem `cargo build`.

## 7. Sub-Agenten Team (Strikte Delegation)

Der Orchestrator (Haupt-Agent) koordiniert alle Aufgaben und **muss** zwingend an die zuständigen Sub-Agenten delegieren. Eigenständige Implementierungen oder Code-Analysen durch den Orchestrator sind untersagt.

### Rollen & Zuweisung

| Agent | Zuständigkeit (MUST-Delegation) |
| :--- | :--- |
| **@rust-backend-expert** | Implementierung, Code-Änderungen, neue Features, Bug-Fixes. |
| **@code-reviewer** | Analysen ("Erkläre mir..."), Reviews, Struktur-Prüfung, Sicherheit. |
| **@tester** | Tests (`cargo test`), Fehlersuche, Reproduktion von Bug-Listen. |
| **@workspace-janitor** | Hygiene, Aufräumen, Kontext-Optimierung (`CONTEXT.md`). |

### Zusätzliche Referenzdateien
- **`SCHEMA.md`**: Aktueller Stand der Datenbank (Source of Truth für SQL).
- **`BUGS.md`**: Liste aller bekannten Fehler und Edge-Cases.
- **`CONTEXT.md`**: Sitzungsbezogene Notizen (wird vom Janitor bereinigt).

### Halluzinations-Prävention (@rust-backend-expert)
- **Cargo.toml zuerst**: Jede Antwort mit Code beginnt mit dem vollständigen
  `[dependencies]`-Block. Kein Code ohne passende Dependencies.
- **Keine Crate-Erfindungen**: Externe Crates NUR nutzen, wenn Cargo.toml-Eintrag
  + konkrete Version angegeben wird. Bei unbekannter API: `// TODO: API prüfen`
  statt Halluzination.
- **PDF-Workflow ist fix**: Ausschließlich Handlebars → HTML-String →
  headless_chrome → PDF-Bytes. Kein anderer Weg ist akzeptabel.
- **Chromium-Abhängigkeit**: `headless_chrome` benötigt eine installierte
  Chrome/Chromium-Binary auf dem Server (relevant für Hetzner-Deployment).

### Datenmodell-Pflicht
Wenn du `src/models.rs` änderst, weise den Nutzer am Ende explizit darauf hin:
"⚠️ Das Datenmodell wurde geändert – bitte erlaube mir, Abschnitt 2 der
GEMINI.md zu synchronisieren."
Tue dies NIEMALS eigenständig – nur mit expliziter Erlaubnis.

### Delegations-Pflicht
Der Orchestrator arbeitet **niemals alleine**, wenn ein spezialisierter Sub-Agent
besser geeignet ist. Die Zuweisung erfolgt explizit zu Beginn jeder Aufgabe.

### Beispiel-Delegation
- Neue Feature-Anfrage → @rust-backend-expert implementiert, @code-reviewer prüft
- Bug-Report → @tester reproduziert zuerst, dann @rust-backend-expert fixt
- Aufräumen / Kontext zu groß → @workspace-janitor

### Before you finish
Bevor du deine Antwort gibst:
1. **Intention des Nutzers**: Habe ich die eigentliche Absicht erfüllt?
2. **Validierung**: Sind 'lint' und 'build' erfolgreich durchgelaufen?
3. **Annahmen**: Habe ich Annahmen getroffen?
4. **Kontext-Hygiene**: Habe ich unnötige Artefakte bereinigt?
