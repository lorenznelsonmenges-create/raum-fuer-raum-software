use chrono::NaiveDate;
use sqlx::SqlitePool;
use wendepunkt_software::database;
use wendepunkt_software::domain::{BelegReferenz, Beschreibung, BuchungsBetrag, BuchungsDatum, BuchungsTyp, Euro, Kategorie};
use wendepunkt_software::domain::{BelegDatei, BelegDateiname, BelegFormat, BelegPfad, MAX_BELEG_BYTES};
use wendepunkt_software::models::{Auftrag, BelegMeta, BuchungEingabe, BuchungsFilterParameter, GeschaeftsjahrInfo, Kunde, Zeitraum, ZeitraumParameter};
use wendepunkt_software::domain::Geschaeftsjahr;

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

fn beleg() -> BelegMeta {
    BelegMeta {
        pfad: BelegPfad::neu(BelegFormat::Pdf),
        dateiname: BelegDateiname::new("quittung.pdf").unwrap(),
        format: BelegFormat::Pdf,
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

    let b1 = database::create_buchung(&pool, &e1, &beleg(), user.id).await.unwrap();
    let b2 = database::create_buchung(&pool, &e2, &beleg(), user.id).await.unwrap();
    database::create_buchung(&pool, &e3, &beleg(), user.id).await.unwrap();

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
    database::create_buchung(&pool, &gross, &beleg(), user.id).await.unwrap();
    assert!(database::get_buchungen_uebersicht(&pool, None, None).await.unwrap().saldo < 0);

    // Update ändert Daten, aber nicht created_by / Identität
    let neu = eingabe("einnahme", 99.99, "2026-04-02").validieren().unwrap();
    let b2_neu = database::update_buchung(&pool, b2.id(), &neu, None).await.unwrap();
    assert_eq!(b2_neu, b2); // Entity-Gleichheit über ID
    assert_eq!(b2_neu.betrag().as_cents(), 9999);
    assert_eq!(b2_neu.buchungs_typ(), BuchungsTyp::Einnahme);
    assert_eq!(b2_neu.created_by(), user.id);

    // Delete
    database::delete_buchung(&pool, b2.id()).await.unwrap();
    assert!(database::get_buchung_by_id(&pool, b2.id()).await.unwrap().is_none());
    assert!(matches!(database::delete_buchung(&pool, b2.id()).await, Err(sqlx::Error::RowNotFound)));
    assert!(matches!(database::update_buchung(&pool, b2.id(), &neu, None).await, Err(sqlx::Error::RowNotFound)));
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
    let b = database::create_buchung(&pool, &e.validieren().unwrap(), &beleg(), user.id).await.unwrap();
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

// --- Belege ---

#[test]
fn test_beleg_format_ueber_magic_bytes() {
    assert_eq!(BelegFormat::erkennen(b"%PDF-1.7 ..."), Some(BelegFormat::Pdf));
    assert_eq!(BelegFormat::erkennen(&[0xFF, 0xD8, 0xFF, 0xE0]), Some(BelegFormat::Jpeg));
    assert_eq!(BelegFormat::erkennen(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, 0]), Some(BelegFormat::Png));
    assert_eq!(BelegFormat::erkennen(b"<html><script>"), None);
    assert_eq!(BelegFormat::erkennen(b"MZ\x90\x00"), None); // Windows-EXE
}

#[test]
fn test_beleg_datei_validierung() {
    assert!(BelegDatei::new("rechnung.pdf", b"%PDF-1.4 inhalt".to_vec()).is_ok());
    assert!(BelegDatei::new("leer.pdf", vec![]).is_err());
    // Endung .pdf hilft nicht: der Inhalt entscheidet
    assert!(BelegDatei::new("boese.pdf", b"<svg onload=alert(1)>".to_vec()).is_err());
    let mut zu_gross = b"%PDF-".to_vec();
    zu_gross.resize(MAX_BELEG_BYTES + 1, b'a');
    assert!(BelegDatei::new("gross.pdf", zu_gross).is_err());
}

#[test]
fn test_beleg_dateiname_wird_bereinigt() {
    assert_eq!(BelegDateiname::new("../../etc/passwd").unwrap().as_str(), "passwd");
    assert_eq!(BelegDateiname::new("C:\\Users\\x\\Quittung März.pdf").unwrap().as_str(), "Quittung März.pdf");
    assert_eq!(BelegDateiname::new("<script>.pdf").unwrap().as_str(), "_script_.pdf");
    assert!(BelegDateiname::new("../").is_err());
    assert_eq!(BelegDateiname::new(&"a".repeat(300)).unwrap().as_str().chars().count(), 200);
}

#[test]
fn test_beleg_pfad_nur_im_belegordner() {
    let p = BelegPfad::neu(BelegFormat::Png);
    assert!(p.as_str().starts_with("uploads/belege/") && p.as_str().ends_with(".png"));
    assert!(BelegPfad::from_db(p.as_str()).is_ok());
    assert!(BelegPfad::from_db("uploads/belege/../../achtsam.db").is_err());
    assert!(BelegPfad::from_db("uploads/rechnungen/R000001.pdf").is_err());
    assert!(BelegPfad::from_db("uploads/belege/kein-uuid.pdf").is_err());
    assert!(BelegPfad::from_db("uploads/belege/0b5e6f2a-6c1d-4d0e-9a51-1f2a3b4c5d6e.exe").is_err());
}

#[tokio::test]
async fn test_buchung_beleg_speichern_und_ersetzen() {
    let pool = setup_db().await;
    let user = database::get_user_by_username(&pool, "admin").await.unwrap();
    let daten = eingabe("ausgabe", 12.5, "2026-09-01").validieren().unwrap();
    let erster = beleg();
    let b = database::create_buchung(&pool, &daten, &erster, user.id).await.unwrap();
    assert_eq!(b.beleg(), Some(&erster));

    // Update ohne neuen Beleg behält den alten
    let b = database::update_buchung(&pool, b.id(), &daten, None).await.unwrap();
    assert_eq!(b.beleg(), Some(&erster));

    // Update mit neuem Beleg ersetzt ihn
    let zweiter = BelegMeta { pfad: BelegPfad::neu(BelegFormat::Jpeg), dateiname: BelegDateiname::new("foto.jpg").unwrap(), format: BelegFormat::Jpeg };
    let b = database::update_buchung(&pool, b.id(), &daten, Some(&zweiter)).await.unwrap();
    assert_eq!(b.beleg(), Some(&zweiter));

    // JSON enthält Abruf-URL, aber kein internes Pfad-Feld
    let json = serde_json::to_value(&b).unwrap();
    assert_eq!(json["beleg"]["typ"], "image/jpeg");
    assert_eq!(json["beleg"]["url"], format!("/{}", zweiter.pfad.as_str()));
    assert!(json["beleg"].get("pfad").is_none());

    // DB lehnt Belegpfade außerhalb von uploads/belege ab
    let r = sqlx::query("UPDATE buchungen SET beleg_pfad = 'uploads/../achtsam.db' WHERE id = ?").bind(b.id()).execute(&pool).await;
    assert!(r.is_err());
}

// --- Geschäftsjahr-Filter ---

#[test]
fn test_jahr_parameter_validierung() {
    // ?jahr= wird über Geschaeftsjahr in von/bis übersetzt
    let z = ZeitraumParameter { jahr: Some("2025".into()), ..Default::default() }.validieren().unwrap();
    assert_eq!(z.von, NaiveDate::from_ymd_opt(2025, 1, 1));
    assert_eq!(z.bis, NaiveDate::from_ymd_opt(2025, 12, 31));
    // jahr + von/bis → 400
    assert!(ZeitraumParameter { jahr: Some("2025".into()), von: Some("2025-01-01".into()), ..Default::default() }.validieren().is_err());
    assert!(BuchungsFilterParameter { jahr: Some("2025".into()), bis: Some("2025-06-30".into()), ..Default::default() }.validieren().is_err());
    // leere Strings zählen als "nicht gesetzt" (Frontend schickt leere Felder nicht, aber kompatibel)
    assert!(ZeitraumParameter { jahr: Some("2025".into()), von: Some("".into()), ..Default::default() }.validieren().is_ok());
    // ungültige Jahre
    assert!(ZeitraumParameter { jahr: Some("1999".into()), ..Default::default() }.validieren().is_err());
    assert!(ZeitraumParameter { jahr: Some("abcd".into()), ..Default::default() }.validieren().is_err());
    // von/bis ohne jahr bleibt kompatibel
    assert!(ZeitraumParameter { von: Some("2025-03-01".into()), bis: Some("2025-03-31".into()), ..Default::default() }.validieren().is_ok());
    // jahr lässt sich nicht mit unbekannten Feldern kombinieren
    assert!(serde_json::from_str::<ZeitraumParameter>(r#"{"jahr":"2025","x":"1"}"#).is_err());
}

#[tokio::test]
async fn test_jahr_filter_uebersicht_und_ohne_beleg() {
    let pool = setup_db().await;
    let user = database::get_user_by_username(&pool, "admin").await.unwrap();
    // Grenzfälle: 31.12.2024, 01.01.2025, 31.12.2025, 01.01.2026
    for (typ, betrag, datum) in [("einnahme", 100.0, "2024-12-31"), ("einnahme", 200.0, "2025-01-01"),
                                 ("ausgabe", 50.0, "2025-12-31"), ("einnahme", 400.0, "2026-01-01")] {
        database::create_buchung(&pool, &eingabe(typ, betrag, datum).validieren().unwrap(), &beleg(), user.id).await.unwrap();
    }
    // Altbuchung ohne Beleg in 2025 (an der Anwendung vorbei, wie vor der Belegpflicht)
    sqlx::query("INSERT INTO buchungen (buchungs_typ, betrag_cent, kategorie, beschreibung, datum, created_by) VALUES ('ausgabe', 1000, 'Alt', '', '2025-06-15', ?)")
        .bind(user.id).execute(&pool).await.unwrap();

    let z = ZeitraumParameter { jahr: Some("2025".into()), ..Default::default() }.validieren().unwrap();
    let u = database::get_buchungen_uebersicht(&pool, z.von, z.bis).await.unwrap();
    assert_eq!(u.einnahmen_gesamt.as_cents(), 20000);
    assert_eq!(u.ausgaben_gesamt.as_cents(), 5000 + 1000);
    assert_eq!(u.saldo, 20000 - 6000);
    assert_eq!(u.anzahl_ohne_beleg, 1);

    let z = ZeitraumParameter { jahr: Some("2024".into()), ..Default::default() }.validieren().unwrap();
    let u = database::get_buchungen_uebersicht(&pool, z.von, z.bis).await.unwrap();
    assert_eq!(u.einnahmen_gesamt.as_cents(), 10000);
    assert_eq!(u.anzahl_ohne_beleg, 0);

    // Gesamt ohne Zeitraum
    assert_eq!(database::get_buchungen_uebersicht(&pool, None, None).await.unwrap().anzahl_ohne_beleg, 1);

    // Liste mit ?jahr=2025: genau die drei 2025er Buchungen, neueste zuerst
    let f = BuchungsFilterParameter { jahr: Some("2025".into()), ..Default::default() }.validieren().unwrap();
    let liste = database::get_buchungen(&pool, &f).await.unwrap();
    let daten: Vec<String> = liste.iter().map(|b| b.datum().to_iso()).collect();
    assert_eq!(daten, vec!["2025-12-31", "2025-06-15", "2025-01-01"]);
    assert!(liste[1].beleg().is_none());

    // Jahre mit Buchungen, absteigend
    let jahre: Vec<i32> = database::get_buchungs_jahre(&pool).await.unwrap().iter().map(|j| j.jahr()).collect();
    assert_eq!(jahre, vec![2026, 2025, 2024]);
}

#[test]
fn test_geschaeftsjahr_info_json() {
    let info = GeschaeftsjahrInfo::from(Geschaeftsjahr::new(2025).unwrap());
    assert_eq!(serde_json::to_value(&info).unwrap(), serde_json::json!({"jahr": 2025, "von": "2025-01-01", "bis": "2025-12-31"}));
}
