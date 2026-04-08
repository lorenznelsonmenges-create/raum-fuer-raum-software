# CONTEXT.md – Aktuelle Sitzung

⚠️ Max. 20 Zeilen. @workspace-janitor bereinigt diese Datei nach jeder Sitzung.

---

## Was gerade gebaut wird
- Dokumenten-Feature Infrastruktur (Upload-Handler, DB-Anbindung) und Feedback implementiert.

## Bekannte offene Punkte dieser Sitzung
- PDF-Rechnungserstellung (Finalisierung des Designs).
- Absicherung der API (Login).

## Letzte Entscheidungen
- Dokumenten-Feedback erfolgt via Toast/Alert (Frontend-seitig vorbereitet).
- PDF-Workflow ist fix: Handlebars -> HTML -> headless_chrome -> PDF.
