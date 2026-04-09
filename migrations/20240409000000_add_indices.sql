CREATE INDEX idx_auftraege_kunde_id ON auftraege(kunde_id);
CREATE INDEX idx_einsaetze_auftrag_id ON einsaetze(auftrag_id);
CREATE INDEX idx_dateien_auftrag_id ON dateien(auftrag_id);
CREATE INDEX idx_rechnungen_auftrag_id ON rechnungen(auftrag_id);
