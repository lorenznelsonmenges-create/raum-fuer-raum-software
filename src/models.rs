use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum AuftragStatus {
    #[default]
    AnfrageLaeuft,
    InBearbeitung,
    Abgeschlossen,
    Storniert,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Kunde {
    #[serde(default)]
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
    #[serde(default)]
    pub id: i64,
    pub kunde_id: i64,
    #[serde(default)]
    pub status: AuftragStatus,
    #[serde(default)]
    pub beschreibung: String,
    #[serde(default)]
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
    #[serde(default, alias = "rechnungs_notizen")]
    pub rechnungs_notizen: Vec<RechnungNotiz>,
}

impl Default for Auftrag {
    fn default() -> Self {
        Self {
            id: 0,
            kunde_id: 0,
            status: AuftragStatus::default(),
            beschreibung: String::new(),
            basis_pauschale: None,
            stundensatz: default_stundensatz(),
            kilometer_satz: default_kilometer_satz(),
            notizen: String::new(),
            einsaetze: Vec::new(),
            dateien: Vec::new(),
            rechnungen: Vec::new(),
            rechnungs_notizen: Vec::new(),
        }
    }
}

fn default_stundensatz() -> f64 { 0.0 }
fn default_kilometer_satz() -> f64 { 0.0 }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Einsatz {
    #[serde(default)]
    pub id: i64,
    pub auftrag_id: i64,
    pub datum: String,
    pub kilometer: f64,
    pub stunden: f64,
    pub notiz: String,
    pub typ: String, // ARBEIT oder FAHRT
    pub signatur_pfad: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Datei {
    #[serde(default)]
    pub id: i64,
    pub auftrag_id: i64,
    pub dateiname: String,
    pub dateipfad: String,
    pub dateityp: String,
    pub hochgeladen_am: String,
    pub kategorie: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Rechnung {
    #[serde(default)]
    pub id: i64,
    pub auftrag_id: i64,
    pub rechnungs_nummer: String,
    pub datum: String,
    pub gesamt_netto: f64,
    pub gesamt_brutto: f64,
    pub status: String,
    pub pdf_pfad: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RechnungNotiz {
    #[serde(default)]
    pub id: i64,
    pub auftrag_id: i64,
    pub text: String,
    pub auf_rechnung: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DashboardStats {
    pub anfrage_laeuft: i64,
    pub in_bearbeitung: i64,
    pub abgeschlossen: i64,
    pub storniert: i64,
    pub aktuelle_auftraege: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub id: i64,
    pub stundensatz: f64,
    pub kilometer_satz: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            id: 1,
            stundensatz: 45.0,
            kilometer_satz: 0.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub role: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
