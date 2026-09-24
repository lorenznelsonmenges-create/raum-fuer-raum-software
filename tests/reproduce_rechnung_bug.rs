use wendepunkt_software::database;
use wendepunkt_software::domain::{Euro, Stunden, Kilometer, EinsatzTyp, RechnungsNummer};
use wendepunkt_software::models::{Kunde, Auftrag, AuftragStatus, Rechnung};
use sqlx::SqlitePool;

async fn setup_db() -> SqlitePool {
    let db_url = "sqlite::memory:";
    let pool = SqlitePool::connect(db_url).await.unwrap();
    
    // Migrationen manuell ausführen, da sie im Filesystem liegen
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}

#[tokio::test]
async fn test_duplicate_rechnung_number_fails() {
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
    
    let re_nr = format!("R123456");
    
    // 3. Erste Rechnung erstellen
    let res1 = database::create_rechnung(&pool, Rechnung {
        id: 0,
        auftrag_id,
        rechnungs_nummer: RechnungsNummer::try_new(re_nr.clone()).unwrap(),
        datum: "2024-04-10".into(),
        gesamt_netto: Euro::from_euro_f64(100.0).unwrap(),
        gesamt_brutto: Euro::from_euro_f64(119.0).unwrap(),
        pdf_pfad: "path1.pdf".into(),
        status: "Offen".into(),
    }).await;
    
    assert!(res1.is_ok(), "Erste Rechnung sollte erfolgreich sein");
    
    // 4. Zweite Rechnung mit derselben Nummer erstellen
    let res2 = database::create_rechnung(&pool, Rechnung {
        id: 0,
        auftrag_id,
        rechnungs_nummer: RechnungsNummer::try_new(re_nr.clone()).unwrap(), // Gleiche Nummer!
        datum: "2024-04-10".into(),
        gesamt_netto: Euro::from_euro_f64(200.0).unwrap(),
        gesamt_brutto: Euro::from_euro_f64(238.0).unwrap(),
        pdf_pfad: "path2.pdf".into(),
        status: "Offen".into(),
    }).await;
    
    assert!(res2.is_err(), "Zweite Rechnung mit gleicher Nummer MUSS fehlschlagen");
    let err = res2.err().unwrap();
    println!("Erwarteter Fehler bei zweiter Rechnung: {:?}", err);
    assert!(err.to_string().contains("UNIQUE constraint failed: rechnungen.rechnungs_nummer"));
}
