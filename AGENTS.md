# Software-Projekt: Wendepunkt – Raum für Neues (vormals Achtsam Entrümpeln) – Backend

**GESCHÜTZTE DATEI:** Diese Datei darf ausschließlich geändert werden,
> wenn der Nutzer dies in der aktuellen Konversation explizit erlaubt hat.
> Kein Agent darf diese Datei eigenständig bearbeiten.

Dieses Dokument dient als zentrale Wissensbasis ("Source of Truth") für die Architektur, das Datenmodell und den Entwicklungsfortschritt der Software.

## 1. Architektur & Design-Entscheidungen

- **Backend:** Rust mit dem **Axum** Web-Framework. Port über Umgebungsvariable `PORT` (Default `3000`, Produktion `3001`).
- **Datenbank:** **SQLite** (`achtsam.db`), asynchron angebunden via `sqlx`. Pfad über `DATABASE_URL` (aus `.env` via `dotenvy`, Default `sqlite:achtsam.db`).
- **Migrationen:** Automatische Migrationen beim Start der Anwendung (Ordner `migrations/`).
- **Error-Handling:** Zentralisiertes System in `src/error.rs` für konsistente HTTP-Statuscodes.
- **Datei-Management:** Uploads landen im Ordner `uploads/` auf der Festplatte; Pfade werden in der DB gespeichert.
- **Frontend-Anbindung:** Aktuell als reine REST-API konzipiert (bereit für Nginx/HTTPS-Reverse-Proxy).
- **Templating:** Handlebars 6.x (nicht Tera – trotz anderslautender älterer Notizen).
- **PDF-Generierung:** `headless_chrome` 1.x via HTML → PDF.
- **Domain Primitives (Secure by Design):** `src/domain.rs` – selbstvalidierende, unveränderliche Typen (`Euro` in Cent als `i64`, `Stunden`, `Kilometer`, `EinsatzTyp`, `RechnungsNummer` sowie für die Buchhaltung `BuchungsTyp`, `BuchungsBetrag`, `Kategorie`, `Beschreibung`, `BelegReferenz`, `BuchungsDatum`, Belege: `BelegFormat`, `BelegDatei`, `BelegDateiname`, `BelegPfad`, sowie `Geschaeftsjahr`). **Niemals `f64` für Geldbeträge.**
- **Geschäftsjahr / EÜR:** Der Betrieb macht eine Einnahmen-Überschuss-Rechnung (keine Bilanz → im UI „Einnahmen & Ausgaben“, nie „Bilanz“). Geschäftsjahr = Kalenderjahr (§ 4a EStG). Diese Regel steht **ausschließlich** in `Geschaeftsjahr` (`aus_datum`, `zeitraum`); das Jahr wird nie gespeichert, sondern aus `datum` (= Zahlungsdatum, Zuflussprinzip) abgeleitet. Auch das Frontend rechnet keine Jahresgrenzen selbst, sondern nutzt `von`/`bis` aus `GET /api/buchungen/jahre`.
- **Code-Struktur:** Monolithisch – Handler + Router in `main.rs`, Modelle in `models.rs`, DB-Funktionen in `database.rs`, Domain Primitives in `domain.rs`. Neue Module folgen diesem Muster.
- **Autorisierung:** Session-basiert (`auth_middleware` + `AuthUser`-Extractor). Keine Rollen – jeder eingeloggte User hat vollen Zugriff auf alle Module.

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
| `basis_pauschale` | `Option<Euro>` | Optionale Fixkosten-Pauschale |
| `stundensatz` | `Euro` | Stundensatz Dienstleistung (fällt bei 0 auf Einstellungen zurück) |
| `stundensatz_nachbereitung` | `Euro` | Stundensatz Nachbereitung (fällt bei 0 auf Einstellungen zurück) |
| `kilometer_satz` | `Euro` | Kilometersatz (fällt bei 0 auf Einstellungen zurück) |
| `notizen` | `String` | Interne Auftragsnotizen |
| `created_by` | `Option<String>` | Ersteller des Auftrags (Benutzername, serverseitig aus Session) |
| `einsaetze`, `dateien`, `rechnungen`, `rechnungs_notizen` | `Vec<…>` | Beim Laden mitgeliefert (nicht in der Tabelle `auftraege`) |

