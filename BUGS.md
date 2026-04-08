# Offene Bugs

Hier werden bekannte Fehler gesammelt, damit der **Tester** eine klare Arbeitsliste hat.

## Kritisch (Blocker)
- [x] **Kundenanlage**: Behoben (Konsistenz stadt/ort geprüft).
- [x] **Auftragserstellung**: Behoben (Spaltenname km_satz -> kilometer_satz korrigiert).
- [x] **Datenbank-Konsistenz**: `gesamt_netto` wird korrekt gespeichert (war Folgetelehler der fehlerhaften Auftrags-Abfrage).

## Mittel / Niedrig
- [ ] **UI/UX**: Datei-Upload gibt kein visuelles Feedback nach Erfolg (siehe GEMINI.md Schritt 1).
- [ ] **Kunden-Validierung**: Validierung für E-Mail-Format fehlt im Backend (führt zu 500er statt 400er Fehler).
- [ ] **PDF-Generierung**: Bei sehr langen Einsatz-Notizen bricht das Layout in `rechnung.html` unschön um.
