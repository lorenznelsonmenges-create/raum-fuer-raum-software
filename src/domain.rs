use serde::{Serialize, Deserialize, Serializer, Deserializer};
use chrono::NaiveDate;
use crate::error::AppError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Euro(i64);

impl Euro {
    pub fn from_cents(cents: i64) -> Result<Self, String> {
        if cents < 0 {
            Err("Betrag darf nicht negativ sein".into())
        } else {
            Ok(Self(cents))
        }
    }

    pub fn from_euro_f64(euros: f64) -> Result<Self, String> {
        // Secure by Design: NaN/Infinity würden sonst still zu 0 bzw. i64::MAX gecastet
        if !euros.is_finite() {
            Err("Betrag muss eine endliche Zahl sein".into())
        } else if euros < 0.0 {
            Err("Betrag darf nicht negativ sein".into())
        } else {
            let cents = (euros * 100.0).round() as i64;
            Ok(Self(cents))
        }
    }

    pub fn as_cents(&self) -> i64 {
        self.0
    }

    pub fn as_f64_for_display(&self) -> f64 {
        self.0 as f64 / 100.0
    }

    pub fn add(&self, other: &Euro) -> Euro {
        Self(self.0 + other.0)
    }

    pub fn mul_rate(&self, quantity: f64) -> Euro {
        let cents = (self.0 as f64 * quantity).round() as i64;
        Self(cents)
    }

    pub fn vat_19_percent(&self) -> Euro {
        let vat_cents = (self.0 as f64 * 0.19).round() as i64;
        Self(vat_cents)
    }

    pub fn total_brutto(&self) -> Euro {
        self.add(&self.vat_19_percent())
    }
}

impl Serialize for Euro {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64(self.as_f64_for_display())
    }
}

impl<'de> Deserialize<'de> for Euro {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = f64::deserialize(deserializer)?;
        Euro::from_euro_f64(val).map_err(serde::de::Error::custom)
    }
}

