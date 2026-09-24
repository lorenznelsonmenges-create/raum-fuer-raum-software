# Architektur: Wendepunkt — Raum für Neues (Auftragsverwaltung)

## Übersicht & Deployment
- **Hetzner Cloud Server**: `46.62.148.232` (Ubuntu 24.04 LTS, Nginx Reverse Proxy mit SSL/Let's Encrypt).
- **Öffentliche Haupt-Website**: `https://wendepunkt-ruf.de` (statische Seiten, getrennt vom Backend betrieben).
- **Interne Web-App (Auftragsverwaltung)**: `https://app.wendepunkt-ruf.de` (passwortgeschützt).
- **Backend**: Rust (Axum Framework, Port 3000), Tokio Async Runtime.
- **Frontend App**: Single-Page Admin-UI (HTML5, Tailwind CSS, Alpine.js) angebunden an REST-API (`/api`).
- **Datenbank**: SQLite (`achtsam.db`) via SQLx mit automatischen Migrationen beim Start (`migrations/`).
- **Dokumentenerzeugung**: Dynamische PDF-Generierung via Handlebars & Headless Chrome.

## Authentifizierung & Sicherheit
1. **Passwort-Hashing**: Alle Passwörter werden sicher per `bcrypt` gehasht in der SQLite-Tabelle `users` gespeichert. Es existieren keine Plaintext-Dateien für Logins.
2. **Session-Handling**: `tower-sessions` mit SQLite-Session-Store (`tower-sessions-sqlx-store`), `with_secure(true)` und 24h Inaktivitäts-Timeout.
3. **Zentrales Error-Handling**: Konsolidierte Fehlerbehandlung in `src/error.rs`.
4. **Typ-Sicherheit**: Strikte Modellierung in Rust (`src/models.rs`).
