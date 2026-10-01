use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use crate::domain::{Euro, Stunden, Kilometer, EinsatzTyp, RechnungsNummer};
use crate::domain::{BuchungsTyp, BuchungsBetrag, Kategorie, Beschreibung, BelegReferenz, BuchungsDatum, parse_iso_datum};
use crate::domain::{BelegFormat, BelegDateiname, BelegPfad, Geschaeftsjahr};
use crate::error::AppError;
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
    pub stundensatz_nachbereitung: Euro,
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
            stundensatz_nachbereitung: Euro::default(),
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
    #[serde(default)]
    pub unterkategorie: Option<String>,
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
    #[serde(default)]
    pub stundensatz_nachbereitung: Euro,
    pub kilometer_satz: Euro,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            id: 1,
            stundensatz: Euro::from_cents(4500).unwrap(),
            stundensatz_nachbereitung: Euro::from_cents(4500).unwrap(),
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


// =====================================================================
// Buchhaltung
// =====================================================================

/// Entity (DDD): Eine Buchung hat eine Identität (`id`). Zwei Buchungen sind genau
/// dann gleich, wenn ihre IDs gleich sind – unabhängig von Betrag oder Datum.
///
/// Alle fachlichen Felder sind Domain Primitives (Value Objects) und damit garantiert
/// gültig. Die Felder sind nur crate-intern sichtbar (Immutability): Außerhalb der
/// Anwendung kann eine Buchung gelesen, aber nicht verändert werden.
#[derive(Debug, Clone, Serialize)]
pub struct Buchung {
    pub(crate) id: i64,
    pub(crate) buchungs_typ: BuchungsTyp,
    pub(crate) betrag: BuchungsBetrag,
    pub(crate) kategorie: Kategorie,
    pub(crate) beschreibung: Beschreibung,
    pub(crate) datum: BuchungsDatum,
    pub(crate) auftrag_id: Option<i64>,
    pub(crate) beleg_referenz: Option<BelegReferenz>,
    /// Angehängte Beleg-Datei. `None` nur bei Altbuchungen aus der Zeit vor der Belegpflicht.
    pub(crate) beleg: Option<BelegMeta>,
    pub(crate) created_at: String,
    /// Traceability (CIA-T): wird ausschließlich serverseitig aus der Session gesetzt
    pub(crate) created_by: i64,
}

impl PartialEq for Buchung {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Buchung {}

impl Buchung {
    pub fn id(&self) -> i64 { self.id }
    pub fn buchungs_typ(&self) -> BuchungsTyp { self.buchungs_typ }
    pub fn betrag(&self) -> BuchungsBetrag { self.betrag }
    pub fn kategorie(&self) -> &Kategorie { &self.kategorie }
    pub fn beschreibung(&self) -> &Beschreibung { &self.beschreibung }
    pub fn datum(&self) -> BuchungsDatum { self.datum }
    pub fn auftrag_id(&self) -> Option<i64> { self.auftrag_id }
    pub fn beleg_referenz(&self) -> Option<&BelegReferenz> { self.beleg_referenz.as_ref() }
    pub fn beleg(&self) -> Option<&BelegMeta> { self.beleg.as_ref() }
    pub fn created_at(&self) -> &str { &self.created_at }
    pub fn created_by(&self) -> i64 { self.created_by }
}

/// Value Object: Metadaten einer gespeicherten Beleg-Datei.
/// JSON: `{ "dateiname": "...", "typ": "application/pdf", "url": "/uploads/belege/<uuid>.pdf" }`
/// – der interne Speicherpfad selbst wird nicht als Feld herausgegeben, nur die Abruf-URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BelegMeta {
    pub pfad: BelegPfad,
    pub dateiname: BelegDateiname,
    pub format: BelegFormat,
}

impl Serialize for BelegMeta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        use serde::ser::SerializeStruct;
        let mut st = serializer.serialize_struct("BelegMeta", 3)?;
        st.serialize_field("dateiname", self.dateiname.as_str())?;
        st.serialize_field("typ", self.format.mime())?;
        st.serialize_field("url", &self.pfad.url())?;
        st.end()
    }
}

