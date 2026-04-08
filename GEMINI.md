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

### RechnungsNotiz
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `i64` | Primärschlüssel |
| `auftrag_id` | `i64` | Fremdschlüssel auf `auftraege` |
| `text` | `String` | Inhalt der Notiz |
| `auf_rechnung` | `bool` | Haken: Erscheint diese Notiz auf der finalen PDF-Rechnung? |

## 3. Datenbankschema – Migrations-Übersicht

Die Migrationen werden automatisch beim Start ausgeführt (Ordner `migrations/`).
⚠️ Neue Migrationen NIE rückgängig machen – immer neue Migrations-Datei erstellen.

| Datei | Inhalt |
| :--- | :--- |
| `20240405120000_init.sql` | Tabellen `kunden`, `auftraege`, `rechnungs_notizen` |
| `20240405130000_add_einsaetze.sql` | Tabelle `einsaetze` (Stunden, Kilometer) |
| `20240405140000_add_dateien.sql` | Tabelle `dateien` (Datei-Uploads) |
| `20240406100000_rename_stadt_to_ort.sql` | `stadt` → `ort` in `kunden` |
| `20240407100000_extend_models.sql` | `kategorie` zu `dateien`, `typ` zu `einsaetze` |
| `20240407110000_add_rechnungen.sql` | Tabelle `rechnungen` |
| `20240408120000_add_signature_to_einsaetze.sql` | `signatur_pfad` zu `einsaetze` |
| `20240408150000_add_prices_to_auftraege.sql` | `stundensatz`, `kilometer_satz` zu `auftraege` |
| `20240408160000_fix_km_satz_naming.sql` | `km_satz` → `kilometer_satz` (Namenskorrektur) |

### Wichtige Spalten-Hinweise
- `kunden.ort` (nicht `stadt` – wurde umbenannt)
- `auftraege.kilometer_satz` (nicht `km_satz` – wurde umbenannt)
- `auftraege.preis_manuell` existiert noch in der DB aber nicht mehr im Rust-Code

## 4. Status der API-Endpunkte

- [x] **Kunden:** CRUD-Operationen (Erstellen, Lesen, Liste, Update, Löschen).
- [x] **Aufträge:** Erstellung, Status-Management und Update.
- [x] **Einsätze:** Dokumentation von Stunden/Kilometern + Digitale Signatur.
- [x] **Uploads:** Multipart-Form Upload für Dokumente/Bilder + Drag & Drop Support.
- [x] **Email:** Platzhalter-Endpunkt für den Stundennachweis-Versand.

## 5. Nächste Schritte

1. [ ] **Dokumenten-Feedback:** Visuelle Hervorhebung nach erfolgreichem Upload.
2. [ ] **PDF-Rechnungserstellung:** Finalisierung des Designs und Einbindung der Vorlagen.
3. [ ] **Frontend:** Weiterer Ausbau der Admin-UI.
4. [ ] **Login:** Absicherung der API.
5. [ ] **Testing:** Einführung automatisierter Tests (cargo test).

## 6. Betriebliche Hinweise

- **Hosting:** Geplant auf Hetzner-Server via Git-Deployment.
- **Email:** Finalisierung der Adressen (Platzhalter: `hallo@achtsam-entruempeln.de`).
- **Entwicklung:** `target/`, `achtsam.db*`, `uploads/` und `Cargo.lock` werden ignoriert.
- **PDF-Generierung:** `headless_chrome` (Crate) benötigt Chromium auf dem Server.
  Bei Deployment via Git: `apt install chromium-browser` im Setup-Script sicherstellen.

## 7. Quality & Validation (Globale Checkliste)

Dieser Abschnitt gilt als **Gesetz** für den Haupt-Agenten und alle Sub-Agenten:

### Zero-Ping-Pong & Architektur-Disziplin
1. **DRY (Don't Repeat Yourself)**: Bevor du neuen Code schreibst, MUSS eine Suche im bestehenden Verzeichnis erfolgen.
2. **SSOT (Single Source of Truth)**: Nutze die dafür vorgesehenen zentralen Dateien exklusiv.
3. **Zero-Ping-Pong**: Führe vor dem Abschluss JEDER Aufgabe `cargo check` oder `cargo build` aus.
4. **Design-Disziplin**: Jede neue Seite, jedes Feature und jede UI-Anpassung MUSS sich strikt am Design Guide in `DESIGN.md` orientieren (Farben, Abstände, Typografie).
5. **`BUGS.md` zuerst lesen**: Bevor du Code schreibst oder änderst, lies `BUGS.md`. Stelle sicher dass du keinen behobenen Bug wieder einführst und trage neue Bugs sofort ein.

### Git-Disziplin (gilt für alle Agenten)
- Nach jeder abgeschlossenen Aufgabe MUSS der Orchestrator einen Commit vorschlagen.
- Commit nur wenn `cargo check` oder `cargo build` erfolgreich war.
- Commit-Message beschreibt was geändert wurde, nicht was der Prompt war.
- Der Nutzer bestätigt den Commit explizit – kein Agent pusht eigenständig.
- Bei größeren Features: `git checkout -b feature/<n>` vor dem Start.
  Merge zurück auf `main` erst nach erfolgreichem `cargo build`.

## 8. Sub-Agenten Team (Strikte Delegation)

## 7. Sub-Agenten Team (Automatisierte Delegation)

**AUTOMATIONS-REGEL:** Jede Benutzeranfrage, die nicht explizit an einen Agenten gerichtet ist (z.B. durch @name), wird ZWINGEND zuerst intern an den **@orchestrator** delegiert. Der Haupt-Agent darf keine eigenständigen Code-Änderungen vornehmen.

### Rollen & Zuweisung

| Agent | Zuständigkeit |
| :--- | :--- |
| **@orchestrator** | **Zentrale Einstiegsinstanz.** Analysiert Prompts, liest BUGS.md/GEMINI.md und delegiert an Spezialisten. Schreibt niemals Code. |
| **@rust-backend-expert** | Implementierung, Code-Änderungen, neue Features, Bug-Fixes im Rust-Code. |
| **@code-reviewer** | Analysen, Reviews, Struktur-Prüfung, Sicherheits-Audits. |
| **@tester** | Reproduktion von Fehlern, Schreiben und Ausführen von Tests (`cargo test`). |
| **@workspace-janitor** | Kontext-Hygiene, Aufräumen, Aktualisierung der `CONTEXT.md` oder `GEMINI.md`. |

### Zusätzliche Referenzdateien
- **`BUGS.md`**: Liste aller bekannten Fehler – vor jeder Arbeit lesen.
- **`CONTEXT.md`**: Sitzungsbezogene Notizen (max. 20 Zeilen, wird vom Janitor bereinigt).

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
5. **BUGS.md**: Habe ich neue Bugs eingetragen oder behobene Bugs verschoben?
