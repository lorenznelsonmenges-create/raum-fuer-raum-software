# CONTEXT.md – Aktuelle Sitzung

⚠️ Max. 20 Zeilen.

---

## Aktueller Stand (24.09.2026)
- **Rebranding:** "Wendepunkt", Mail `info@wendepunkt-ruf.de`, Domain `wendepunkt-ruf.de`.
- **Sicherheit & Auth:** `logins.json` entfernt. Migration `20260924000001_ensure_users.sql` für `admin` & `stefanie` via Bcrypt.
- **Backend & Tests:** Alle Tests (`cargo test`) grün (Auth, PDF, DB-Stats, Rechnungen).
- **Hosting & Server:** Hetzner Cloud VPS (`46.62.148.232`), `app.wendepunkt-ruf.de` & `wendepunkt-ruf.de`.
- **Doku:** `Architektur.md`, `GEMINI.md` und Datenschutz-Hoster auf Hetzner aktualisiert.
- **Ordner:** Umbenannt von `Achtsam_entruempeln` nach `Wendepunkt`.

## Nächste Schritte
1. Auf Server (`46.62.148.232`): `git pull` & `cargo build --release` & `systemctl restart wendepunkt`.
2. Status & Login (`stefanie` / `admin`) im Web testen.
