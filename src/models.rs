use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AuftragStatus {
    Angefragt,
    Besichtigt,
    AngebotErstellt,
    Durchfuehrung,
    Abgeschlossen,
    Storniert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kunde {
    pub id: i64,
    pub vorname: String,
    pub nachname: String,
    pub strasse: String,
    pub hausnummer: String,
    pub plz: String,
    pub stadt: String,
    pub email: String,
    pub telefon: String,
    pub notizen: String,
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
    pub rechnungs_notizen: Vec<RechnungsNotiz>, // Neu: Notizen für die Rechnung
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RechnungsNotiz {
    pub id: i64,
    pub auftrag_id: i64,
    pub text: String,
    pub auf_rechnung: bool, // Der "Haken" für die Rechnung
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Einsatz {
    pub id: i64,
    pub auftrag_id: i64,
    pub datum: String,
    pub kilometer: f64,
    pub stunden: f64,
    pub notiz: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Datei {
    pub id: i64,
    pub auftrag_id: i64,
    pub dateiname: String,
    pub dateipfad: String,
    pub dateityp: String,
    pub hochgeladen_am: String,
}

impl Kunde {
    pub fn new(id: i64, vorname: String, nachname: String) -> Self {
        Self {
            id,
            vorname,
            nachname,
            strasse: String::new(),
            hausnummer: String::new(),
            plz: String::new(),
            stadt: String::new(),
            email: String::new(),
            telefon: String::new(),
            notizen: String::new(),
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
            rechnungs_notizen: Vec::new(),
        }
    }
}
