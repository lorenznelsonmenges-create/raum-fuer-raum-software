# CONTEXT.md – Aktuelle Sitzung

⚠️ Max. 20 Zeilen.

---

## Aktueller Stand (23.09.2026)
- **Rebranding:** "Wendepunkt", Mail `info@wendepunkt-ruf.de`, Domain `wendepunkt-ruf.de`.
- **E-Mail & DNS:** Mailbox in konsoleH aktiv, DNS (MX, SPF, DKIM) bei Hetzner gesetzt, DENIC-Update läuft.
- **Backend:** Crate umbenannt (`wendepunkt-software`), 3 Einsatz-Typen (`ARBEIT_VOR_ORT`, `ARBEIT_VORBEREITUNG`, `KILOMETER`) inkl. Migration & PDF.
- **Templates:** Rechnung, Vertrag, Datenschutz auf Wendepunkt & `info@wendepunkt-ruf.de` umgestellt.
- **Reset:** `reset_db.sh` erstellt für vollständigen Daten-Reset.

## Nächste Schritte (Morgen)
1. **Website:** Branding & Mail in `Website/` anpassen ("Wendepunkt", `info@wendepunkt-ruf.de`).
2. **Server (Hetzner VPS):** Nginx Vhosts für Website (`wendepunkt-ruf.de`) & App (`app.wendepunkt-ruf.de`).
3. **SSL & Deploy:** Certbot für alle Domains, DB Reset auf Server ausführen, App starten.
