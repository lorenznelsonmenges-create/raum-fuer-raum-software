# CONTEXT.md – Aktuelle Sitzung

⚠️ Max. 20 Zeilen.

---

## Aktueller Stand (01.10.2026)
- **Auf `main`:** Buchhaltung (`b94c713`), Stundensatz Nachbereitung (`590221f`), Belegpflicht (`c3e09f6`), Geschäftsjahr + Dashboard (`fad6a2f`).
- **Geschäftsjahr:** `Geschaeftsjahr` (Kalenderjahr, nur in `domain.rs`), `?jahr=` für Liste/Übersicht, `GET /api/buchungen/jahre`, `anzahl_ohne_beleg`, zweigeteiltes Dashboard, Jahresfilter in der Buchhaltung.
- **Fachlich:** EÜR (keine Bilanz), Geschäftsjahr = Kalenderjahr, Jahr wird aus `datum` (Zahlungsdatum) abgeleitet.
- **Server:** `/var/www/achtsam-backend`, `achtsam.service`, Port 3001; Nginx-Uploadlimit 50 MB.

## Nächste Schritte
1. Auf dem Server ausrollen (Backup, `git pull`, `cargo build --release`, `systemctl restart achtsam`).
2. Offen (siehe `BUGS.md`): Jahresabschluss/Festschreibung, Aufbewahrung § 147 AO (Belege nicht löschen), USt-Ausweis.
3. Sicherheit (siehe `BUGS.md`): Path Traversal in `files.rs`, Port 3001 absichern, Passwörter in Migration.
