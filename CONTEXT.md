# CONTEXT.md – Aktuelle Sitzung

⚠️ Max. 20 Zeilen.

---

## Aktueller Stand (01.10.2026)
- **Buchhaltung:** Modul (Einnahmen/Ausgaben, Übersicht, Filter) auf `main` gepusht (Commit `b94c713`), Tests grün.
- **Uncommitted lokal:** Stundensatz Nachbereitung (inkl. Migration `20260928120000`), `DATABASE_URL`/`dotenvy`, `reset_db.sh test`, `pdf.rs`.
- **Server:** `/var/www/achtsam-backend`, Dienst `achtsam.service`, Port 3001 (Port 3000 = carsharing-backend).
- **Doku:** `AGENTS.md`, `SCHEMA.md`, `Architektur.md`, `BUGS.md` auf Stand 01.10.2026 gebracht.

## Nächste Schritte
1. Lokale Änderungen (Stundensatz Nachbereitung etc.) committen & pushen.
2. Server: Backup `achtsam.db`, `git pull`, `cargo build --release`, `systemctl restart achtsam`.
3. Buchhaltung im Browser testen (`app.wendepunkt-ruf.de` → Buchhaltung).
4. Sicherheitspunkte aus `BUGS.md` angehen (Port 3001 absichern).
