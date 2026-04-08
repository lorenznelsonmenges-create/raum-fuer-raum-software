# Architektur: Achtsam Entrümpeln

## Übersicht
- **Frontend**: Admin-UI (HTML/CSS/JS) angebunden an REST-API.
- **Backend**: Rust (Axum Framework).
- **Datenbank**: SQLite via SQLx (Migrationsbasiert).
- **Deployment**: Hetzner Server (Reverse Proxy).

## Design-Prinzipien
1. **Zentralisiertes Error-Handling**: Alle API-Fehler werden in `src/error.rs` konsolidiert.
2. **Typ-Sicherheit**: Strenge Nutzung des Rust-Typ-Systems für alle Datenmodelle in `src/models.rs`.
3. **Automatisierung**: Migrationen beim Start (`migrations/`), automatische PDF-Generierung (geplant).
