# CONTEXT.md – Aktuelle Sitzung

⚠️ Max. 20 Zeilen. @workspace-janitor bereinigt diese Datei nach jeder Sitzung.

---

## Finaler Status der heutigen Änderungen
- **Suchfelder:** Globale Suche (Dashboard) und spezifische Filter für Kunden/Aufträge implementiert.
- **Archiv-Header:** Getrennte Ansicht für abgeschlossene/stornierte Aufträge ("Archiv") integriert.
- **Branding:** UI-Branding ("achtsam entrümpeln") in der Sidebar und im Dashboard-Layout konsolidiert.
- **Robuste Lösch-Logik:** Kaskadierendes Löschen von Kunden/Aufträgen inklusive physischer Datei-Bereinigung (Signaturen, Uploads).
- **DB-Indizes:** Performance-Optimierung durch Indizes auf Fremdschlüsselspalten (`20240409000000_add_indices.sql`).
- **Foreign Key Support:** SQLite erzwingt nun aktiv Fremdschlüssel-Constraints.

## Nächste Schritte
- PDF-Rechnungserstellung finalisieren.
- API-Absicherung (Authentifizierung).
