# CONTEXT.md – Aktuelle Sitzung

⚠️ Max. 20 Zeilen. @workspace-janitor bereinigt diese Datei nach jeder Sitzung.

---

## Aktueller Stand
- **Login-System:** Hybrid-Login (logins.json + DB) mit Passwort-Sichtbarkeits-Toggle.
- **UI & UX:** 'Erstellt von'-Anzeige im Dashboard, 2-stellige Datumsformatierung.
- **Dokumente:** PDF-Templates (Datenschutz, Vertrag) visuell aufbereitet, Bedienungsanleitung als PDF hinterlegt.
- **Sicherheit:** Path Traversal & Race Conditions (Rechnungsnummern) behoben.

## Nächste Schritte
- **Fokus:** Kunden-Validierung (E-Mail-Format, Pflichtfelder im Backend).
- **Kommunikation:** SMTP-Integration (aktuell nur `println!`).
- Platzhalter (Firma, Domain) in PDF-Templates austauschen.