impl Default for Euro {
    fn default() -> Self {
        Self(0)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Stunden(f64);

impl Stunden {
    pub fn try_new(val: f64) -> Result<Self, String> {
        if val < 0.0 {
            Err("Stunden dürfen nicht negativ sein".into())
        } else {
            Ok(Self(val))
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Default for Stunden {
    fn default() -> Self {
        Self(0.0)
    }
}

impl Serialize for Stunden {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_f64(self.0)
    }
}

impl<'de> Deserialize<'de> for Stunden {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let val = f64::deserialize(deserializer)?;
        Stunden::try_new(val).map_err(serde::de::Error::custom)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Kilometer(f64);

impl Kilometer {
    pub fn try_new(val: f64) -> Result<Self, String> {
        if val < 0.0 {
            Err("Kilometer dürfen nicht negativ sein".into())
        } else {
            Ok(Self(val))
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Default for Kilometer {
    fn default() -> Self {
        Self(0.0)
    }
}

impl Serialize for Kilometer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_f64(self.0)
    }
}

impl<'de> Deserialize<'de> for Kilometer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let val = f64::deserialize(deserializer)?;
        Kilometer::try_new(val).map_err(serde::de::Error::custom)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EinsatzTyp {
    Dienstleistung,
    Nachbereitung,
    KilometerFahrt,
}

#[allow(non_upper_case_globals)]
impl EinsatzTyp {
    pub const ArbeitVorOrt: Self = Self::Dienstleistung;
    pub const ArbeitVorbereitung: Self = Self::Nachbereitung;
}

impl Default for EinsatzTyp {
    fn default() -> Self {
        Self::Dienstleistung
    }
}

impl ToString for EinsatzTyp {
    fn to_string(&self) -> String {
        match self {
            Self::Dienstleistung => "DIENSTLEISTUNG".to_string(),
            Self::Nachbereitung => "NACHBEREITUNG".to_string(),
            Self::KilometerFahrt => "KILOMETER".to_string(),
        }
    }
}

impl EinsatzTyp {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_uppercase().as_str() {
            "DIENSTLEISTUNG" | "ARBEIT_VOR_ORT" | "ARBEIT" => Ok(Self::Dienstleistung),
            "NACHBEREITUNG" | "ARBEIT_VORBEREITUNG" | "VORBEREITUNG" => Ok(Self::Nachbereitung),
            "KILOMETER" | "FAHRT" | "KILOMETERFAHRT" => Ok(Self::KilometerFahrt),
            _ => Err("Ungültiger EinsatzTyp".into()),
        }
    }
}

impl Serialize for EinsatzTyp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for EinsatzTyp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        EinsatzTyp::from_str(&s).map_err(serde::de::Error::custom)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RechnungsNummer(String);

impl RechnungsNummer {
    pub fn try_new(val: String) -> Result<Self, String> {
        if val.len() == 7 && val.starts_with('R') && val[1..].chars().all(|c| c.is_ascii_digit()) {
            return Ok(Self(val));
        }
        if val.starts_with("RE-") || val.starts_with("R-") {
            return Ok(Self(val));
        }
        Err("Rechnungsnummer muss Format R + 6 Ziffern haben".into())
    }

    pub fn from_db(val: String) -> Self {
        Self(val)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Default for RechnungsNummer {
    fn default() -> Self {
        Self("R000000".to_string())
    }
}

impl Serialize for RechnungsNummer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for RechnungsNummer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let val = String::deserialize(deserializer)?;
        RechnungsNummer::try_new(val).map_err(serde::de::Error::custom)
    }
}


// =====================================================================
// Buchhaltung – Domain Primitives
//
// Secure by Design, Prinzip 3 (Domain Primitives): Jeder Typ validiert sich
// selbst im Konstruktor (Preconditions). Existiert ein Wert, ist er garantiert
// gültig (Postcondition). Alle Typen sind unveränderlich (Prinzip 5: Immutability):
// private Felder, keine Setter, nur lesende Zugriffe.
//
// Die Prüfungen folgen der Reihenfolge der Input Validation (Prinzip 6):
// Size → Lexical Content → Syntax. Semantische Prüfungen (z.B. "existiert der
// Auftrag?") benötigen die Datenbank und erfolgen daher erst im Handler.
// =====================================================================

/// Obergrenze für einen einzelnen Buchungsbetrag: 10 Mio. € (in Cent).
/// Deep Modeling: Ein Kleinbetrieb bucht keine Milliarden – ein absurder Betrag ist
/// ein Eingabefehler oder ein Angriff (z.B. Überlauf beim Aufsummieren).
pub const MAX_BUCHUNGSBETRAG_CENT: i64 = 1_000_000_000;

/// Transaktionstyp – nur zwei gültige Werte.
/// Als Rust-Enum ist Injection strukturell unmöglich: Es gibt keinen dritten Zustand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BuchungsTyp {
    Einnahme,
    Ausgabe,
}

impl BuchungsTyp {
    /// Precondition: exakt "einnahme" oder "ausgabe" (Whitelist, keine Normalisierung)
    pub fn parse(value: &str) -> Result<Self, AppError> {
        match value {
            "einnahme" => Ok(Self::Einnahme),
            "ausgabe" => Ok(Self::Ausgabe),
            _ => Err(AppError::BadRequest("Ungültiger Buchungstyp (erlaubt: einnahme, ausgabe)".into())),
        }
    }

    /// Repräsentation in der Datenbank (passt zum CHECK-Constraint der Migration)
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Einnahme => "einnahme",
            Self::Ausgabe => "ausgabe",
        }
    }
}

/// Betrag einer Buchung. Baut auf dem bestehenden `Euro(i64)` auf (Cent, KEIN float),
/// verschärft aber die Invariante: Eine Buchung über 0 € oder einen negativen Betrag
/// gibt es nicht (vgl. Fallstudie "-1 Buch kaufen"). Ob Geld hinein- oder hinausgeht,
/// drückt ausschließlich der `BuchungsTyp` aus – niemals das Vorzeichen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuchungsBetrag(Euro);

impl BuchungsBetrag {
    /// Precondition: 0 < Betrag <= MAX_BUCHUNGSBETRAG_CENT
    /// Postcondition: Self enthält einen positiven, plausiblen Betrag
    pub fn new(betrag: Euro) -> Result<Self, AppError> {
        if betrag.as_cents() <= 0 {
            return Err(AppError::BadRequest("Betrag muss größer als 0 € sein".into()));
        }
        if betrag.as_cents() > MAX_BUCHUNGSBETRAG_CENT {
            return Err(AppError::BadRequest("Betrag ist unplausibel hoch (max. 10.000.000 €)".into()));
        }
        Ok(Self(betrag))
    }

    pub fn from_cents(cents: i64) -> Result<Self, AppError> {
        Self::new(Euro::from_cents(cents).map_err(AppError::BadRequest)?)
    }

    /// Gibt eine Kopie zurück (Euro ist Copy) – keine Referenz auf den inneren Wert.
    pub fn euro(&self) -> Euro {
        self.0
    }

    pub fn as_cents(&self) -> i64 {
        self.0.as_cents()
    }
}

impl Serialize for BuchungsBetrag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        self.0.serialize(serializer)
    }
}

