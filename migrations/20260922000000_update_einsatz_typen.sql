-- Migration: Einsatz-Typen auf 3 Kategorien umstellen
-- Alte Typen: ARBEIT, FAHRT
-- Neue Typen: ARBEIT_VOR_ORT, ARBEIT_VORBEREITUNG, KILOMETER

-- Vorhandene Einträge: ARBEIT -> ARBEIT_VOR_ORT, FAHRT -> KILOMETER
UPDATE einsaetze SET typ = 'ARBEIT_VOR_ORT' WHERE typ = 'ARBEIT';
UPDATE einsaetze SET typ = 'KILOMETER' WHERE typ = 'FAHRT';
