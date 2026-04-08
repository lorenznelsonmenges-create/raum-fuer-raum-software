# Dokumentation: Sub-Agenten Team

Dieser Ordner dient zur Übersicht für menschliche Nutzer. Die tatsächlichen Definitionen der Agenten liegen in `.gemini/agents/`.

## Übersicht der Rollen

### 1. Orchestrator (Ich - der Haupt-Agent)
- **Rolle**: Projektmanager & Schnittstelle zum Nutzer.
- **Prinzip**: Koordination über die "Globale Checkliste" (Zero-Ping-Pong).

### 2. @rust-backend-expert (Entwickler)
- **Rolle**: Experte für Rust, Axum & SQLx.
- **Wichtig**: Beachtet streng die Architektur-Disziplin & DRY-Regel.

### 3. @code-reviewer (Kritiker)
- **Rolle**: Findet Fehler, bevor sie zum Problem werden.
- **Wichtig**: Prüft auf Best Practices und Security.

### 4. @tester (QA-Agent)
- **Rolle**: Schreibt automatisierte Tests (cargo test).
- **Wichtig**: Deckt Randfälle ab und reproduziert Bugs durch Tests.

### 5. @workspace-janitor (Hausmeister)
- **Rolle**: Hält den Workspace sauber.
- **Wichtig**: Reduziert Kontext-Overhead durch Artefakt-Bereinigung.

## Gemeinsame Standards
Alle Agenten (und der Orchestrator) unterliegen der **"Globalen Checkliste"** in der `GEMINI.md`. Das bedeutet: Kein Feedback ohne vorherigen `cargo check` oder `cargo build` ("Zero-Ping-Pong").
