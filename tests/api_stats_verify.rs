use achtsam_entruempeln_software::database;
use achtsam_entruempeln_software::models::{Kunde, Auftrag, AuftragStatus};
use sqlx::SqlitePool;

async fn setup_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}

#[tokio::test]
async fn test_dashboard_stats_calculation() {
    let pool = setup_db().await;
    
    // 1. Initialer Check (alles 0)
    let stats = database::get_dashboard_stats(&pool).await.unwrap();
    assert_eq!(stats.anfrage_laeuft, 0);
    assert_eq!(stats.in_bearbeitung, 0);
    assert_eq!(stats.abgeschlossen, 0);
    assert_eq!(stats.storniert, 0);
    assert_eq!(stats.aktuelle_auftraege, 0);

    // 2. Kunde anlegen
    let kunde_id = database::create_kunde(&pool, Kunde {
        id: 0,
        vorname: "Test".into(),
        nachname: "Kunde".into(),
        ..Default::default()
    }).await.unwrap();

    // 3. Einen Auftrag in "AnfrageLaeuft" anlegen
    database::create_auftrag(&pool, Auftrag {
        id: 0,
        kunde_id,
        status: AuftragStatus::AnfrageLaeuft,
        beschreibung: "Anfrage 1".into(),
        ..Default::default()
    }).await.unwrap();

    let stats = database::get_dashboard_stats(&pool).await.unwrap();
    assert_eq!(stats.anfrage_laeuft, 1);
    assert_eq!(stats.aktuelle_auftraege, 1);

    // 4. Einen weiteren in "InBearbeitung" anlegen
    database::create_auftrag(&pool, Auftrag {
        id: 0,
        kunde_id,
        status: AuftragStatus::InBearbeitung,
        beschreibung: "Bearbeitung 1".into(),
        ..Default::default()
    }).await.unwrap();

    let stats = database::get_dashboard_stats(&pool).await.unwrap();
    assert_eq!(stats.anfrage_laeuft, 1);
    assert_eq!(stats.in_bearbeitung, 1);
    assert_eq!(stats.aktuelle_auftraege, 2);

    // 5. Einen in "Abgeschlossen" anlegen
    database::create_auftrag(&pool, Auftrag {
        id: 0,
        kunde_id,
        status: AuftragStatus::Abgeschlossen,
        beschreibung: "Fertig 1".into(),
        ..Default::default()
    }).await.unwrap();

    let stats = database::get_dashboard_stats(&pool).await.unwrap();
    assert_eq!(stats.abgeschlossen, 1);
    assert_eq!(stats.aktuelle_auftraege, 2); // Sollte 1 (Anfrage) + 1 (InBearbeitung) sein

    // 6. Einen in "Storniert" anlegen
    database::create_auftrag(&pool, Auftrag {
        id: 0,
        kunde_id,
        status: AuftragStatus::Storniert,
        beschreibung: "Abgebrochen 1".into(),
        ..Default::default()
    }).await.unwrap();

    let stats = database::get_dashboard_stats(&pool).await.unwrap();
    assert_eq!(stats.storniert, 1);
    assert_eq!(stats.aktuelle_auftraege, 2);
}
