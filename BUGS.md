# Offene Bugs

Hier werden bekannte Fehler gesammelt, damit der **Tester** eine klare Arbeitsliste hat.

## Nächster Fokus
- [ ] **Kunden-Validierung**: Validierung für E-Mail-Format fehlt im Backend (führt zu 500er statt 400er Fehler).

## Kritisch (Behoben)
- [x] **Path Traversal in Template-Handlern**: Behoben durch `sanitize_template_path()` in `main.rs` – Whitelist-Ansatz mit Zeichenprüfung und `.html`-Pflicht.
- [x] **Race Condition bei Rechnungsnummern**: Behoben durch `get_next_rechnung_number()` in `database.rs` – nutzt `MAX()` statt `COUNT(*)`.
- [x] **Bcrypt-Hash-Fehler**: Behoben durch Migration '20240411000000' (Hash-Format korrigiert).
- [x] **Kundenanlage**: Behoben (Konsistenz stadt/ort geprüft).
- [x] **Auftragserstellung**: Behoben (Spaltenname km_satz -> kilometer_satz korrigiert).
- [x] **Datenbank-Konsistenz**: `gesamt_netto` wird korrekt gespeichert.

## Mittel (Behoben)
- [x] **PDF-Layout bei langen Notizen**: Behoben durch `page-break-inside: avoid` in `rechnung.html`.
- [x] **Datenschutz-Template leere Seite**: Behoben durch Entfernung von `min-height: 297mm` und Umstellung auf `@page`-CSS.
- [x] **Rechnung: Falsche Schriftarten**: Montserrat/Playfair Display durch Manrope/Noto Serif (Design System) ersetzt.
- [x] **Rechnung: Footer absolute Positionierung**: Footer fließt nun natürlich mit dem Inhalt.
- [x] **UI/UX**: Datei-Upload gibt visuelles Feedback nach Erfolg (behoben).

## Prozess & Disziplin
- [ ] **Role-Drift**: Der Haupt-Agent verliert bei komplexen Korrekturen die Delegations-Disziplin und nimmt eigenständig Änderungen vor (Bruch der Orchestrator-Regel).

