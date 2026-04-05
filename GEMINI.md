# Software-Projekt: Achtsam Entrümpeln (Backend)

Dieses Dokument beschreibt den aktuellen Stand und die Architektur der Software-Komponente.

## 1. Status & Fortschritt

- [x] **Projekt-Initialisierung:** Cargo-Projekt mit Rust/Axum erstellt.
- [x] **Datenmodell:** `Kunde` und `Auftrag` Strukturen in `src/models.rs` definiert (inkl. Serde-Unterstützung).
- [x] **Datenbank-Anbindung:** SQLite Integration mit `sqlx` in `src/database.rs`.
- [x] **Migrationen:** Initiales SQL-Schema für `kunden` und `auftraege` erstellt.
- [x] **API-Basis:** Erste Endpunkte für Kunden-CRUD (`/api/kunden`) in `src/main.rs` implementiert.

## 2. Architektur & Entscheidungen

- **Backend:** Axum Web-Framework (asynchron via Tokio).
- **Datenbank:** SQLite (`achtsam.db`), verwaltet über `sqlx` (asynchron, Compile-Zeit SQL-Checks).
- **Deployment-Strategie:** 
    - Software läuft auf einer **Subdomain** (z.B. `admin.achtsam-entruempeln.de`).
    - Keine direkte Kopplung mit der statischen Haupt-Website.
    - Zugriffsschutz via Passwort/Login ist für die nächste Phase geplant.
- **Server-Ready:** Konfiguriert für Betrieb hinter einem Reverse-Proxy (Nginx) auf Port 3000.

## 3. Nächste Schritte

1. Implementierung der vollständigen Auftrags-Logik (CRUD für Aufträge).
2. Aufbau eines Frontends (Web-UI) für die Administration.
3. Integration eines sicheren Login-Verfahrens.
4. Implementierung einer automatisierten PDF-Rechnungserstellung (Stunden, Kilometer, Preise).
