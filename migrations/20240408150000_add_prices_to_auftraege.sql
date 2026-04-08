-- Add prices to auftraege
ALTER TABLE auftraege ADD COLUMN stundensatz REAL DEFAULT 45.0;
ALTER TABLE auftraege ADD COLUMN km_satz REAL DEFAULT 0.50;
