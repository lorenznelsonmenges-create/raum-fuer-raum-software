# CONTEXT.md – Aktuelle Sitzung

Dieses Dokument wird am Ende jeder Sitzung vom **Workspace-Janitor** bereinigt oder aktualisiert.

## Was gerade gebaut wird
- Optimierung des Agenten-Systems (Strukturierung, Rollenschärfung).
- Dokumentation des Datenbank-Schemas (`SCHEMA.md`).
- Einführung von Bug-Tracking (`BUGS.md`).

## Bekannte offene Bugs
- Siehe `BUGS.md` (Fokus: `gesamt_netto` 0.0 Problem).

## Letzte Entscheidungen
- `SCHEMA.md` wird als zentrale Referenz für DB-Strukturen genutzt.
- Agenten werden über exklusivere Trigger in ihren `.md`-Files gesteuert.
- `sqlx` Migrationen sind die alleinige Quelle für Schema-Änderungen.
