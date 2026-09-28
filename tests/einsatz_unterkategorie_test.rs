use wendepunkt_software::database;
use wendepunkt_software::domain::{Euro, Stunden, Kilometer, EinsatzTyp};
use wendepunkt_software::models::{Kunde, Auftrag, AuftragStatus, Einsatz};
use wendepunkt_software::pdf::generate_dynamic_pdf;
use sqlx::SqlitePool;

async fn setup_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}

#[tokio::test]
async fn test_einsatz_with_fractional_hours_and_unterkategorie() {
    let pool = setup_db().await;

    let kunde_id = database::create_kunde(&pool, Kunde {
        id: 0,
        vorname: "Erika".into(),
        nachname: "Musterfrau".into(),
        ..Default::default()
    }).await.unwrap();

    let auftrag_id = database::create_auftrag(&pool, Auftrag {
        id: 0,
        kunde_id,
        status: AuftragStatus::InBearbeitung,
        stundensatz: Euro::from_euro_f64(40.0).unwrap(),
        kilometer_satz: Euro::from_euro_f64(0.5).unwrap(),
        ..Default::default()
    }).await.unwrap();

    // 1. Einsatz: Dienstleistung mit 0.75 Stunden, Unterkategorie "Praktische Unterstützung", Notiz
    let einsatz1 = Einsatz {
        id: 0,
        auftrag_id,
        datum: "2026-09-28".into(),
        kilometer: Kilometer::default(),
        stunden: Stunden::try_new(0.75).unwrap(),
        notiz: "Praktische Hilfe im Garten".into(),
        typ: EinsatzTyp::Dienstleistung,
        unterkategorie: Some("Praktische Unterstützung".into()),
        signatur_pfad: None,
    };
    let e1_id = database::create_einsatz(&pool, einsatz1).await.unwrap();
    assert!(e1_id > 0);

    // 2. Einsatz: Nachbereitung mit 1.25 Stunden, Unterkategorie "Auswertung", Notiz
    let einsatz2 = Einsatz {
        id: 0,
        auftrag_id,
        datum: "2026-09-28".into(),
        kilometer: Kilometer::default(),
        stunden: Stunden::try_new(1.25).unwrap(),
        notiz: "Bericht und Analyse".into(),
        typ: EinsatzTyp::Nachbereitung,
        unterkategorie: Some("Auswertung".into()),
        signatur_pfad: None,
    };
    let e2_id = database::create_einsatz(&pool, einsatz2).await.unwrap();
    assert!(e2_id > 0);

    // 3. Auslesen und prüfen
    let einsaetze = database::get_einsaetze_for_auftrag(&pool, auftrag_id).await.unwrap();
    assert_eq!(einsaetze.len(), 2);

    let e1 = &einsaetze[0];
    assert_eq!(e1.stunden.value(), 0.75);
    assert_eq!(e1.typ, EinsatzTyp::Dienstleistung);
    assert_eq!(e1.unterkategorie, Some("Praktische Unterstützung".into()));
    assert_eq!(e1.notiz, "Praktische Hilfe im Garten");

    let e2 = &einsaetze[1];
    assert_eq!(e2.stunden.value(), 1.25);
    assert_eq!(e2.typ, EinsatzTyp::Nachbereitung);
    assert_eq!(e2.unterkategorie, Some("Auswertung".into()));
    assert_eq!(e2.notiz, "Bericht und Analyse");

    // 4. Test Serde JSON round-trip
    let json_str = serde_json::to_string(&e1).unwrap();
    let deserialized: Einsatz = serde_json::from_str(&json_str).unwrap();
    assert_eq!(deserialized.stunden.value(), 0.75);
    assert_eq!(deserialized.typ, EinsatzTyp::Dienstleistung);
    assert_eq!(deserialized.unterkategorie, Some("Praktische Unterstützung".into()));
    assert_eq!(deserialized.notiz, "Praktische Hilfe im Garten");

    // 5. Test PDF Generation mit Einsätzen
    let auftrag = database::get_auftrag_by_id(&pool, auftrag_id).await.unwrap();
    let kunde = database::get_kunde_by_id(&pool, kunde_id).await.unwrap();
    let (pdf_bytes, netto, brutto) = generate_dynamic_pdf(
        "templates/rechnung.html",
        &auftrag,
        &kunde,
        Some(&einsaetze),
        None,
        Some("R100001"),
        None,
        Some("Stefanie Ruf"),
    ).unwrap();

    assert!(!pdf_bytes.is_empty());
    // 0.75h * 40€ = 30€; 1.25h * 40€ = 50€; Netto = 80€; Brutto = 80 * 1.19 = 95.20€
    assert_eq!(netto.as_cents(), 8000);
    assert_eq!(brutto.as_cents(), 9520);
}
