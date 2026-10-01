use chrono::NaiveDate;
use sqlx::SqlitePool;
use wendepunkt_software::database;
use wendepunkt_software::domain::{BelegReferenz, Beschreibung, BuchungsBetrag, BuchungsDatum, BuchungsTyp, Euro, Kategorie};
use wendepunkt_software::models::{Auftrag, BuchungEingabe, BuchungsFilterParameter, Kunde, Zeitraum};

async fn setup_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}

fn eingabe(typ: &str, betrag: f64, datum: &str) -> BuchungEingabe {
    BuchungEingabe {
        buchungs_typ: typ.into(),
        betrag,
        kategorie: "Material".into(),
        beschreibung: "Testbuchung".into(),
        datum: datum.into(),
        auftrag_id: None,
        beleg_referenz: None,
    }
}

// --- Domain Primitives ---

#[test]
fn test_buchungs_typ_whitelist() {
    assert_eq!(BuchungsTyp::parse("einnahme").unwrap(), BuchungsTyp::Einnahme);
    assert_eq!(BuchungsTyp::parse("ausgabe").unwrap(), BuchungsTyp::Ausgabe);
    assert!(BuchungsTyp::parse("Einnahme").is_err());
    assert!(BuchungsTyp::parse("einnahme' OR 1=1 --").is_err());
    assert!(BuchungsTyp::parse("").is_err());
    assert_eq!(serde_json::to_string(&BuchungsTyp::Ausgabe).unwrap(), "\"ausgabe\"");
}

#[test]
fn test_buchungs_betrag_nur_positiv_und_plausibel() {
    assert!(BuchungsBetrag::from_cents(1).is_ok());
    assert!(BuchungsBetrag::from_cents(0).is_err());
    assert!(BuchungsBetrag::from_cents(-100).is_err());
    assert!(BuchungsBetrag::from_cents(1_000_000_000).is_ok());
    assert!(BuchungsBetrag::from_cents(1_000_000_001).is_err());
    // Euro-Konvertierung: 12,50 € → 1250 Cent, 0,29 € darf nicht zu 28 Cent werden
    assert_eq!(BuchungsBetrag::new(Euro::from_euro_f64(12.5).unwrap()).unwrap().as_cents(), 1250);
    assert_eq!(Euro::from_euro_f64(0.29).unwrap().as_cents(), 29);
    assert!(Euro::from_euro_f64(f64::NAN).is_err());
    assert!(Euro::from_euro_f64(f64::INFINITY).is_err());
}

#[test]
fn test_kategorie_validierung() {
    assert_eq!(Kategorie::new("  Büro & Material ").unwrap().as_str(), "Büro & Material");
    assert!(Kategorie::new("Fahrtkosten (PKW)").is_ok());
    assert!(Kategorie::new("").is_err());
    assert!(Kategorie::new("   ").is_err());
    assert!(Kategorie::new(&"a".repeat(100)).is_ok());
    assert!(Kategorie::new(&"a".repeat(101)).is_err());
    assert!(Kategorie::new("<script>alert(1)</script>").is_err());
    assert!(Kategorie::new("Material\"; DROP TABLE buchungen; --").is_err());
}

#[test]
fn test_beschreibung_validierung() {
    assert!(Beschreibung::new("").is_ok());
    assert!(Beschreibung::new("Müllsäcke, 2x à 5 €").is_ok());
    assert!(Beschreibung::new(&"ä".repeat(500)).is_ok());
    assert!(Beschreibung::new(&"ä".repeat(501)).is_err());
    assert!(Beschreibung::new("<img src=x onerror=alert(1)>").is_err());
    assert!(Beschreibung::new("Zeile1\nZeile2").is_err());
}

#[test]
fn test_beleg_referenz_validierung() {
    assert!(BelegReferenz::new("RE-2026/0042").is_ok());
    assert!(BelegReferenz::new("").is_err());
    assert!(BelegReferenz::new(&"1".repeat(101)).is_err());
    assert!(BelegReferenz::new("RE<1>").is_err());
}