### Einsatz (Arbeitszeit & Fahrtkosten)
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `i64` | Primärschlüssel |
| `auftrag_id` | `i64` | Fremdschlüssel auf `auftraege` |
| `datum` | `String` | Datum des Einsatzes |
| `kilometer` | `Kilometer` | Gefahrene Kilometer (≥ 0) |
| `stunden` | `Stunden` | Gearbeitete Stunden (≥ 0) |
| `notiz` | `String` | Interne Notiz zum Einsatz |
| `typ` | `EinsatzTyp` | `DIENSTLEISTUNG`, `NACHBEREITUNG` oder `KILOMETER` (Altwerte `ARBEIT_VOR_ORT`/`ARBEIT`, `ARBEIT_VORBEREITUNG`, `FAHRT` werden beim Einlesen gemappt) |
| `unterkategorie` | `Option<String>` | Dienstleistung: Praktische Unterstützung, Beratung, Mediation · Nachbereitung: Auswertung, Verteilung |
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

### Rechnung
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `i64` | Primärschlüssel |
| `auftrag_id` | `i64` | Fremdschlüssel auf `auftraege` |
| `rechnungs_nummer` | `RechnungsNummer` | Fortlaufend, Format `R` + 6 Ziffern |
| `datum` | `String` | Ausstellungsdatum |
| `gesamt_netto` | `Euro` | Gesamtsumme (Netto) |
| `gesamt_brutto` | `Euro` | Gesamtsumme (Brutto, 19 % USt.) |
| `status` | `String` | z.B. ENTWURF, GESENDET, BEZAHLT |
| `pdf_pfad` | `String` | Relativer Pfad zur PDF-Datei |

### RechnungsNotiz
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `i64` | Primärschlüssel |
| `auftrag_id` | `i64` | Fremdschlüssel auf `auftraege` |
| `text` | `String` | Inhalt der Notiz |
| `auf_rechnung` | `bool` | Haken: Erscheint diese Notiz auf der finalen PDF-Rechnung? |

### Settings (Einstellungen) 
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `i64` | Primärschlüssel (immer 1) |
| `stundensatz` | `Euro` | Standard-Stundensatz Dienstleistung (45,00 €) |
| `stundensatz_nachbereitung` | `Euro` | Standard-Stundensatz Nachbereitung (45,00 €) |
| `kilometer_satz` | `Euro` | Standard-Kilometersatz (0,50 €) |

### User
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `i64` | Primärschlüssel |
| `username` | `String` | Benutzername |
| `password_hash` | `String` | Gehashtes Passwort |
| `role` | `String` | Benutzerrolle (aktuell nur `ADMIN`, wird nicht ausgewertet) |

### LoginRequest (DTO)
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `username` | `String` | Benutzername |
| `password` | `String` | Passwort (Klartext für Login-Prozess) |

### DashboardStats (DTO)
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `anfrage_laeuft` | `i64` | Anzahl Aufträge mit Status 'AnfrageLaeuft' |
| `in_bearbeitung` | `i64` | Anzahl Aufträge mit Status 'InBearbeitung' |
| `abgeschlossen` | `i64` | Anzahl Aufträge mit Status 'Abgeschlossen' |
| `storniert` | `i64` | Anzahl Aufträge mit Status 'Storniert' |
| `aktuelle_auftraege` | `i64` | Summe aller nicht-stornierten & nicht-abgeschlossenen Aufträge |

