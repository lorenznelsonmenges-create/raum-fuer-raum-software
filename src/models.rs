use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AuftragStatus {
    Angefragt,
    Besichtigt,
    Durchfuehrung,
    Abgeschlossen,
    Storniert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kunde {
    pub id: i64,
    pub vorname: String,
    pub nachname: String,
    pub strasse: Option<String>,
    pub hausnummer: Option<String>,
    pub plz: Option<String>,
    pub ort: Option<String>,
    pub email: Option<String>,
    pub telefon: Option<String>,
    pub notizen: Option<String>,
    pub auftraege: Vec<Auftrag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auftrag {
    pub id: i64,
    pub kunde_id: i64,
    pub status: AuftragStatus,
    pub beschreibung: String,
    pub basis_pauschale: Option<f64>, // Optional
    pub preis_manuell: Option<f64>,   // Neu: Manuelle Preisanpassung
    pub notizen: String,             // Interne Notizen
    pub einsaetze: Vec<Einsatz>,
    pub dateien: Vec<Datei>,
    pub rechnungen: Vec<Rechnung>,   // Liste der Rechnungen
    pub rechnungs_notizen: Vec<RechnungsNotiz>, // Notizen für die Rechnung
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RechnungsNotiz {
    pub id: i64,
    pub auftrag_id: i64,
    pub text: String,
    pub auf_rechnung: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Einsatz {
    pub id: i64,
    pub auftrag_id: i64,
    pub datum: String,
    pub kilometer: f64,
    pub stunden: f64,
    pub notiz: String,
    pub typ: String, // ARBEIT, FAHRT
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Datei {
    pub id: i64,
    pub auftrag_id: i64,
    pub dateiname: String,
    pub dateipfad: String,
    pub dateityp: String,
    pub hochgeladen_am: String,
    pub kategorie: String, // DATENSCHUTZ, VERTRAG, RAHMENBEDINGUNGEN, SONSTIGES, RECHNUNG
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rechnung {
    pub id: i64,
    pub auftrag_id: i64,
    pub rechnungs_nummer: String,
    pub datum: String,
    pub gesamt_netto: f64,
    pub gesamt_brutto: f64,
    pub pdf_pfad: String,
    pub status: String, // OFFEN, BEZAHLT, STORNIERT
}

impl Kunde {
    pub fn new(id: i64, vorname: String, nachname: String) -> Self {
        Self {
            id,
            vorname,
            nachname,
            strasse: None,
            hausnummer: None,
            plz: None,
            ort: None,
            email: None,
            telefon: None,
            notizen: None,
            auftraege: Vec::new(),
        }
    }
}

impl Auftrag {
    pub fn new(id: i64, kunde_id: i64) -> Self {
        Self {
            id,
            kunde_id,
            status: AuftragStatus::Angefragt,
            beschreibung: String::new(),
            basis_pauschale: None,
            preis_manuell: None,
            notizen: String::new(),
            einsaetze: Vec::new(),
            dateien: Vec::new(),
            rechnungen: Vec::new(),
            rechnungs_notizen: Vec::new(),
        }
    }
}
