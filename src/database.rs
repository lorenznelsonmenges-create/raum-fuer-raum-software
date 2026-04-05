use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::env;
use crate::models::{Kunde, Auftrag, AuftragStatus, Einsatz, Datei, RechnungsNotiz};

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:achtsam.db".to_string());

    if !std::path::Path::new("achtsam.db").exists() {
        std::fs::File::create("achtsam.db").expect("Datenbankdatei konnte nicht erstellt werden");
    }

    // Erstellt den Uploads-Ordner, falls er nicht existiert
    if !std::path::Path::new("uploads").exists() {
        std::fs::create_dir("uploads").expect("Uploads-Verzeichnis konnte nicht erstellt werden");
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}

// --- Kunden Funktionen ---

pub async fn get_all_kunden(pool: &SqlitePool) -> Result<Vec<Kunde>, sqlx::Error> {
    let kunden = sqlx::query_as!(
        Kunde,
        r#"SELECT id, vorname, nachname, strasse as "strasse!", hausnummer as "hausnummer!", plz as "plz!", stadt as "stadt!", email as "email!", telefon as "telefon!", notizen as "notizen!" FROM kunden"#
    )
    .fetch_all(pool)
    .await?;

    let mut kunden_liste = Vec::new();
    for mut k in kunden {
        k.auftraege = Vec::new(); // Später: Aufträge laden
        kunden_liste.push(k);
    }

    Ok(kunden_liste)
}

pub async fn create_kunde(pool: &SqlitePool, kunde: Kunde) -> Result<i64, sqlx::Error> {
    let id = sqlx::query!(
        r#"INSERT INTO kunden (vorname, nachname, strasse, hausnummer, plz, stadt, email, telefon, notizen) 
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        kunde.vorname,
        kunde.nachname,
        kunde.strasse,
        kunde.hausnummer,
        kunde.plz,
        kunde.stadt,
        kunde.email,
        kunde.telefon,
        kunde.notizen
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    Ok(id)
}

// --- Auftrags Funktionen ---

pub async fn create_auftrag(pool: &SqlitePool, auftrag: Auftrag) -> Result<i64, sqlx::Error> {
    let status_str = format!("{:?}", auftrag.status);
    let id = sqlx::query!(
        r#"INSERT INTO auftraege (kunde_id, status, beschreibung, basis_pauschale, preis_manuell, notizen) 
           VALUES (?, ?, ?, ?, ?, ?)"#,
        auftrag.kunde_id,
        status_str,
        auftrag.beschreibung,
        auftrag.basis_pauschale,
        auftrag.preis_manuell,
        auftrag.notizen
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    Ok(id)
}

// --- Einsatz Funktionen (Arbeitsdokumentation) ---

pub async fn create_einsatz(pool: &SqlitePool, einsatz: Einsatz) -> Result<i64, sqlx::Error> {
    let id = sqlx::query!(
        r#"INSERT INTO einsaetze (auftrag_id, datum, kilometer, stunden, notiz) 
           VALUES (?, ?, ?, ?, ?)"#,
        einsatz.auftrag_id,
        einsatz.datum,
        einsatz.kilometer,
        einsatz.stunden,
        einsatz.notiz
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    Ok(id)
}

pub async fn get_einsaetze_for_auftrag(pool: &SqlitePool, auftrag_id: i64) -> Result<Vec<Einsatz>, sqlx::Error> {
    let e = sqlx::query_as!(
        Einsatz,
        r#"SELECT id, auftrag_id, datum, kilometer, stunden, notiz as "notiz!" FROM einsaetze WHERE auftrag_id = ?"#,
        auftrag_id
    )
    .fetch_all(pool)
    .await?;

    Ok(e)
}

// --- Datei Funktionen ---

pub async fn create_datei(pool: &SqlitePool, datei: Datei) -> Result<i64, sqlx::Error> {
    let id = sqlx::query!(
        r#"INSERT INTO dateien (auftrag_id, dateiname, dateipfad, dateityp) 
           VALUES (?, ?, ?, ?)"#,
        datei.auftrag_id,
        datei.dateiname,
        datei.dateipfad,
        datei.dateityp
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    Ok(id)
}

pub async fn get_dateien_for_auftrag(pool: &SqlitePool, auftrag_id: i64) -> Result<Vec<Datei>, sqlx::Error> {
    let d = sqlx::query_as!(
        Datei,
        r#"SELECT id, auftrag_id, dateiname, dateipfad, dateityp, hochgeladen_am as "hochgeladen_am!" FROM dateien WHERE auftrag_id = ?"#,
        auftrag_id
    )
    .fetch_all(pool)
    .await?;

    Ok(d)
}

// --- RechnungsNotiz Funktionen ---

pub async fn create_rechnungs_notiz(pool: &SqlitePool, notiz: RechnungsNotiz) -> Result<i64, sqlx::Error> {
    let id = sqlx::query!(
        r#"INSERT INTO rechnungs_notizen (auftrag_id, text, auf_rechnung) 
           VALUES (?, ?, ?)"#,
        notiz.auftrag_id,
        notiz.text,
        notiz.auf_rechnung
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    Ok(id)
}