### Buchung (Entity – Buchhaltung)
Felder sind nur crate-intern sichtbar (`pub(crate)`), Zugriff von außen über Getter. Gleichheit nur über `id`.
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `id` | `i64` | Primärschlüssel |
| `buchungs_typ` | `BuchungsTyp` | Enum `Einnahme` / `Ausgabe` (DB/JSON: `einnahme` / `ausgabe`) |
| `betrag` | `BuchungsBetrag` | Wrapper um `Euro` (Cent, i64): > 0 und ≤ 10 Mio. € (DB: `betrag_cent`, JSON: Euro als Zahl) |
| `kategorie` | `Kategorie` | 1–100 Zeichen, Whitelist (Buchstaben, Ziffern, Leerzeichen, `- _ . , & / ( ) +`) |
| `beschreibung` | `Beschreibung` | max. 500 Zeichen, keine Steuerzeichen, kein `<` `>` |
| `datum` | `BuchungsDatum` | ISO `YYYY-MM-DD`, nicht in der Zukunft, nicht vor 2000 |
| `auftrag_id` | `Option<i64>` | FK auf `auftraege` (`ON DELETE SET NULL`) |
| `beleg_referenz` | `Option<BelegReferenz>` | Belegnummer, max. 100 Zeichen, Whitelist (`- _ / . #`) |
| `beleg` | `Option<BelegMeta>` | Beleg-Datei (`pfad: BelegPfad`, `dateiname: BelegDateiname`, `format: BelegFormat`); JSON `{dateiname, typ, url}`. `None` nur bei Altbuchungen vor der Belegpflicht |
| `created_at` | `String` | Zeitstempel (DB-Default `datetime('now')`, UTC) |
| `created_by` | `i64` | FK auf `users`, **serverseitig** aus der Session gesetzt |

### BuchungEingabe (DTO)
Roh-Eingabe vom Client mit `deny_unknown_fields` (kein Einschleusen von `id`/`created_by`). `validieren()` → `BuchungsDaten` (nur Domain Primitives).
Wird als Multipart-Feld `daten` (JSON) gesendet, zusammen mit dem Datei-Feld `beleg` (→ `BelegDatei`: PDF/JPG/PNG per Magic Bytes, max. 10 MB).
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `buchungs_typ` | `String` | `einnahme` oder `ausgabe` |
| `betrag` | `f64` | Euro, nur Transportformat → sofort `Euro::from_euro_f64()` |
| `kategorie` | `String` | Pflicht |
| `beschreibung` | `String` | optional (Default leer) |
| `datum` | `String` | `YYYY-MM-DD` |
| `auftrag_id` | `Option<i64>` | optional, muss existieren (semantische Prüfung im Handler) |
| `beleg_referenz` | `Option<String>` | optional, leer = `None` |

### BuchungsUebersicht (DTO)
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `einnahmen_gesamt` | `Euro` | Summe der Einnahmen (JSON: Euro) |
| `ausgaben_gesamt` | `Euro` | Summe der Ausgaben (JSON: Euro) |
| `saldo` | `i64` | Einnahmen − Ausgaben in **Cent** (kann negativ sein) |
| `anzahl_ohne_beleg` | `i64` | Buchungen im Zeitraum ohne Beleg-Datei (`beleg_pfad IS NULL`) |

Filter-DTOs: `BuchungsFilterParameter` (`von`, `bis`, `jahr`, `typ`, `kategorie`) → `BuchungsFilter`; `ZeitraumParameter` (`von`, `bis`, `jahr`) → `Zeitraum` (prüft `von` ≤ `bis`; `jahr` wird über `Geschaeftsjahr::zeitraum()` übersetzt, `jahr` + `von`/`bis` → 400).

### GeschaeftsjahrInfo (DTO)
| Feld | Typ | Beschreibung |
| :--- | :--- | :--- |
| `jahr` | `Geschaeftsjahr` | JSON: Zahl, z.B. `2026` |
| `von` | `String` | Erster Tag (`YYYY-01-01`) aus `Geschaeftsjahr::zeitraum()` |
| `bis` | `String` | Letzter Tag (`YYYY-12-31`) aus `Geschaeftsjahr::zeitraum()` |

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
| `20240409000000_add_indices.sql` | Performance-Indizes für Fremdschlüssel (Kunde/Auftrag) |
| `20240409000001_add_status_index.sql` | Index auf `auftraege.status` für Performance |
| `20240409000002_add_settings.sql` | Tabelle `einstellungen` |
| `20240410000000_add_users.sql` | Tabelle `users` |
| `20240411000000_fix_admin_hash.sql` | Valider Bcrypt-Hash für `admin` |
| `20260914000000_add_created_by_to_auftraege.sql` | Spalte `created_by` in `auftraege` |
| `20260922000000_update_einsatz_typen.sql` | Migration auf 3 Einsatz-Typen (`ARBEIT_VOR_ORT`, `ARBEIT_VORBEREITUNG`, `KILOMETER`) |
| `20260924000001_ensure_users.sql` | Tabelle `users` & sichere Bcrypt-Hashes für `admin` und `stefanie` |
| `20260928000000_update_categories_and_unterkategorie.sql` | Spalte `unterkategorie` in `einsaetze`; Typen → `DIENSTLEISTUNG` / `NACHBEREITUNG` |
| `20260928120000_add_stundensatz_nachbereitung.sql` | Spalte `stundensatz_nachbereitung` in `einstellungen` und `auftraege` |
| `20261001000000_add_buchungen.sql` | Tabelle `buchungen` (Buchhaltung) mit CHECK-Constraints + Indizes auf `datum`, `buchungs_typ`, `created_by` |
| `20261002000000_add_beleg_zu_buchungen.sql` | Spalten `beleg_pfad`, `beleg_dateiname`, `beleg_typ` in `buchungen` (Belegpflicht) |

