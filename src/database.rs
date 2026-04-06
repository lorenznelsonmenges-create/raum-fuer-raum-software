use sqlx::{sqlite::SqlitePoolOptions, SqlitePool, Row};
use std::env;
use crate::models::{Kunde, Auftrag, AuftragStatus, Einsatz, Datei, RechnungsNotiz, Rechnung};

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
    let rows = sqlx::query(
        "SELECT id, vorname, nachname, strasse, hausnummer, plz, ort, email, telefon, notizen FROM kunden"
    )
    .fetch_all(pool)
    .await?;

    let mut kunden_liste = Vec::new();
    for row in rows {
        kunden_liste.push(Kunde {
            id: row.get("id"),
            vorname: row.get("vorname"),
            nachname: row.get("nachname"),
            strasse: row.get("strasse"),
            hausnummer: row.get("hausnummer"),
            plz: row.get("plz"),
            ort: row.get("ort"),
            email: row.get("email"),
            telefon: row.get("telefon"),
            notizen: row.get("notizen"),
            auftraege: Vec::new(),
        });
    }

    Ok(kunden_liste)
}

pub async fn get_kunde_by_id(pool: &SqlitePool, id: i64) -> Result<Kunde, sqlx::Error> {
    let row = sqlx::query(
        "SELECT id, vorname, nachname, strasse, hausnummer, plz, ort, email, telefon, notizen FROM kunden WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(Kunde {
        id: row.get("id"),
        vorname: row.get("vorname"),
        nachname: row.get("nachname"),
        strasse: row.get("strasse"),
        hausnummer: row.get("hausnummer"),
        plz: row.get("plz"),
        ort: row.get("ort"),
        email: row.get("email"),
        telefon: row.get("telefon"),
        notizen: row.get("notizen"),
        auftraege: Vec::new(),
    })
}

pub async fn create_kunde(pool: &SqlitePool, kunde: Kunde) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO kunden (vorname, nachname, strasse, hausnummer, plz, ort, email, telefon, notizen) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(kunde.vorname)
    .bind(kunde.nachname)
    .bind(kunde.strasse)
    .bind(kunde.hausnummer)
    .bind(kunde.plz)
    .bind(kunde.ort)
    .bind(kunde.email)
    .bind(kunde.telefon)
    .bind(kunde.notizen)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn update_kunde(pool: &SqlitePool, id: i64, kunde: Kunde) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE kunden SET vorname = ?, nachname = ?, strasse = ?, hausnummer = ?, plz = ?, ort = ?, email = ?, telefon = ?, notizen = ? WHERE id = ?"
    )
    .bind(kunde.vorname)
    .bind(kunde.nachname)
    .bind(kunde.strasse)
    .bind(kunde.hausnummer)
    .bind(kunde.plz)
    .bind(kunde.ort)
    .bind(kunde.email)
    .bind(kunde.telefon)
    .bind(kunde.notizen)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

// --- Auftrags Funktionen ---

