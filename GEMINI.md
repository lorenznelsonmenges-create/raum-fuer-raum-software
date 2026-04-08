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

### Einsatz
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `i64` | Primärschlüssel |
| `auftrag_id` | `i64` | Fremdschlüssel auf `auftraege` |
| `datum` | `String` | Datum des Einsatzes |
| `kilometer` | `f64` | Gefahrene Kilometer |
| `stunden` | `f64` | Gearbeitete Stunden |
| `notiz` | `String` | Notiz zum Einsatz |
| `typ` | `String` | ARBEIT oder FAHRT |
| `signatur_pfad` | `Option<String>` | Pfad zum Signaturbild |

### Datei
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `i64` | Primärschlüssel |
| `auftrag_id` | `i64` | Fremdschlüssel auf `auftraege` |
| `dateiname` | `String` | Originaler Name der Datei |
| `dateipfad` | `String` | Relativer Pfad im `uploads/` Ordner |
| `dateityp` | `String` | MIME-Type oder Endung |
| `hochgeladen_am` | `String` | Zeitstempel des Uploads |
| `kategorie` | `String` | DATENSCHUTZ, VERTRAG, SONSTIGES, SIGNATUR, RECHNUNG |

### RechnungsNotiz
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `i64` | Primärschlüssel |
| `auftrag_id` | `i64` | Fremdschlüssel auf `auftraege` |
| `text` | `String` | Inhalt der Notiz |
| `auf_rechnung` | `bool` | Erscheint auf der PDF-Rechnung |

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
- **PDF-Generierung:** Headless Chrome (`chromiumoxide`) benötigt Chromium 
  auf dem Server. Bei Deployment via Git: `apt install chromium-browser` 
  im Setup-Script sicherstellen.

## 6. Quality & Validation (Globale Checkliste)

Dieser Abschnitt gilt als **Gesetz** für den Haupt-Agenten und alle Sub-Agenten:
### Zero-Ping-Pong & Architektur-Disziplin
1. **DRY (Don't Repeat Yourself)**: Bevor du neuen Code schreibst, MUSS eine Suche im bestehenden Verzeichnis erfolgen.
2. **SSOT (Single Source of Truth)**: Nutze die dafür vorgesehenen zentralen Dateien exklusiv.
3. **Zero-Ping-Pong**: Führe vor dem Abschluss JEDER Aufgabe `cargo check` oder `cargo build` aus.
### Git-Disziplin (gilt für alle Agenten)
- Nach jeder abgeschlossenen Aufgabe MUSS der Orchestrator einen Commit vorschlagen.
- Commit nur wenn `cargo check` oder `cargo build` erfolgreich war.
- Commit-Message beschreibt was geändert wurde, nicht was der Prompt war.
- Der Nutzer bestätigt den Commit explizit – kein Agent pusht eigenständig.
- Bei größeren Features: `git checkout -b feature/<name>` vor dem Start.
  Merge zurück auf `main` erst nach erfolgreichem `cargo build`.

## 7. Sub-Agenten Team

Der Orchestrator (Haupt-Agent) koordiniert alle Aufgaben und delegiert aktiv
an die zuständigen Sub-Agenten. **Für jede Aufgabe MUSS der Orchestrator
explizit entscheiden, welcher Sub-Agent die Arbeit übernimmt.**

### Rollen

| Agent | Zuständigkeit |
| :--- | :--- |
| **Orchestrator** | Projektmanager & Schnittstelle zum Nutzer. Koordiniert, delegiert, validiert. |
| **@rust-backend-expert** | Implementierung in Rust, Axum & SQLx. Hält streng DRY & Architektur-Disziplin ein. |
| **@code-reviewer** | Prüft Code auf Best Practices, Sicherheit und Korrektheit – bevor Probleme entstehen. |
| **@tester** | Schreibt automatisierte Tests (`cargo test`), deckt Randfälle ab, reproduziert Bugs. |
| **@workspace-janitor** | Bereinigt den Workspace, reduziert Kontext-Overhead durch Artefakt-Entfernung. |

### Halluzinations-Prävention (@rust-backend-expert)
- **Cargo.toml zuerst**: Jede Antwort mit Code beginnt mit dem vollständigen
  `[dependencies]`-Block. Kein Code ohne passende Dependencies.
- **Keine Crate-Erfindungen**: Externe Crates NUR nutzen, wenn Cargo.toml-Eintrag
  + konkrete Version angegeben wird. Bei unbekannter API: `// TODO: API prüfen`
  statt Halluzination.
- **PDF-Workflow ist fix**: Ausschließlich Tera → HTML-String →
  chromiumoxide → PDF-Bytes. Kein anderer Weg ist akzeptabel.
- **Chromium-Abhängigkeit**: `chromiumoxide` benötigt eine installierte
  Chrome/Chromium-Binary auf dem Server (relevant für Hetzner-Deployment).

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
