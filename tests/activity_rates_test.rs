use sqlx::SqlitePool;
use wendepunkt_software::database;
use wendepunkt_software::domain::{Euro, Stunden, EinsatzTyp};
use wendepunkt_software::models::{Kunde, Auftrag, Einsatz, Settings};
use wendepunkt_software::pdf;

async fn setup_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}

#[tokio::test]
async fn test_distinct_stundensatz_dienstleistung_and_nachbereitung() {
    let pool = setup_db().await;

    // 1. Settings anpassen: Dienstleistung 50€/h, Nachbereitung 35€/h, KM 0.60€/km
    let custom_settings = Settings {
        id: 1,
        stundensatz: Euro::from_cents(5000).unwrap(),
        stundensatz_nachbereitung: Euro::from_cents(3500).unwrap(),
        kilometer_satz: Euro::from_cents(60).unwrap(),
    };
    database::update_settings(&pool, custom_settings).await.unwrap();

    let loaded_settings = database::get_settings(&pool).await.unwrap();
    assert_eq!(loaded_settings.stundensatz.as_cents(), 5000);
    assert_eq!(loaded_settings.stundensatz_nachbereitung.as_cents(), 3500);
    assert_eq!(loaded_settings.kilometer_satz.as_cents(), 60);

    // 2. Kunde und Auftrag anlegen mit diesen Sätzen
    let kunde_id = database::create_kunde(&pool, Kunde {
        vorname: "Maria".into(),
        nachname: "Musterfrau".into(),
        ..Default::default()
    }).await.unwrap();

    let auftrag = Auftrag {
        id: 0,
        kunde_id,
        stundensatz: loaded_settings.stundensatz,
        stundensatz_nachbereitung: loaded_settings.stundensatz_nachbereitung,
        kilometer_satz: loaded_settings.kilometer_satz,
        ..Default::default()
    };
    let auftrag_id = database::create_auftrag(&pool, auftrag).await.unwrap();

    let loaded_auftrag = database::get_auftrag_by_id(&pool, auftrag_id).await.unwrap();
    assert_eq!(loaded_auftrag.stundensatz.as_cents(), 5000);
    assert_eq!(loaded_auftrag.stundensatz_nachbereitung.as_cents(), 3500);

    // 3. Einsätze erfassen: 2h Dienstleistung (2 * 50€ = 100€) und 3h Nachbereitung (3 * 35€ = 105€)
    let e1 = Einsatz {
        id: 0,
        auftrag_id,
        datum: "2026-09-28".into(),
        stunden: Stunden::try_new(2.0).unwrap(),
        typ: EinsatzTyp::Dienstleistung,
        ..Default::default()
    };
    let e2 = Einsatz {
        id: 0,
        auftrag_id,
        datum: "2026-09-28".into(),
        stunden: Stunden::try_new(3.0).unwrap(),
        typ: EinsatzTyp::Nachbereitung,
        ..Default::default()
    };
    database::create_einsatz(&pool, e1).await.unwrap();
    database::create_einsatz(&pool, e2).await.unwrap();

    let einsaetze = database::get_einsaetze_for_auftrag(&pool, auftrag_id).await.unwrap();
    let kunde = database::get_kunde_by_id(&pool, kunde_id).await.unwrap();

    // 4. Rechnung generieren und Netto-Berechnung prüfen: 100€ + 105€ = 205€
    let (_pdf, netto, brutto) = pdf::generate_dynamic_pdf(
        "templates/rechnung.html",
        &loaded_auftrag,
        &kunde,
        Some(&einsaetze),
        None,
        Some("R000001"),
        None,
        Some("admin"),
    ).unwrap();

    assert_eq!(netto.as_cents(), 20500, "Netto muss exakt 205,00 Euro sein (2h*50€ + 3h*35€)");
    assert_eq!(brutto.as_cents(), 24395, "Brutto muss exakt 243,95 Euro sein (205€ * 1.19)");
}