pub async fn get_auftrag_by_id(pool: &SqlitePool, id: i64) -> Result<Auftrag, sqlx::Error> {
    let row = sqlx::query(
        "SELECT id, kunde_id, status, beschreibung, basis_pauschale, preis_manuell, notizen FROM auftraege WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    let status_str: String = row.get("status");
    let status = match status_str.as_str() {
        "Besichtigt" => AuftragStatus::Besichtigt,
        "Durchfuehrung" => AuftragStatus::Durchfuehrung,
        "Abgeschlossen" => AuftragStatus::Abgeschlossen,
        "Storniert" => AuftragStatus::Storniert,
        _ => AuftragStatus::Angefragt,
    };

    let mut auftrag = Auftrag {
        id: row.get("id"),
        kunde_id: row.get("kunde_id"),
        status,
        beschreibung: row.get("beschreibung"),
        basis_pauschale: row.get("basis_pauschale"),
        preis_manuell: row.get("preis_manuell"),
        notizen: row.get("notizen"),
        einsaetze: Vec::new(),
        dateien: Vec::new(),
        rechnungen: Vec::new(),
        rechnungs_notizen: Vec::new(),
    };

    auftrag.rechnungen = get_rechnungen_for_auftrag(pool, id).await?;

    Ok(auftrag)
}

pub async fn get_all_auftraege(pool: &SqlitePool) -> Result<Vec<Auftrag>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT id, kunde_id, status, beschreibung, basis_pauschale, preis_manuell, notizen FROM auftraege"
    )
    .fetch_all(pool)
    .await?;

    let mut auftraege = Vec::new();
    for row in rows {
        let status_str: String = row.get("status");
        let status = match status_str.as_str() {
            "Besichtigt" => AuftragStatus::Besichtigt,
            "Durchfuehrung" => AuftragStatus::Durchfuehrung,
            "Abgeschlossen" => AuftragStatus::Abgeschlossen,
            "Storniert" => AuftragStatus::Storniert,
            _ => AuftragStatus::Angefragt,
        };

        let mut auftrag = Auftrag {
            id: row.get("id"),
            kunde_id: row.get("kunde_id"),
            status,
            beschreibung: row.get("beschreibung"),
            basis_pauschale: row.get("basis_pauschale"),
            preis_manuell: row.get("preis_manuell"),
            notizen: row.get("notizen"),
            einsaetze: Vec::new(),
            dateien: Vec::new(),
            rechnungen: Vec::new(),
            rechnungs_notizen: Vec::new(),
        };

        auftrag.rechnungen = get_rechnungen_for_auftrag(pool, auftrag.id).await?;
        auftraege.push(auftrag);
    }

    Ok(auftraege)
}

pub async fn create_auftrag(pool: &SqlitePool, auftrag: Auftrag) -> Result<i64, sqlx::Error> {
    let status_str = format!("{:?}", auftrag.status);
    let result = sqlx::query(
        "INSERT INTO auftraege (kunde_id, status, beschreibung, basis_pauschale, preis_manuell, notizen) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(auftrag.kunde_id)
    .bind(status_str)
    .bind(auftrag.beschreibung)
    .bind(auftrag.basis_pauschale)
    .bind(auftrag.preis_manuell)
    .bind(auftrag.notizen)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn update_auftrag(pool: &SqlitePool, id: i64, auftrag: Auftrag) -> Result<(), sqlx::Error> {
    let status_str = format!("{:?}", auftrag.status);
    sqlx::query(
        "UPDATE auftraege SET status = ?, beschreibung = ?, basis_pauschale = ?, preis_manuell = ?, notizen = ? WHERE id = ?"
    )
    .bind(status_str)
    .bind(auftrag.beschreibung)
    .bind(auftrag.basis_pauschale)
    .bind(auftrag.preis_manuell)
    .bind(auftrag.notizen)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

// --- Einsatz Funktionen (Arbeitsdokumentation) ---

pub async fn create_einsatz(pool: &SqlitePool, einsatz: Einsatz) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO einsaetze (auftrag_id, datum, kilometer, stunden, notiz, typ) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(einsatz.auftrag_id)
    .bind(einsatz.datum)
    .bind(einsatz.kilometer)
    .bind(einsatz.stunden)
    .bind(einsatz.notiz)
    .bind(einsatz.typ)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_einsaetze_for_auftrag(pool: &SqlitePool, auftrag_id: i64) -> Result<Vec<Einsatz>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT id, auftrag_id, datum, kilometer, stunden, notiz, typ FROM einsaetze WHERE auftrag_id = ?"
    )
    .bind(auftrag_id)
    .fetch_all(pool)
    .await?;

    let mut einsaetze = Vec::new();
    for row in rows {
        einsaetze.push(Einsatz {
            id: row.get("id"),
            auftrag_id: row.get("auftrag_id"),
            datum: row.get("datum"),
            kilometer: row.get("kilometer"),
            stunden: row.get("stunden"),
            notiz: row.get("notiz"),
            typ: row.get("typ"),
        });
    }

    Ok(einsaetze)
}

// --- Datei Funktionen ---

pub async fn create_datei(pool: &SqlitePool, datei: Datei) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO dateien (auftrag_id, dateiname, dateipfad, dateityp, kategorie) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(datei.auftrag_id)
    .bind(datei.dateiname)
    .bind(datei.dateipfad)
    .bind(datei.dateityp)
    .bind(datei.kategorie)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_dateien_for_auftrag(pool: &SqlitePool, auftrag_id: i64) -> Result<Vec<Datei>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT id, auftrag_id, dateiname, dateipfad, dateityp, CAST(hochgeladen_am AS TEXT) as hochgeladen_am, kategorie FROM dateien WHERE auftrag_id = ?"
    )
    .bind(auftrag_id)
    .fetch_all(pool)
    .await?;

    let mut dateien = Vec::new();
    for row in rows {
        dateien.push(Datei {
            id: row.get("id"),
            auftrag_id: row.get("auftrag_id"),
            dateiname: row.get("dateiname"),
            dateipfad: row.get("dateipfad"),
            dateityp: row.get("dateityp"),
            hochgeladen_am: row.get("hochgeladen_am"),
            kategorie: row.get("kategorie"),
        });
    }

    Ok(dateien)
}

