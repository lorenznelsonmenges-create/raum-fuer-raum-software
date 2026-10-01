# Architektur: Wendepunkt — Raum für Neues (Auftragsverwaltung)

## Übersicht & Deployment
- **Hetzner Cloud Server**: `46.62.148.232` (`ubuntu-4gb-hel1-1`, Ubuntu 24.04 LTS, Nginx Reverse Proxy mit SSL/Let's Encrypt).
- **Öffentliche Haupt-Website**: `https://wendepunkt-ruf.de` (statische Seiten, getrennt vom Backend betrieben).
- **Interne Web-App (Auftragsverwaltung)**: `https://app.wendepunkt-ruf.de` (passwortgeschützt).
- **Backend**: Rust (Axum Framework), Tokio Async Runtime. Port über `PORT` (Default 3000, Produktion **3001**).
- **Betrieb**: Systemd-Service `achtsam.service`, Projektordner `/var/www/achtsam-backend` (Details & Deploy-Befehle: `AGENTS.md`, Abschnitt 6).
- **Frontend App**: Single-Page Admin-UI (HTML5, Tailwind CSS, Alpine.js) in `static/index.html`, angebunden an REST-API (`/api`).
- **Datenbank**: SQLite (`achtsam.db`, konfigurierbar über `DATABASE_URL`) via SQLx mit automatischen Migrationen beim Start (`migrations/`). Schema: `SCHEMA.md`.
- **Dokumentenerzeugung**: Dynamische PDF-Generierung via Handlebars & Headless Chrome.

## Module
| Modul | Inhalt |
| :--- | :--- |
| Kunden | Stammdaten, CRUD |
| Aufträge | Status, Stundensätze, Pauschale, Archiv |
| Einsätze | Dienstleistung / Nachbereitung (Stunden, Unterkategorie) und Kilometer, digitale Signatur |
| Dokumente | Uploads, Vorlagen (Vertrag, Datenschutz …), PDF-Rechnungen |
| Buchhaltung | Einnahmen & Ausgaben, Filter, Übersicht (Einnahmen, Ausgaben, Saldo) |
| Admin | Vorlagenverwaltung, Standard-Verrechnungssätze |

## Authentifizierung & Sicherheit
1. **Passwort-Hashing**: Alle Passwörter werden per `bcrypt` gehasht in der SQLite-Tabelle `users` gespeichert. Es existieren keine Plaintext-Dateien für Logins.
2. **Session-Handling**: `tower-sessions` mit SQLite-Session-Store (`tower-sessions-sqlx-store`), `with_secure(true)` und 24h Inaktivitäts-Timeout. Alle `/api`-Routen (außer Login/Logout) und `/uploads` liegen hinter `auth_middleware`; keine Rollenunterscheidung.
3. **Zentrales Error-Handling**: Konsolidierte Fehlerbehandlung in `src/error.rs`.
4. **Domain Primitives (Secure by Design)**: Selbstvalidierende, unveränderliche Typen in `src/domain.rs`. Geld wird ausschließlich als `Euro(i64)` in Cent modelliert – nie als `f64`.
5. **Input Validation (Buchhaltung)**: Feste Reihenfolge Origin (Session) → Size (16-KB-Body-Limit) → Lexical → Syntax (Domain Primitives) → Semantic (DB-Prüfungen). DTOs mit `deny_unknown_fields` verhindern Mass Assignment (`created_by` kommt immer aus der Session).
6. **Traceability**: Jede Buchungs-Mutation wird mit Zeitstempel und User-ID als `[AUDIT]`-Zeile geloggt (`journalctl -u achtsam`).
7. **Pfad-Schutz**: Template- und Upload-Handler verhindern Path Traversal.
