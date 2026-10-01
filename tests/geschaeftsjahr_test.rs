use chrono::{Datelike, NaiveDate};
use wendepunkt_software::domain::Geschaeftsjahr;

fn d(y: i32, m: u32, t: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, t).unwrap()
}

#[test]
fn test_zeitraum_ist_kalenderjahr() {
    let j = Geschaeftsjahr::new_mit_stichtag(2025, d(2026, 10, 1)).unwrap();
    assert_eq!(j.zeitraum(), (d(2025, 1, 1), d(2025, 12, 31)));
}

#[test]
fn test_jahresgrenzen_aus_datum() {
    // 01.01. und 31.12. gehören zum selben Geschäftsjahr, der 31.12. nicht zum Folgejahr
    assert_eq!(Geschaeftsjahr::aus_datum(d(2025, 1, 1)).unwrap().jahr(), 2025);
    assert_eq!(Geschaeftsjahr::aus_datum(d(2025, 12, 31)).unwrap().jahr(), 2025);
    assert_eq!(Geschaeftsjahr::aus_datum(d(2026, 1, 1)).unwrap().jahr(), 2026);
}

#[test]
fn test_schaltjahr() {
    let j = Geschaeftsjahr::new_mit_stichtag(2024, d(2026, 10, 1)).unwrap();
    let (von, bis) = j.zeitraum();
    assert_eq!((bis - von).num_days() + 1, 366);
    assert_eq!(Geschaeftsjahr::aus_datum(d(2024, 2, 29)).unwrap().jahr(), 2024);
    let j = Geschaeftsjahr::new_mit_stichtag(2025, d(2026, 10, 1)).unwrap();
    let (von, bis) = j.zeitraum();
    assert_eq!((bis - von).num_days() + 1, 365);
}

#[test]
fn test_ungueltige_jahre() {
    let heute = d(2026, 10, 1);
    assert!(Geschaeftsjahr::new_mit_stichtag(1999, heute).is_err());
    assert!(Geschaeftsjahr::new_mit_stichtag(2000, heute).is_ok());
    assert!(Geschaeftsjahr::new_mit_stichtag(2026, heute).is_ok());
    assert!(Geschaeftsjahr::new_mit_stichtag(2027, heute).is_err()); // Zukunft
    assert!(Geschaeftsjahr::aus_datum(d(1999, 12, 31)).is_err());
}

#[test]
fn test_parse() {
    assert_eq!(Geschaeftsjahr::parse("2025").unwrap().jahr(), 2025);
    assert_eq!(Geschaeftsjahr::parse(" 2025 ").unwrap().jahr(), 2025);
    assert!(Geschaeftsjahr::parse("25").is_err());
    assert!(Geschaeftsjahr::parse("20255").is_err());
    assert!(Geschaeftsjahr::parse("2O25").is_err());
    assert!(Geschaeftsjahr::parse("-202").is_err());
    assert!(Geschaeftsjahr::parse("1999").is_err());
    assert!(Geschaeftsjahr::parse("9999").is_err());
}

#[test]
fn test_aktuelles_und_vorjahr() {
    let aktuell = Geschaeftsjahr::aktuelles();
    assert_eq!(aktuell.jahr(), chrono::Local::now().year());
    assert_eq!(aktuell.vorjahr().unwrap().jahr(), aktuell.jahr() - 1);
    assert!(Geschaeftsjahr::new_mit_stichtag(2000, d(2026, 1, 1)).unwrap().vorjahr().is_none());
    assert_eq!(serde_json::to_string(&aktuell).unwrap(), aktuell.jahr().to_string());
}
