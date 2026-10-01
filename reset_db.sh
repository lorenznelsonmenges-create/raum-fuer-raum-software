#!/bin/bash
# ============================================================
# Wendepunkt – Datenbank & Uploads Reset
# ACHTUNG: Löscht ALLE Kunden, Aufträge, Rechnungen, Dateien!
# ============================================================

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET="${1:-prod}"
if [ "$TARGET" = "test" ] || [ "$TARGET" = "achtsam_test.db" ]; then
    DB_FILE="$SCRIPT_DIR/achtsam_test.db"
else
    DB_FILE="$SCRIPT_DIR/achtsam.db"
fi
UPLOADS_DIR="$SCRIPT_DIR/uploads"

echo "============================================"
echo "  Wendepunkt – RESET (Ziel: $(basename "$DB_FILE"))"
echo "============================================"
echo ""
echo "WARNUNG: Alle Daten werden unwiderruflich gelöscht!"
echo "  - Datenbank: $DB_FILE"
echo "  - Uploads:   $UPLOADS_DIR"
echo ""
read -p "Bist du sicher? (ja/nein): " confirm

if [ "$confirm" != "ja" ]; then
    echo "Abgebrochen."
    exit 0
fi

# 1. Datenbank löschen
if [ -f "$DB_FILE" ]; then
    rm -f "$DB_FILE"
    echo "✓ Datenbank gelöscht: $DB_FILE"
else
    echo "  Keine Datenbank gefunden (bereits leer)."
fi

# 2. Uploads behandeln
if [ "$TARGET" = "test" ] || [ "$TARGET" = "achtsam_test.db" ]; then
    echo "ℹ Uploads wurden beibehalten (im Test-Modus bleiben Dateien geschützt)."
else
    if [ -d "$UPLOADS_DIR" ]; then
        find "$UPLOADS_DIR" -type f -delete
        echo "✓ Uploads geleert: $UPLOADS_DIR"
    else
        mkdir -p "$UPLOADS_DIR"
        echo "✓ Uploads-Ordner erstellt: $UPLOADS_DIR"
    fi
fi

echo ""
echo "Fertig! Die Datenbank wird beim nächsten Start"
echo "automatisch neu erstellt (sqlx migrate)."
echo ""
echo "Server neu starten mit:"
echo "  systemctl restart wendepunkt"
echo "  # oder lokal:"
echo "  cargo run"
