use wendepunkt_software::domain::{Euro, Stunden, Kilometer, EinsatzTyp, RechnungsNummer};

#[test]
fn test_euro_invariants() {
    assert!(Euro::from_cents(100).is_ok());
    assert!(Euro::from_cents(-1).is_err());
    
    assert!(Euro::from_euro_f64(1.0).is_ok());
    assert!(Euro::from_euro_f64(-0.1).is_err());
    
    let e1 = Euro::from_cents(100).unwrap(); // 1.00 Euro
    let e2 = Euro::from_cents(200).unwrap(); // 2.00 Euro
    assert_eq!(e1.add(&e2).as_cents(), 300);
    
    let vat = e1.vat_19_percent();
    assert_eq!(vat.as_cents(), 19);
}

#[test]
fn test_stunden_invariants() {
    assert!(Stunden::try_new(1.0).is_ok());
    assert!(Stunden::try_new(-1.0).is_err());
}

#[test]
fn test_kilometer_invariants() {
    assert!(Kilometer::try_new(1.0).is_ok());
    assert!(Kilometer::try_new(-1.0).is_err());
}

#[test]
fn test_einsatz_typ_parsing() {
    assert_eq!(EinsatzTyp::from_str("ARBEIT_VOR_ORT").unwrap(), EinsatzTyp::ArbeitVorOrt);
    assert_eq!(EinsatzTyp::from_str("ARBEIT").unwrap(), EinsatzTyp::ArbeitVorOrt);
    assert_eq!(EinsatzTyp::from_str("KILOMETER").unwrap(), EinsatzTyp::KilometerFahrt);
    assert!(EinsatzTyp::from_str("INVALID").is_err());
}

#[test]
fn test_rechnungs_nummer_invariants() {
    assert!(RechnungsNummer::try_new("R123456".to_string()).is_ok());
    assert!(RechnungsNummer::try_new("A123456".to_string()).is_err()); // Wrong prefix
    assert!(RechnungsNummer::try_new("R12345".to_string()).is_err()); // Too short
    assert!(RechnungsNummer::try_new("R1234567".to_string()).is_err()); // Too long
    assert!(RechnungsNummer::try_new("R12345a".to_string()).is_err()); // Non-digit
}
