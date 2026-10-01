# Offene Bugs

Hier werden bekannte Fehler gesammelt, damit der **Tester** eine klare Arbeitsliste hat.

## Nächster Fokus
- [ ] **Kunden-Validierung**: Validierung für E-Mail-Format fehlt im Backend (führt zu 500er statt 400er Fehler).

## Sicherheit (Offen)
- [ ] **Path Traversal beim Datei-Upload (Aufträge)**: `files.rs` schreibt nach `uploads/{auftrag_id}_{dateiname}` mit dem ungeprüften Dateinamen des Clients – ein Name wie `../../x` schreibt außerhalb von `uploads/`. Lösung wie bei den Buchungsbelegen: serverseitig erzeugter UUID-Name, Originalname nur zur Anzeige (`BelegDateiname`), Typ per Magic Bytes.
- [ ] **Port 3001 öffentlich erreichbar**: Backend bindet auf `0.0.0.0:3001` (`main.rs`) – auf dem Server direkt per HTTP erreichbar, an Nginx/HTTPS vorbei. Lösung: auf `127.0.0.1` binden (z.B. über Env-Variable) und/oder Port per `ufw` sperren.
- [ ] **Klartext-Passwörter im Repo**: `migrations/20260924000001_ensure_users.sql` enthält die Passwörter von `admin` und `stefanie` als Kommentar. Kommentare entfernen (neue Migration ändert nichts an der Git-Historie → Passwörter ändern).
- [ ] **DB-Fehlerdetails an Client**: `AppError::Sqlx` gibt `e.to_string()` an den Browser zurück (Kunden, Aufträge …). Buchhaltung loggt stattdessen und meldet generisch – Muster übernehmen.

## Buchhaltung – fachlich offen (bewusst noch nicht umgesetzt)
- [ ] **Jahresabschluss / Festschreibung**: Tabelle `geschaeftsjahre(jahr, abgeschlossen_am, abgeschlossen_von)`. Abschluss nur, wenn im Jahr keine Buchung ohne Beleg existiert (`anzahl_ohne_beleg = 0`). Danach Anlegen/Ändern/Löschen **und** Beleg-Ersetzen für Buchungen dieses Jahres sperren; Korrekturen per Gegenbuchung im offenen Jahr.
- [ ] **Aufbewahrungspflicht (§ 147 AO)**: Beim Ersetzen eines Belegs und beim Löschen einer Buchung wird die Beleg-Datei heute physisch gelöscht (`entferne_beleg` in `main.rs`). Fachlich klären: Storno statt Löschen, alte Belege archivieren statt löschen.
- [ ] **USt-Ausweis**: Buchungen speichern nur einen Bruttobetrag. Für EÜR/UStVA netto und USt getrennt erfassen (inkl. Steuersatz).

## Kundendokumente / Rechtliches (Offen)
- [ ] **Rechnung vs. Kleinunternehmer**: `templates/rechnung.html` weist 19 % USt aus (`pdf.rs` berechnet `vat_19_percent`), geplant ist § 19 UStG. Dann: keine USt, Pflichthinweis auf die Steuerbefreiung (§ 34a UStDV), Steuernummer. Außerdem steht im Footer eine **Dummy-IBAN/BIC**.
- [ ] **Unterschrift auf dem Tablet nicht erreichbar**: Die UI hat keine Signatur-Erfassung; Uploads gehen immer als `SONSTIGES`. Der `SIGNATUR`-Zweig in `files.rs` erzeugt nur das Datenschutz-PDF (ohne angekreuzte Einwilligungen), nie den Vertrag. Nötig: Signatur-Erfassung je Dokument inkl. Checkbox-Status (Anhang 3, Einwilligungen).
- [ ] **Website-Impressum veraltet**: „§ 5 TMG“ → § 5 DDG, „§ 55 Abs. 2 RStV“ → § 18 Abs. 2 MStV; Link zur OS-Plattform entfernen (seit 20.07.2025 abgeschaltet). Website-Datenschutzerklärung unvollständig (Rechtsgrundlagen, Speicherdauer, Widerspruch, Beschwerderecht, Kontaktformular/Mailer).
- [ ] **Altlasten**: `templates/vertrag.typ` und `templates/datenschutz.typ` werden nirgends verwendet.
- [x] **PDFs im US-Letter-Format**: `print_to_pdf` ignorierte `@page { size: A4 }`. Behoben in `pdf.rs` (`prefer_css_page_size`, `print_background`).
- [x] **Unterschriftsbild erschien nicht im PDF** (`file://` aus `data:`-URL blockiert). Behoben: `pdf.rs` bettet das Bild als Data-URI ein (`signatur_bild`).

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

