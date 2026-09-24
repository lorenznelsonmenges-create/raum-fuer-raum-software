use serde::{Deserialize, Serialize};
use crate::domain::{Euro, Stunden, Kilometer, EinsatzTyp, RechnungsNummer};
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
    pub basis_pauschale: Option<Euro>,
    #[serde(default)]
    pub stundensatz: Euro,
    #[serde(default)]
    pub kilometer_satz: Euro,
    #[serde(default)]
    pub notizen: String,
    pub created_by: Option<String>,
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
            stundensatz: Euro::default(),
            kilometer_satz: Euro::default(),
            notizen: String::new(),
            created_by: None,
            einsaetze: Vec::new(),
            dateien: Vec::new(),
            rechnungen: Vec::new(),
            rechnungs_notizen: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Einsatz {
    #[serde(default)]
    pub id: i64,
    pub auftrag_id: i64,
    pub datum: String,
    pub kilometer: Kilometer,
    pub stunden: Stunden,
    pub notiz: String,
    pub typ: EinsatzTyp,
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
    pub rechnungs_nummer: RechnungsNummer,
    pub datum: String,
    pub gesamt_netto: Euro,
    pub gesamt_brutto: Euro,
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
    pub stundensatz: Euro,
    pub kilometer_satz: Euro,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            id: 1,
            stundensatz: Euro::from_cents(4500).unwrap(),
            kilometer_satz: Euro::from_cents(50).unwrap(),
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
