use achtsam_entruempeln_software::database;
use achtsam_entruempeln_software::models::{Kunde, Auftrag, AuftragStatus, Rechnung};
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
        stundensatz: 45.0,
        kilometer_satz: 0.5,
        ..Default::default()
    }).await.unwrap();
    
    // Simulations-Logik für RE-Nummer aus main.rs
    let get_re_nr = |existing_count: usize, auftrag_id: i64| {
        if existing_count == 0 {
            format!("RE-{}-{}", Local::now().format("%Y"), auftrag_id)
        } else {
            format!("RE-{}-{}-{}", Local::now().format("%Y"), auftrag_id, existing_count + 1)
        }
    };

    // 3. Erste Rechnung erstellen
    let re_nr1 = get_re_nr(0, auftrag_id);
    database::create_rechnung(&pool, Rechnung {
        id: 0,
        auftrag_id,
        rechnungs_nummer: re_nr1.clone(),
        datum: "2024-04-10".into(),
        gesamt_netto: 100.0,
        gesamt_brutto: 119.0,
        pdf_pfad: "path1.pdf".into(),
        status: "Offen".into(),
    }).await.expect("Erste Rechnung sollte funktionieren");

    // 4. Zweite Rechnung erstellen
    let existing = database::get_rechnungen_for_auftrag(&pool, auftrag_id).await.unwrap();
    let re_nr2 = get_re_nr(existing.len(), auftrag_id);
    
    assert_ne!(re_nr1, re_nr2, "Rechnungsnummern MÜSSEN unterschiedlich sein");
    assert!(re_nr2.ends_with("-2"), "Zweite Rechnungsnummer sollte auf -2 enden");

    let res2 = database::create_rechnung(&pool, Rechnung {
        id: 0,
        auftrag_id,
        rechnungs_nummer: re_nr2,
        datum: "2024-04-10".into(),
        gesamt_netto: 200.0,
        gesamt_brutto: 238.0,
        pdf_pfad: "path2.pdf".into(),
        status: "Offen".into(),
    }).await;
    
    assert!(res2.is_ok(), "Zweite Rechnung sollte jetzt erfolgreich sein");
}