#[test]
fn test_buchungs_datum_validierung() {
    let heute = NaiveDate::from_ymd_opt(2026, 10, 1).unwrap();
    assert!(BuchungsDatum::new_mit_stichtag("2026-10-01", heute).is_ok());
    assert!(BuchungsDatum::new_mit_stichtag("2026-10-02", heute).is_err()); // Zukunft
    assert!(BuchungsDatum::new_mit_stichtag("2026-02-30", heute).is_err()); // existiert nicht
    assert!(BuchungsDatum::new_mit_stichtag("01.10.2026", heute).is_err()); // falsches Format
    assert!(BuchungsDatum::new_mit_stichtag("2026-1-01", heute).is_err());
    assert!(BuchungsDatum::new_mit_stichtag("1999-12-31", heute).is_err()); // unplausibel
    assert_eq!(BuchungsDatum::new_mit_stichtag("2026-09-15", heute).unwrap().to_iso(), "2026-09-15");
}

#[test]
fn test_eingabe_lehnt_unbekannte_felder_ab() {
    // Mass Assignment: created_by darf nicht vom Client kommen
    let json = r#"{"buchungs_typ":"einnahme","betrag":10.0,"kategorie":"X","datum":"2026-01-01","created_by":999}"#;
    assert!(serde_json::from_str::<BuchungEingabe>(json).is_err());
}

#[test]
fn test_filter_validierung() {
    assert!(Zeitraum::new(Some("2026-01-01"), Some("2026-12-31")).is_ok());
    assert!(Zeitraum::new(Some("2026-12-31"), Some("2026-01-01")).is_err());
    assert!(Zeitraum::new(Some(""), None).unwrap().von.is_none());
    let f = BuchungsFilterParameter { typ: Some("".into()), ..Default::default() }.validieren().unwrap();
    assert!(f.typ.is_none());
    assert!(BuchungsFilterParameter { typ: Some("alles".into()), ..Default::default() }.validieren().is_err());
}

// --- Datenbank ---

#[tokio::test]
async fn test_buchungen_crud_und_uebersicht() {
    let pool = setup_db().await;
    let user = database::get_user_by_username(&pool, "admin").await.unwrap();

    let e1 = eingabe("einnahme", 1234.56, "2026-03-10").validieren().unwrap();
    let e2 = eingabe("ausgabe", 200.0, "2026-04-01").validieren().unwrap();
    let e3 = eingabe("ausgabe", 0.44, "2025-12-31").validieren().unwrap();

    let b1 = database::create_buchung(&pool, &e1, user.id).await.unwrap();
    let b2 = database::create_buchung(&pool, &e2, user.id).await.unwrap();
    database::create_buchung(&pool, &e3, user.id).await.unwrap();

    assert_eq!(b1.betrag().as_cents(), 123456);
    assert_eq!(b1.created_by(), user.id);
    assert!(!b1.created_at().is_empty());

    // Liste: neueste zuerst
    let alle = database::get_buchungen(&pool, &BuchungsFilterParameter::default().validieren().unwrap()).await.unwrap();
    assert_eq!(alle.len(), 3);
    assert_eq!(alle[0].datum().to_iso(), "2026-04-01");
    assert_eq!(alle[2].datum().to_iso(), "2025-12-31");

    // Filter: Typ + Zeitraum + Kategorie (case-insensitive)
    let f = BuchungsFilterParameter { typ: Some("ausgabe".into()), von: Some("2026-01-01".into()), ..Default::default() };
    let gefiltert = database::get_buchungen(&pool, &f.validieren().unwrap()).await.unwrap();
    assert_eq!(gefiltert, vec![b2.clone()]);
    let f = BuchungsFilterParameter { kategorie: Some("material".into()), ..Default::default() };
    assert_eq!(database::get_buchungen(&pool, &f.validieren().unwrap()).await.unwrap().len(), 3);

    // Übersicht gesamt: 1234,56 - 200,00 - 0,44 = 1034,12
    let u = database::get_buchungen_uebersicht(&pool, None, None).await.unwrap();
    assert_eq!(u.einnahmen_gesamt.as_cents(), 123456);
    assert_eq!(u.ausgaben_gesamt.as_cents(), 20044);
    assert_eq!(u.saldo, 103412);

    // Übersicht mit Zeitraum (nur 2026)
    let z = Zeitraum::new(Some("2026-01-01"), Some("2026-12-31")).unwrap();
    let u = database::get_buchungen_uebersicht(&pool, z.von, z.bis).await.unwrap();
    assert_eq!(u.ausgaben_gesamt.as_cents(), 20000);
    assert_eq!(u.saldo, 103456);

    // Negativer Saldo
    let gross = eingabe("ausgabe", 5000.0, "2026-05-01").validieren().unwrap();
    database::create_buchung(&pool, &gross, user.id).await.unwrap();
    assert!(database::get_buchungen_uebersicht(&pool, None, None).await.unwrap().saldo < 0);

    // Update ändert Daten, aber nicht created_by / Identität
    let neu = eingabe("einnahme", 99.99, "2026-04-02").validieren().unwrap();
    let b2_neu = database::update_buchung(&pool, b2.id(), &neu).await.unwrap();
    assert_eq!(b2_neu, b2); // Entity-Gleichheit über ID
    assert_eq!(b2_neu.betrag().as_cents(), 9999);
    assert_eq!(b2_neu.buchungs_typ(), BuchungsTyp::Einnahme);
    assert_eq!(b2_neu.created_by(), user.id);

    // Delete
    database::delete_buchung(&pool, b2.id()).await.unwrap();
    assert!(database::get_buchung_by_id(&pool, b2.id()).await.unwrap().is_none());
    assert!(matches!(database::delete_buchung(&pool, b2.id()).await, Err(sqlx::Error::RowNotFound)));
    assert!(matches!(database::update_buchung(&pool, b2.id(), &neu).await, Err(sqlx::Error::RowNotFound)));
}

