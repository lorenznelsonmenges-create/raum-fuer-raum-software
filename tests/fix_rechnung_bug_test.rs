use wendepunkt_software::database;
use wendepunkt_software::domain::{Euro, Stunden, Kilometer, EinsatzTyp, RechnungsNummer};
use wendepunkt_software::models::{Kunde, Auftrag, AuftragStatus, Rechnung};
use sqlx::SqlitePool;
use chrono::Local;

async fn setup_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}

#[tokio::test]
async fn test_multiple_rechnungen_for_one_auftrag_now_works() {
    let pool = setup_db().await;
    
    // 1. Kunde erstellen
    let kunde_id = database::create_kunde(&pool, Kunde {
        id: 0,
        vorname: "Test".into(),
        nachname: "Kunde".into(),
        ..Default::default()
    }).await.unwrap();
    
    // 2. Auftrag erstellen
    let auftrag_id = database::create_auftrag(&pool, Auftrag {
        id: 0,
        kunde_id,
        status: AuftragStatus::AnfrageLaeuft,
        beschreibung: "Test Auftrag".into(),
        stundensatz: Euro::from_euro_f64(45.0).unwrap(),
        kilometer_satz: Euro::from_euro_f64(0.5).unwrap(),
        ..Default::default()
    }).await.unwrap();
    
    // Simulations-Logik für RE-Nummer aus main.rs
    let get_re_nr = |existing_count: usize, auftrag_id: i64| {
        if existing_count == 0 {
            format!("R123{:03}", existing_count + 1)
        } else {
            format!("R123{:03}", existing_count + 1)
        }
    };

    // 3. Erste Rechnung erstellen
    let re_nr1 = get_re_nr(0, auftrag_id);
    database::create_rechnung(&pool, Rechnung {
        id: 0,
        auftrag_id,
        rechnungs_nummer: RechnungsNummer::try_new(re_nr1.clone()).unwrap(),
        datum: "2024-04-10".into(),
        gesamt_netto: Euro::from_euro_f64(100.0).unwrap(),
        gesamt_brutto: Euro::from_euro_f64(119.0).unwrap(),
        pdf_pfad: "path1.pdf".into(),
        status: "Offen".into(),
    }).await.expect("Erste Rechnung sollte funktionieren");

    // 4. Zweite Rechnung erstellen
    let existing = database::get_rechnungen_for_auftrag(&pool, auftrag_id).await.unwrap();
    let re_nr2 = get_re_nr(existing.len(), auftrag_id);
    
    assert_ne!(re_nr1, re_nr2, "Rechnungsnummern MÜSSEN unterschiedlich sein");
    assert!(re_nr2.ends_with("002"), "Zweite Rechnungsnummer sollte auf -2 enden");

    let res2 = database::create_rechnung(&pool, Rechnung {
        id: 0,
        auftrag_id,
        rechnungs_nummer: RechnungsNummer::try_new(re_nr2).unwrap(),
        datum: "2024-04-10".into(),
        gesamt_netto: Euro::from_euro_f64(200.0).unwrap(),
        gesamt_brutto: Euro::from_euro_f64(238.0).unwrap(),
        pdf_pfad: "path2.pdf".into(),
        status: "Offen".into(),
    }).await;
    
    assert!(res2.is_ok(), "Zweite Rechnung sollte jetzt erfolgreich sein");
}
