# CONTEXT.md – Aktuelle Sitzung

⚠️ Max. 20 Zeilen.

---

## Aktueller Stand (01.10.2026)
- **Buchhaltung:** Modul (Einnahmen/Ausgaben, Übersicht, Filter) auf `main` (Commit `b94c713`), Tests grün.
- **Stundensatz Nachbereitung**, `DATABASE_URL`/`dotenvy`, `reset_db.sh test` committet (`590221f`).
- **Server:** `/var/www/achtsam-backend`, Dienst `achtsam.service`, Port 3001 (Port 3000 = carsharing-backend).
- **Doku:** `AGENTS.md`, `SCHEMA.md`, `Architektur.md`, `BUGS.md` auf Stand 01.10.2026 gebracht.

## Nächste Schritte
1. Server: Backup `achtsam.db`, `git pull`, `cargo build --release`, `systemctl restart achtsam`.
2. Buchhaltung im Browser testen (`app.wendepunkt-ruf.de` → Buchhaltung).
3. Sicherheitspunkte aus `BUGS.md` angehen (Port 3001 absichern).