/// Roh-Eingabe vom Client (DTO). Enthält bewusst nur primitive Typen – sie ist
/// NICHT vertrauenswürdig und wird über `validieren()` in `BuchungsDaten` überführt.
///
/// `deny_unknown_fields`: Felder wie `id`, `created_by` oder `created_at` können
/// vom Client nicht eingeschleust werden (Mass-Assignment-Schutz).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuchungEingabe {
    pub buchungs_typ: String,
    /// Betrag in Euro (z.B. 12.5) – nur das JSON-Transportformat. Wird sofort über
    /// `Euro::from_euro_f64()` in Cent (i64) umgewandelt; gerechnet wird nie mit f64.
    pub betrag: f64,
    pub kategorie: String,
    #[serde(default)]
    pub beschreibung: String,
    pub datum: String,
    #[serde(default)]
    pub auftrag_id: Option<i64>,
    #[serde(default)]
    pub beleg_referenz: Option<String>,
}

impl BuchungEingabe {
    /// Input Validation Stufen 3 + 4 (Lexical Content, Syntax) über die Domain
    /// Primitives. Stufe 5 (Semantic, z.B. Existenz des Auftrags) folgt im Handler,
    /// weil sie die Datenbank benötigt und damit die teuerste Prüfung ist.
    pub fn validieren(&self) -> Result<BuchungsDaten, AppError> {
        let buchungs_typ = BuchungsTyp::parse(&self.buchungs_typ)?;
        let euro = Euro::from_euro_f64(self.betrag).map_err(AppError::BadRequest)?;
        let betrag = BuchungsBetrag::new(euro)?;
        let kategorie = Kategorie::new(&self.kategorie)?;
        let beschreibung = Beschreibung::new(&self.beschreibung)?;
        let datum = BuchungsDatum::new(&self.datum)?;
        let auftrag_id = match self.auftrag_id {
            Some(id) if id <= 0 => return Err(AppError::BadRequest("Ungültige Auftrags-ID".into())),
            other => other,
        };
        let beleg_referenz = match self.beleg_referenz.as_deref().map(str::trim) {
            None | Some("") => None,
            Some(v) => Some(BelegReferenz::new(v)?),
        };
        Ok(BuchungsDaten { buchungs_typ, betrag, kategorie, beschreibung, datum, auftrag_id, beleg_referenz })
    }
}

/// Validierte fachliche Daten einer Buchung (ohne Identität und ohne Audit-Felder).
/// Kann nur aus gültigen Domain Primitives zusammengesetzt werden.
#[derive(Debug, Clone)]
pub struct BuchungsDaten {
    pub buchungs_typ: BuchungsTyp,
    pub betrag: BuchungsBetrag,
    pub kategorie: Kategorie,
    pub beschreibung: Beschreibung,
    pub datum: BuchungsDatum,
    pub auftrag_id: Option<i64>,
    pub beleg_referenz: Option<BelegReferenz>,
}

/// Roh-Query-Parameter für `GET /api/buchungen` (leere Strings = kein Filter).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuchungsFilterParameter {
    pub von: Option<String>,
    pub bis: Option<String>,
    /// Geschäftsjahr (JJJJ) – Alternative zu `von`/`bis`, nicht kombinierbar
    pub jahr: Option<String>,
    pub typ: Option<String>,
    pub kategorie: Option<String>,
}

/// Validierter Zeitraum (beide Grenzen inklusive).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Zeitraum {
    pub von: Option<NaiveDate>,
    pub bis: Option<NaiveDate>,
}

impl Zeitraum {
    /// Precondition: Datumsangaben im Format YYYY-MM-DD, `von` <= `bis`
    /// (Filter-Daten dürfen in der Zukunft liegen, z.B. "bis Monatsende").
    pub fn new(von: Option<&str>, bis: Option<&str>) -> Result<Self, AppError> {
        let parse = |v: Option<&str>| match v.map(str::trim) {
            None | Some("") => Ok(None),
            Some(s) => parse_iso_datum(s).map(Some),
        };
        let zeitraum = Self { von: parse(von)?, bis: parse(bis)? };
        if let (Some(v), Some(b)) = (zeitraum.von, zeitraum.bis) {
            if v > b {
                return Err(AppError::BadRequest("Zeitraum ungültig: 'Von' liegt nach 'Bis'".into()));
            }
        }
        Ok(zeitraum)
    }