/// Buchungskategorie mit Whitelist und Längenbeschränkung.
/// Die Zeichen-Whitelist verhindert strukturell XSS/HTML-Injection: `<`, `>`, `"`, `'`
/// usw. können in einer Kategorie gar nicht vorkommen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Kategorie(String);

impl Kategorie {
    pub const MAX_LAENGE: usize = 100;

    /// Precondition: Nicht leer, max. 100 Zeichen, nur zulässige Zeichen
    /// (Buchstaben inkl. Umlaute, Ziffern, Leerzeichen und `- _ . , & / ( ) +`)
    /// Postcondition: Self enthält eine gültige Kategorie (getrimmt)
    pub fn new(value: &str) -> Result<Self, AppError> {
        let value = value.trim();
        // Size
        if value.is_empty() {
            return Err(AppError::BadRequest("Kategorie darf nicht leer sein".into()));
        }
        if value.chars().count() > Self::MAX_LAENGE {
            return Err(AppError::BadRequest(format!("Kategorie darf max. {} Zeichen lang sein", Self::MAX_LAENGE)));
        }
        // Lexical Content (Whitelist)
        if !value.chars().all(|c| c.is_alphanumeric() || matches!(c, ' ' | '-' | '_' | '.' | ',' | '&' | '/' | '(' | ')' | '+')) {
            return Err(AppError::BadRequest("Kategorie enthält unzulässige Zeichen".into()));
        }
        Ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Serialize for Kategorie {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_str(&self.0)
    }
}

/// Freitext-Beschreibung einer Buchung mit Längenbeschränkung.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Beschreibung(String);

impl Beschreibung {
    pub const MAX_LAENGE: usize = 500;

    /// Precondition: Max. 500 Zeichen, keine Steuerzeichen, keine HTML-Klammern `<` `>`
    /// Postcondition: Self enthält eine gültige (ggf. leere) Beschreibung (getrimmt)
    pub fn new(value: &str) -> Result<Self, AppError> {
        let value = value.trim();
        // Size
        if value.chars().count() > Self::MAX_LAENGE {
            return Err(AppError::BadRequest(format!("Beschreibung darf max. {} Zeichen lang sein", Self::MAX_LAENGE)));
        }
        // Lexical Content (Blacklist ist hier vertretbar, da Freitext; Ausgabe erfolgt
        // zusätzlich nur escaped via x-text – Defense in Depth)
        if value.chars().any(|c| c.is_control() || c == '<' || c == '>') {
            return Err(AppError::BadRequest("Beschreibung enthält unzulässige Zeichen".into()));
        }
        Ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Serialize for Beschreibung {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_str(&self.0)
    }
}

/// Belegnummer (z.B. Rechnungsnummer eines Lieferanten, Quittungsnummer).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BelegReferenz(String);

impl BelegReferenz {
    pub const MAX_LAENGE: usize = 100;

