-- Migration: Add category to files and type to entries
ALTER TABLE dateien ADD COLUMN kategorie TEXT NOT NULL DEFAULT 'SONSTIGES';
ALTER TABLE einsaetze ADD COLUMN typ TEXT NOT NULL DEFAULT 'ARBEIT';