    /// Zeitraum aus den Query-Parametern: entweder `jahr` ODER `von`/`bis`.
    /// Beides zusammen ist widersprüchlich und wird abgelehnt (400), statt still
    /// eine der Angaben zu ignorieren.
    pub fn aus_parametern(von: Option<&str>, bis: Option<&str>, jahr: Option<&str>) -> Result<Self, AppError> {
        let gesetzt = |v: Option<&str>| v.map(str::trim).is_some_and(|s| !s.is_empty());
        if !gesetzt(jahr) {
            return Self::new(von, bis);
        }
        if gesetzt(von) || gesetzt(bis) {
            return Err(AppError::BadRequest("'jahr' kann nicht zusammen mit 'von'/'bis' verwendet werden".into()));
        }
        Ok(Self::aus_geschaeftsjahr(Geschaeftsjahr::parse(jahr.unwrap_or_default())?))
    }

    /// Die Jahresgrenzen kommen ausschließlich aus `Geschaeftsjahr::zeitraum()`.
    pub fn aus_geschaeftsjahr(jahr: Geschaeftsjahr) -> Self {
        let (von, bis) = jahr.zeitraum();
        Self { von: Some(von), bis: Some(bis) }
    }
}

/// Validierter Filter für die Buchungsliste.
#[derive(Debug, Clone, Default)]
pub struct BuchungsFilter {
    pub zeitraum: Zeitraum,
    pub typ: Option<BuchungsTyp>,
    pub kategorie: Option<Kategorie>,
}

impl BuchungsFilterParameter {
    pub fn validieren(&self) -> Result<BuchungsFilter, AppError> {
        let zeitraum = Zeitraum::aus_parametern(self.von.as_deref(), self.bis.as_deref(), self.jahr.as_deref())?;
        let typ = match self.typ.as_deref().map(str::trim) {
            None | Some("") => None,
            Some(t) => Some(BuchungsTyp::parse(t)?),
        };
        let kategorie = match self.kategorie.as_deref().map(str::trim) {
            None | Some("") => None,
            Some(k) => Some(Kategorie::new(k)?),
        };
        Ok(BuchungsFilter { zeitraum, typ, kategorie })
    }
}

/// Roh-Query-Parameter für `GET /api/buchungen/uebersicht`.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ZeitraumParameter {
    pub von: Option<String>,
    pub bis: Option<String>,
    /// Geschäftsjahr (JJJJ) – Alternative zu `von`/`bis`, nicht kombinierbar
    pub jahr: Option<String>,
}

impl ZeitraumParameter {
    pub fn validieren(&self) -> Result<Zeitraum, AppError> {
        Zeitraum::aus_parametern(self.von.as_deref(), self.bis.as_deref(), self.jahr.as_deref())
    }
}

/// Eintrag für die Jahresauswahl (`GET /api/buchungen/jahre`). Liefert die Grenzen
/// mit, damit das Frontend die Kalenderjahr-Regel nicht selbst nachbauen muss.
#[derive(Debug, Clone, Serialize)]
pub struct GeschaeftsjahrInfo {
    pub jahr: Geschaeftsjahr,
    pub von: String,
    pub bis: String,
}

impl From<Geschaeftsjahr> for GeschaeftsjahrInfo {
    fn from(jahr: Geschaeftsjahr) -> Self {
        let (von, bis) = jahr.zeitraum();
        Self { jahr, von: von.format("%Y-%m-%d").to_string(), bis: bis.format("%Y-%m-%d").to_string() }
    }
}

/// Übersicht für das Buchhaltungs-Dashboard.
#[derive(Debug, Clone, Default, Serialize)]
pub struct BuchungsUebersicht {
    /// Summe aller Einnahmen (JSON: Euro als Zahl, wie alle `Euro`-Felder)
    pub einnahmen_gesamt: Euro,
    /// Summe aller Ausgaben (JSON: Euro als Zahl)
    pub ausgaben_gesamt: Euro,
    /// Einnahmen - Ausgaben in **Cent** (kann negativ sein, daher kein `Euro`)
    pub saldo: i64,
    /// Buchungen im Zeitraum ohne Beleg-Datei (Altbuchungen vor der Belegpflicht)
    pub anzahl_ohne_beleg: i64,
}