    /// Precondition: Nicht leer, max. 100 Zeichen, nur Buchstaben, Ziffern,
    /// Leerzeichen und `- _ / . #`
    /// Postcondition: Self enthält eine gültige Belegnummer (getrimmt)
    pub fn new(value: &str) -> Result<Self, AppError> {
        let value = value.trim();
        // Size
        if value.is_empty() {
            return Err(AppError::BadRequest("Belegnummer darf nicht leer sein".into()));
        }
        if value.chars().count() > Self::MAX_LAENGE {
            return Err(AppError::BadRequest(format!("Belegnummer darf max. {} Zeichen lang sein", Self::MAX_LAENGE)));
        }
        // Lexical Content (Whitelist)
        if !value.chars().all(|c| c.is_alphanumeric() || matches!(c, ' ' | '-' | '_' | '/' | '.' | '#')) {
            return Err(AppError::BadRequest("Belegnummer enthält unzulässige Zeichen".into()));
        }
        Ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Serialize for BelegReferenz {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_str(&self.0)
    }
}

/// Parst ein Datum im Format YYYY-MM-DD.
/// Reihenfolge der Input Validation: Size → Lexical → Syntax (Kalenderprüfung durch chrono,
/// d.h. "2026-02-30" wird abgelehnt).
pub fn parse_iso_datum(value: &str) -> Result<NaiveDate, AppError> {
    // Size + Lexical Content: exakt 10 Zeichen, nur Ziffern und Bindestriche
    if value.len() != 10 || !value.bytes().all(|b| b.is_ascii_digit() || b == b'-') {
        return Err(AppError::BadRequest("Datum muss im Format JJJJ-MM-TT angegeben werden".into()));
    }
    // Syntax
    NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map_err(|_| AppError::BadRequest("Ungültiges Datum".into()))
}

/// Buchungsdatum, validiertes Format, nicht in der Zukunft.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BuchungsDatum(NaiveDate);

impl BuchungsDatum {
    /// Precondition: Gültiges Datum im Format YYYY-MM-DD, nicht in der Zukunft
    /// Postcondition: Self enthält ein real existierendes, nicht zukünftiges Datum
    pub fn new(date_str: &str) -> Result<Self, AppError> {
        Self::new_mit_stichtag(date_str, chrono::Local::now().date_naive())
    }

    /// Wie `new`, aber mit explizitem Stichtag "heute" (für deterministische Tests).
    pub fn new_mit_stichtag(date_str: &str, heute: NaiveDate) -> Result<Self, AppError> {
        use chrono::Datelike;
        let datum = parse_iso_datum(date_str)?;
        if datum > heute {
            return Err(AppError::BadRequest("Buchungsdatum darf nicht in der Zukunft liegen".into()));
        }
        if datum.year() < 2000 {
            return Err(AppError::BadRequest("Buchungsdatum ist unplausibel (vor dem Jahr 2000)".into()));
        }
        Ok(Self(datum))
    }

    /// Rekonstruktion aus der Datenbank: Die fachliche Invariante wurde beim Schreiben
    /// geprüft, hier wird nur das Format verifiziert (ein gestern gültiges Datum darf
    /// nicht durch Zeitzonen-Effekte plötzlich unlesbar werden).
    pub fn from_db(date_str: &str) -> Result<Self, AppError> {
        parse_iso_datum(date_str).map(Self)
    }

    /// Gibt eine Kopie zurück (NaiveDate ist Copy).
    pub fn datum(&self) -> NaiveDate {
        self.0
    }

    pub fn to_iso(&self) -> String {
        self.0.format("%Y-%m-%d").to_string()
    }
}

impl Serialize for BuchungsDatum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_str(&self.to_iso())
    }
}


// =====================================================================
// Buchhaltung – Beleg-Dateien (Belegpflicht)
// =====================================================================

/// Maximale Größe eines Belegs: 10 MB (Input Validation Stufe 2: Size).
pub const MAX_BELEG_BYTES: usize = 10 * 1024 * 1024;

/// Erlaubte Beleg-Formate. Das Format wird aus dem **Inhalt** (Magic Bytes) bestimmt,
/// nie aus Dateiname oder Content-Type des Clients – beides ist frei fälschbar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BelegFormat {
    Pdf,
    Jpeg,
    Png,
}

impl BelegFormat {
    /// Syntax-Prüfung über die Dateisignatur
    pub fn erkennen(inhalt: &[u8]) -> Option<Self> {
        if inhalt.starts_with(b"%PDF-") {
            Some(Self::Pdf)
        } else if inhalt.starts_with(&[0xFF, 0xD8, 0xFF]) {
            Some(Self::Jpeg)
        } else if inhalt.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]) {
            Some(Self::Png)
        } else {
            None
        }
    }

    pub fn endung(&self) -> &'static str {
        match self {
            Self::Pdf => "pdf",
            Self::Jpeg => "jpg",
            Self::Png => "png",
        }
    }

    pub fn mime(&self) -> &'static str {
        match self {
            Self::Pdf => "application/pdf",
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
        }
    }

    pub fn aus_mime(mime: &str) -> Result<Self, AppError> {
        match mime {
            "application/pdf" => Ok(Self::Pdf),
            "image/jpeg" => Ok(Self::Jpeg),
            "image/png" => Ok(Self::Png),
            _ => Err(AppError::BadRequest("Unbekannter Beleg-Typ".into())),
        }
    }
}