// --- Rechnungs Funktionen ---

pub async fn create_rechnung(pool: &SqlitePool, rechnung: Rechnung) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO rechnungen (auftrag_id, rechnungs_nummer, datum, gesamt_netto, gesamt_brutto, pdf_pfad, status) VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(rechnung.auftrag_id)
    .bind(rechnung.rechnungs_nummer)
    .bind(rechnung.datum)
    .bind(rechnung.gesamt_netto)
    .bind(rechnung.gesamt_brutto)
    .bind(rechnung.pdf_pfad)
    .bind(rechnung.status)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_rechnungen_for_auftrag(pool: &SqlitePool, auftrag_id: i64) -> Result<Vec<Rechnung>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT id, auftrag_id, rechnungs_nummer, datum, gesamt_netto, gesamt_brutto, pdf_pfad, status FROM rechnungen WHERE auftrag_id = ?"
    )
    .bind(auftrag_id)
    .fetch_all(pool)
    .await?;

    let mut rechnungen = Vec::new();
    for row in rows {
        rechnungen.push(Rechnung {
            id: row.get("id"),
            auftrag_id: row.get("auftrag_id"),
            rechnungs_nummer: row.get("rechnungs_nummer"),
            datum: row.get("datum"),
            gesamt_netto: row.get("gesamt_netto"),
            gesamt_brutto: row.get("gesamt_brutto"),
            pdf_pfad: row.get("pdf_pfad"),
            status: row.get("status"),
        });
    }

    Ok(rechnungen)
}

pub async fn get_last_rechnungs_nummer(pool: &SqlitePool) -> Result<Option<String>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT rechnungs_nummer FROM rechnungen ORDER BY id DESC LIMIT 1"
    )
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| r.get("rechnungs_nummer")))
}

// --- RechnungsNotiz Funktionen ---

pub async fn create_rechnungs_notiz(pool: &SqlitePool, notiz: RechnungsNotiz) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO rechnungs_notizen (auftrag_id, text, auf_rechnung) VALUES (?, ?, ?)"
    )
    .bind(notiz.auftrag_id)
    .bind(notiz.text)
    .bind(notiz.auf_rechnung)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_rechnungs_notizen_for_auftrag(pool: &SqlitePool, auftrag_id: i64) -> Result<Vec<RechnungsNotiz>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT id, auftrag_id, text, auf_rechnung FROM rechnungs_notizen WHERE auftrag_id = ?"
    )
    .bind(auftrag_id)
    .fetch_all(pool)
    .await?;

    let mut notizen = Vec::new();
    for row in rows {
        notizen.push(RechnungsNotiz {
            id: row.get("id"),
            auftrag_id: row.get("auftrag_id"),
            text: row.get("text"),
            auf_rechnung: row.get("auf_rechnung"),
        });
    }

    Ok(notizen)
}