#[tokio::test]
async fn test_buchung_auftrag_verknuepfung() {
    let pool = setup_db().await;
    let user = database::get_user_by_username(&pool, "admin").await.unwrap();
    let kunde_id = database::create_kunde(&pool, Kunde { vorname: "Test".into(), nachname: "Kunde".into(), ..Default::default() }).await.unwrap();
    let auftrag_id = database::create_auftrag(&pool, Auftrag { kunde_id, ..Default::default() }).await.unwrap();

    assert!(database::auftrag_existiert(&pool, auftrag_id).await.unwrap());
    assert!(!database::auftrag_existiert(&pool, auftrag_id + 1000).await.unwrap());

    let mut e = eingabe("einnahme", 100.0, "2026-01-15");
    e.auftrag_id = Some(auftrag_id);
    e.beleg_referenz = Some("R000001".into());
    let b = database::create_buchung(&pool, &e.validieren().unwrap(), user.id).await.unwrap();
    assert_eq!(b.auftrag_id(), Some(auftrag_id));
    assert_eq!(b.beleg_referenz().unwrap().as_str(), "R000001");

    // Auftrag löschen → Buchung bleibt erhalten, Verknüpfung wird NULL (ON DELETE SET NULL)
    database::delete_auftrag(&pool, auftrag_id).await.unwrap();
    let b = database::get_buchung_by_id(&pool, b.id()).await.unwrap().unwrap();
    assert_eq!(b.auftrag_id(), None);
}

#[tokio::test]
async fn test_db_check_constraints_als_zweite_schicht() {
    // Defense in Depth: Selbst an den Domain Primitives vorbei lehnt die DB ungültige Daten ab
    let pool = setup_db().await;
    let user = database::get_user_by_username(&pool, "admin").await.unwrap();
    let insert = |typ: &'static str, cent: i64, datum: &'static str| {
        sqlx::query("INSERT INTO buchungen (buchungs_typ, betrag_cent, kategorie, beschreibung, datum, created_by) VALUES (?, ?, 'X', '', ?, ?)")
            .bind(typ).bind(cent).bind(datum).bind(user.id)
    };
    assert!(insert("einnahme", 100, "2026-01-01").execute(&pool).await.is_ok());
    assert!(insert("gutschrift", 100, "2026-01-01").execute(&pool).await.is_err());
    assert!(insert("einnahme", -1, "2026-01-01").execute(&pool).await.is_err());
    assert!(insert("einnahme", 0, "2026-01-01").execute(&pool).await.is_err());
    assert!(insert("einnahme", 100, "2026-02-30").execute(&pool).await.is_err());
    assert!(insert("einnahme", 100, "01.01.2026").execute(&pool).await.is_err());
}
