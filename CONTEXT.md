# CONTEXT.md – Aktuelle Sitzung

⚠️ Max. 20 Zeilen. @workspace-janitor bereinigt diese Datei nach jeder Sitzung.

---

## Aktueller Stand
- **Login-System:** Aktiviert und durch Session-Management abgesichert.
- **Bcrypt-Fix:** Hash-Fehler via Migration '20240411000000' behoben (Admin-Account).
- **Input-Sanitization:** 'login_handler' trimmt Benutzernamen/Passwörter automatisch.
- **Infrastruktur:** DNS-Probleme (Cache) beim Nutzer lokalisiert; Zugriff via Mobile OK.

## Nächste Schritte
- **Fokus:** Kunden-Validierung (E-Mail-Format, Pflichtfelder im Backend).
- PDF-Rechnungserstellung (Layout-Fixes bei langen Notizen).
