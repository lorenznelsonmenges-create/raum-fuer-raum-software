# CONTEXT.md – Aktuelle Sitzung

⚠️ Max. 20 Zeilen. @workspace-janitor bereinigt diese Datei nach jeder Sitzung.

---

## Was gerade gebaut wurde
- **Backend-Logging:** Einfache Konsolen-Ausgaben (println!/eprintln!) für Auftrag-Löschvorgänge hinzugefügt.
- **Kaskadierendes Löschen:** Kunden- und Auftrags-Löschung bereinigt nun auch physische Dateien (Signaturen, Rechnungen, Uploads) von der Festplatte.
- **Datenbank-Optimierung:** Neue Indizes für Fremdschlüssel-Beziehungen (kunde_id, auftrag_id) zur Performance-Steigerung angelegt.
- **Foreign Key Support:** SQLite-Verbindung erzwingt nun Fremdschlüssel-Constraints.

## Bekannte offene Punkte dieser Sitzung
- PDF-Rechnungserstellung (Stunden-Zusammenfassung & Steuersatz).
- Absicherung der API (Login).

## Letzte Entscheidungen
- Beim Löschen eines Auftrags werden alle zugehörigen Dateien unwiderruflich von der Festplatte gelöscht.