### Wichtige Spalten-Hinweise
- `kunden.ort` (nicht `stadt` – wurde umbenannt)
- `auftraege.kilometer_satz` (nicht `km_satz` – wurde umbenannt)
- `auftraege.created_by` (Ersteller des Auftrags)
- `auftraege.preis_manuell` existiert noch in der DB aber nicht mehr im Rust-Code
- Geldbeträge: Alt-Tabellen speichern `REAL` (Euro), `buchungen.betrag_cent` speichert `INTEGER` (Cent). Im Rust-Code ist beides `Euro(i64)`.
- Vollständiges Tabellenschema: siehe `SCHEMA.md`.

## 4. Status der API-Endpunkte

- [x] **Kunden:** CRUD-Operationen (Erstellen, Lesen, Liste, Update, Löschen).
- [x] **Aufträge:** Erstellung, Status-Management und Update.
- [x] **Einsätze:** Dokumentation von Stunden/Kilometern + Digitale Signatur (3 Typen).
- [x] **Uploads:** Multipart-Form Upload für Dokumente/Bilder + Drag & Drop Support.
- [x] **Email:** Platzhalter-Endpunkt für den Stundennachweis-Versand.
- [x] **Dashboard:** Oben Kacheln Kunden / Aktuelle Aufträge; darunter zweispaltig links Status-Verteilung (`/api/stats`), rechts „Einnahmen & Ausgaben“ je Geschäftsjahr mit Vorjahresvergleich und Hinweis auf Buchungen ohne Beleg.
- [x] **Buchhaltung-Übersicht:** Zeitraum-Modus *Alles* / *Geschäftsjahr* / *Monat* (bzw. *Freier Zeitraum* bei manuellem Von/Bis); Kacheln und Liste folgen dem Zeitraum.
- [x] **Buchhaltung:** `GET|POST /api/buchungen` (Filter: `von`, `bis` **oder** `jahr`, `typ`, `kategorie`), `GET /api/buchungen/uebersicht` (`von`, `bis` **oder** `jahr`; inkl. `anzahl_ohne_beleg`), `GET /api/buchungen/jahre` (Jahre mit Buchungen + laufendes Jahr, absteigend, je mit `von`/`bis`), `GET|POST /api/buchungen/:id`, `POST /api/buchungen/:id/delete`. Anlegen/Ändern als Multipart (`daten` + `beleg`), **Belegpflicht** (PDF/JPG/PNG, max. 10 MB, gespeichert unter `uploads/belege/<uuid>`, Abruf nur eingeloggt über `/uploads/…`). Audit-Log (`[AUDIT]`) für jede Mutation.

## 5. Nächste Schritte