/// Ursprünglicher Dateiname eines Belegs – nur zur Anzeige, nie Teil eines Pfads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BelegDateiname(String);

impl BelegDateiname {
    pub const MAX_LAENGE: usize = 200;

    /// Kanonisierung vor Validierung: Pfadanteile werden abgeschnitten, unzulässige
    /// Zeichen durch `_` ersetzt (ein Dateiname soll den Upload nicht verhindern,
    /// darf aber keine Steuer- oder HTML-Zeichen enthalten).
    /// Postcondition: 1–200 Zeichen, nur Buchstaben, Ziffern, Leerzeichen und `- _ . ( ) , + &`
    pub fn new(value: &str) -> Result<Self, AppError> {
        let basis = value.rsplit(['/', '\\']).next().unwrap_or("").trim();
        let bereinigt: String = basis
            .chars()
            .map(|c| if c.is_alphanumeric() || matches!(c, ' ' | '-' | '_' | '.' | '(' | ')' | ',' | '+' | '&') { c } else { '_' })
            .take(Self::MAX_LAENGE)
            .collect();
        let bereinigt = bereinigt.trim().to_string();
        if bereinigt.is_empty() || bereinigt.chars().all(|c| c == '.' || c == '_') {
            return Err(AppError::BadRequest("Ungültiger Dateiname des Belegs".into()));
        }
        Ok(Self(bereinigt))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Speicherpfad eines Belegs: immer `uploads/belege/<uuid>.<endung>`.
/// Der Pfad wird ausschließlich serverseitig erzeugt – Path Traversal ist strukturell
/// ausgeschlossen. Auch Pfade aus der Datenbank werden vor dem Löschen geprüft.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BelegPfad(String);

impl BelegPfad {
    pub const ORDNER: &'static str = "uploads/belege";

    /// Erzeugt einen neuen, zufälligen Pfad für das gegebene Format.
    pub fn neu(format: BelegFormat) -> Self {
        Self(format!("{}/{}.{}", Self::ORDNER, uuid::Uuid::new_v4(), format.endung()))
    }

    /// Precondition: exakt `uploads/belege/<uuid>.(pdf|jpg|png)`
    pub fn from_db(value: &str) -> Result<Self, AppError> {
        let ungueltig = || AppError::BadRequest("Ungültiger Beleg-Pfad".into());
        let datei = value.strip_prefix("uploads/belege/").ok_or_else(ungueltig)?;
        let (stamm, endung) = datei.rsplit_once('.').ok_or_else(ungueltig)?;
        if !matches!(endung, "pdf" | "jpg" | "png") || uuid::Uuid::parse_str(stamm).is_err() {
            return Err(ungueltig());
        }
        Ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// URL für den Abruf über die geschützte `/uploads`-Route
    pub fn url(&self) -> String {
        format!("/{}", self.0)
    }
}

/// Validierter Beleg-Upload. Existiert ein `BelegDatei`, ist der Inhalt garantiert
/// ein nicht-leeres PDF/JPG/PNG mit höchstens 10 MB.
#[derive(Debug, Clone)]
pub struct BelegDatei {
    format: BelegFormat,
    dateiname: BelegDateiname,
    inhalt: Vec<u8>,
}

impl BelegDatei {
    /// Reihenfolge: Size → Syntax (Magic Bytes) → Dateiname
    pub fn new(dateiname: &str, inhalt: Vec<u8>) -> Result<Self, AppError> {
        if inhalt.is_empty() {
            return Err(AppError::BadRequest("Der Beleg ist leer".into()));
        }
        if inhalt.len() > MAX_BELEG_BYTES {
            return Err(AppError::BadRequest("Beleg ist zu groß (max. 10 MB)".into()));
        }
        let format = BelegFormat::erkennen(&inhalt)
            .ok_or_else(|| AppError::BadRequest("Beleg muss ein PDF, JPG oder PNG sein".into()))?;
        let dateiname = BelegDateiname::new(dateiname)?;
        Ok(Self { format, dateiname, inhalt })
    }

    pub fn format(&self) -> BelegFormat {
        self.format
    }

    pub fn dateiname(&self) -> &BelegDateiname {
        &self.dateiname
    }

    pub fn inhalt(&self) -> &[u8] {
        &self.inhalt
    }
}


// =====================================================================
// Buchhaltung – Geschäftsjahr
// =====================================================================

/// Geschäftsjahr eines EÜR-Betriebs.
///
/// Deep Modeling: Das Geschäftsjahr ist das **Kalenderjahr** (§ 4a EStG). Diese Regel
/// steht ausschließlich in diesem Typ (`aus_datum` und `zeitraum`) – kein anderer Code
/// rechnet Jahresgrenzen selbst aus. Ein abweichendes Wirtschaftsjahr würde nur hier
/// geändert. Das Geschäftsjahr wird nie gespeichert, sondern immer aus dem
/// Buchungsdatum (= Zahlungsdatum, Zuflussprinzip) abgeleitet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Geschaeftsjahr(i32);

impl Geschaeftsjahr {
    /// Frühestes plausibles Geschäftsjahr (passt zur Untergrenze von `BuchungsDatum`).
    pub const ERSTES_JAHR: i32 = 2000;

    /// Precondition: ERSTES_JAHR <= jahr <= aktuelles Jahr
    /// Postcondition: Self ist ein Jahr, in dem Buchungen existieren können
    pub fn new(jahr: i32) -> Result<Self, AppError> {
        Self::new_mit_stichtag(jahr, chrono::Local::now().date_naive())
    }

    /// Wie `new`, aber mit explizitem Stichtag "heute" (für deterministische Tests).
    pub fn new_mit_stichtag(jahr: i32, heute: NaiveDate) -> Result<Self, AppError> {
        use chrono::Datelike;
        if jahr < Self::ERSTES_JAHR {
            return Err(AppError::BadRequest(format!("Geschäftsjahr muss ab {} liegen", Self::ERSTES_JAHR)));
        }
        if jahr > heute.year() {
            return Err(AppError::BadRequest("Geschäftsjahr darf nicht in der Zukunft liegen".into()));
        }
        Ok(Self(jahr))
    }

    /// Parst eine Jahresangabe aus einem Query-Parameter.
    /// Reihenfolge: Size + Lexical (exakt 4 Ziffern) → Syntax → Semantic (`new`).
    pub fn parse(value: &str) -> Result<Self, AppError> {
        let value = value.trim();
        if value.len() != 4 || !value.bytes().all(|b| b.is_ascii_digit()) {
            return Err(AppError::BadRequest("Jahr muss vierstellig angegeben werden (JJJJ)".into()));
        }
        let jahr: i32 = value.parse().map_err(|_| AppError::BadRequest("Ungültiges Jahr".into()))?;
        Self::new(jahr)
    }

    /// Geschäftsjahr, in das ein Datum fällt (Kalenderjahr-Regel).
    /// Liefert einen Fehler für Daten außerhalb des gültigen Bereichs, damit die
    /// Invariante (ab 2000, nicht in der Zukunft) auch hier gilt.
    pub fn aus_datum(datum: NaiveDate) -> Result<Self, AppError> {
        use chrono::Datelike;
        Self::new(datum.year())
    }

    /// Das laufende Geschäftsjahr (immer gültig).
    pub fn aktuelles() -> Self {
        use chrono::Datelike;
        Self(chrono::Local::now().date_naive().year())
    }

    /// Erster und letzter Tag des Geschäftsjahres (beide inklusive): 01.01. bis 31.12.
    pub fn zeitraum(&self) -> (NaiveDate, NaiveDate) {
        let von = NaiveDate::from_ymd_opt(self.0, 1, 1).expect("01.01. existiert in jedem Jahr");
        let bis = NaiveDate::from_ymd_opt(self.0, 12, 31).expect("31.12. existiert in jedem Jahr");
        (von, bis)
    }

    /// Das Vorjahr – `None`, wenn es vor ERSTES_JAHR läge.
    pub fn vorjahr(&self) -> Option<Self> {
        (self.0 > Self::ERSTES_JAHR).then(|| Self(self.0 - 1))
    }

    pub fn jahr(&self) -> i32 {
        self.0
    }
}

impl Serialize for Geschaeftsjahr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_i32(self.0)
    }
}
