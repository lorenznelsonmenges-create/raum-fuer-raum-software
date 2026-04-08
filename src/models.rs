use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AuftragStatus {
    AnfrageLaeuft,
    InBearbeitung,
    Abgeschlossen,
    Storniert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kunde {
    pub id: i64,
    #[serde(default)]
    pub vorname: String,
    #[serde(default)]
    pub nachname: String,
    pub strasse: Option<String>,
    pub hausnummer: Option<String>,
    pub plz: Option<String>,
    pub ort: Option<String>,
    pub email: Option<String>,
    pub telefon: Option<String>,
    pub notizen: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auftrag {
    pub id: i64,
    pub kunde_id: i64,
    pub status: AuftragStatus,
    #[serde(default)]
    pub beschreibung: String,
    pub basis_pauschale: Option<f64>,
    #[serde(default = "default_stundensatz")]
    pub stundensatz: f64,
    #[serde(default = "default_kilometer_satz")]
    pub kilometer_satz: f64,
    #[serde(default)]
    pub notizen: String,
    #[serde(default)]
    pub einsaetze: Vec<Einsatz>,
    #[serde(default)]
    pub dateien: Vec<Datei>,
    #[serde(default)]
    pub rechnungen: Vec<Rechnung>,
}

fn default_stundensatz() -> f64 { 45.0 }
fn default_kilometer_satz() -> f64 { 0.50 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Einsatz {
    pub id: i64,
    pub auftrag_id: i64,
    pub datum: String,
    pub kilometer: f64,
    pub stunden: f64,
    pub notiz: String,
    pub typ: String, // ARBEIT oder FAHRT
    pub signatur_pfad: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Datei {
    pub id: i64,
    pub auftrag_id: i64,
    pub dateiname: String,
    pub dateipfad: String,
    pub dateityp: String,
    pub hochgeladen_am: String,
    pub kategorie: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rechnung {
    pub id: i64,
    pub auftrag_id: i64,
    pub rechnungs_nummer: String,
    pub datum: String,
    pub gesamt_netto: f64,
    pub gesamt_brutto: f64,
    pub status: String,
    pub pdf_pfad: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RechnungsNotiz {
    pub id: i64,
    pub auftrag_id: i64,
    pub text: String,
    pub auf_rechnung: bool,
}