1. [x] **Dashboard-Chart:** Statistische Auswertung der Auftragszahlen.
2. [x] **PDF-Rechnungserstellung:** Finalisierung des Designs und Einbindung der Vorlagen.
3. [x] **Login:** Rein datenbankbasierte Absicherung via Bcrypt (`logins.json` entfernt).
4. [x] **Testing:** Automatisierte Integrationstests (`cargo test`).
5. [ ] **Dokumenten-Feedback:** Visuelle Hervorhebung nach erfolgreichem Upload.
6. [ ] **Frontend:** Weiterer Ausbau der Admin-UI.
7. [ ] **Kunden-Validierung:** Backend-Prüfung für E-Mail-Formate (400 statt 500 Fehler).
8. [x] **Buchhaltung:** Einnahmen/Ausgaben mit Übersicht (Einnahmen, Ausgaben, Saldo).
9. [ ] **Buchhaltung GoBD:** Storno-Buchungen statt Löschen, Jahresabschluss, Aufbewahrung, USt-Ausweis – Details in `BUGS.md` (Abschnitt „Buchhaltung – fachlich offen“).
10. [ ] **Sicherheit:** Offene Punkte siehe `BUGS.md` (Abschnitt „Sicherheit“).

## 6. Betriebliche Hinweise

- **Projekt-Name:** Wendepunkt — Raum für Neues (vormals Achtsam Entrümpeln).
- **Hosting:** Hetzner Cloud VPS (`ubuntu-4gb-hel1-1`, IP: `46.62.148.232`).
- **Domains:**
  - Haupt-Website: `https://wendepunkt-ruf.de`
  - Interne Auftragsverwaltung: `https://app.wendepunkt-ruf.de`
- **Server-Setup:** Ubuntu 24.04 (Noble), Nginx als Reverse-Proxy (HTTPS) vor dem Backend.
- **SSL:** Let's Encrypt via Certbot.
- **Prozess-Management:** Systemd-Service **`achtsam.service`** („Achtsam Entruempeln Backend“, enabled) – *nicht* `wendepunkt.service`.
- **Port (Produktion):** Backend lauscht auf **3001** (`0.0.0.0:3001`). Port 3000 ist auf dem Server durch ein anderes Projekt (`carsharing-backend`) belegt.
- **Dateipfade:** Projektordner **`/var/www/achtsam-backend`** (Git-Checkout, Arbeitsverzeichnis des Dienstes). Dort liegen auch `achtsam.db` und `uploads/`.
- **Deployment:**
  ```bash
  cd /var/www/achtsam-backend
  cp achtsam.db achtsam.db.bak-$(date +%F)   # Backup vor Migrationen
  git pull
  cargo build --release
  systemctl restart achtsam
  systemctl status achtsam | head -5
  ```
  Migrationen laufen beim Start automatisch. Neue Migrationen vor dem Deploy committen, damit sie in der richtigen Reihenfolge laufen.
- **Logs / Audit:** `journalctl -u achtsam` – Buchhaltungs-Mutationen sind mit `[AUDIT]` markiert (Zeitstempel, User-ID).
- **Email:** `info@wendepunkt-ruf.de` (Postfach in konsoleH, DNS bei Hetzner konfiguriert).
- **PDF-Generierung:** `headless_chrome` benötigt `chromium-browser` auf dem Server.
- **Sicherheit/Sessions:** Da die App hinter einem Nginx-Reverse-Proxy mit HTTPS läuft, MUSS `tower_sessions` in `src/main.rs` zwingend mit `.with_secure(true)` konfiguriert sein, andernfalls verweigern Browser (wie Firefox/Chrome) das Speichern des Login-Cookies.

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

**AUTOMATIONS-REGEL:** Jede Benutzeranfrage, die nicht explizit an einen Agenten gerichtet ist (z.B. durch @name), wird ZWINGEND zuerst intern an den **@orchestrator** delegiert. Der Haupt-Agent darf keine eigenständigen Code-Änderungen vornehmen.

### Eiserne Regeln für den @orchestrator (Verhinderung von Role-Drift)

1. **Eiserne Regel:** Jeder Aufruf von schreibenden Tools (`replace`, `write_file`, etc.) durch den Haupt-Agenten OHNE Delegation an einen Sub-Agenten gilt als schwerer Protokollbruch.
2. **Stopp-Signal:** Bevor der Agent ein schreibendes Tool nutzt, MUSS er innerlich prüfen: *"Bin ich der @rust-backend-expert?"*. Wenn nein -> Delegation ist Pflicht.
3. **Delegations-Primat:** Der @orchestrator ist verpflichtet, jede Aufgabe IMMER erst im Kopf zu delegieren, bevor ein Implementierungs-Gedanke entsteht.

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
