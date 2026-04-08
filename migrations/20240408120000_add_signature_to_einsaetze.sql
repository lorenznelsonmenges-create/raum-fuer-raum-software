-- Migration: Add signatur_pfad to einsaetze
ALTER TABLE einsaetze ADD COLUMN signatur_pfad TEXT;
