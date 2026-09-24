use serde::{Serialize, Deserialize, Serializer, Deserializer};
use std::fmt;

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
        if euros < 0.0 {
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


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EinsatzTyp {
    ArbeitVorOrt,
    ArbeitVorbereitung,
    KilometerFahrt,
}

impl Default for EinsatzTyp {
    fn default() -> Self {
        Self::ArbeitVorOrt
    }
}

impl ToString for EinsatzTyp {
    fn to_string(&self) -> String {
        match self {
            Self::ArbeitVorOrt => "ARBEIT_VOR_ORT".to_string(),
            Self::ArbeitVorbereitung => "ARBEIT_VORBEREITUNG".to_string(),
            Self::KilometerFahrt => "KILOMETER".to_string(),
        }
    }
}

impl EinsatzTyp {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_uppercase().as_str() {
            "ARBEIT_VOR_ORT" | "ARBEIT" => Ok(Self::ArbeitVorOrt),
            "ARBEIT_VORBEREITUNG" => Ok(Self::ArbeitVorbereitung),
            "KILOMETER" | "FAHRT" => Ok(Self::KilometerFahrt),
            _ => Err("Ungültiger EinsatzTyp".into()),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RechnungsNummer(String);

impl RechnungsNummer {
    pub fn try_new(val: String) -> Result<Self, String> {
        if val.len() != 7 || !val.starts_with('R') {
            return Err("Rechnungsnummer muss Format R + 6 Ziffern haben".into());
        }
        let digits = &val[1..];
        if digits.chars().all(|c| c.is_ascii_digit()) {
            Ok(Self(val))
        } else {
            Err("Rechnungsnummer muss Format R + 6 Ziffern haben".into())
        }
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
