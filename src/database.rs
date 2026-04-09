use sqlx::{sqlite::{SqlitePoolOptions, SqliteConnectOptions}, SqlitePool, Row};
use crate::models::{Kunde, Auftrag, AuftragStatus, Einsatz, Datei, RechnungNotiz, Rechnung};

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    if !std::path::Path::new("uploads").exists() {
        std::fs::create_dir("uploads").expect("Uploads-Verzeichnis konnte nicht erstellt werden");
    }

    let options = SqliteConnectOptions::new()
        .filename("achtsam.db")
        .create_if_missing(true)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

// --- Kunden ---
pub async fn get_all_kunden(pool: &SqlitePool) -> Result<Vec<Kunde>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, vorname, nachname, strasse, hausnummer, plz, ort, email, telefon, notizen FROM kunden").fetch_all(pool).await?;
    Ok(rows.into_iter().map(|row| Kunde {
        id: row.get("id"), vorname: row.get("vorname"), nachname: row.get("nachname"),
        strasse: row.get("strasse"), hausnummer: row.get("hausnummer"), plz: row.get("plz"),
        ort: row.get("ort"), email: row.get("email"), telefon: row.get("telefon"), notizen: row.get("notizen")
    }).collect())
}

pub async fn get_kunde_by_id(pool: &SqlitePool, id: i64) -> Result<Kunde, sqlx::Error> {
    let row = sqlx::query("SELECT id, vorname, nachname, strasse, hausnummer, plz, ort, email, telefon, notizen FROM kunden WHERE id = ?").bind(id).fetch_one(pool).await?;
    Ok(Kunde {
        id: row.get("id"), vorname: row.get("vorname"), nachname: row.get("nachname"),
        strasse: row.get("strasse"), hausnummer: row.get("hausnummer"), plz: row.get("plz"),
        ort: row.get("ort"), email: row.get("email"), telefon: row.get("telefon"), notizen: row.get("notizen")
    })
}

pub async fn create_kunde(pool: &SqlitePool, kunde: Kunde) -> Result<i64, sqlx::Error> {
    let res = sqlx::query("INSERT INTO kunden (vorname, nachname, strasse, hausnummer, plz, ort, email, telefon, notizen) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(kunde.vorname).bind(kunde.nachname).bind(kunde.strasse).bind(kunde.hausnummer).bind(kunde.plz).bind(kunde.ort).bind(kunde.email).bind(kunde.telefon).bind(kunde.notizen)
        .execute(pool).await?;
    Ok(res.last_insert_rowid())
}

pub async fn update_kunde(pool: &SqlitePool, id: i64, kunde: Kunde) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE kunden SET vorname = ?, nachname = ?, strasse = ?, hausnummer = ?, plz = ?, ort = ?, email = ?, telefon = ?, notizen = ? WHERE id = ?")
        .bind(kunde.vorname).bind(kunde.nachname).bind(kunde.strasse).bind(kunde.hausnummer).bind(kunde.plz).bind(kunde.ort).bind(kunde.email).bind(kunde.telefon).bind(kunde.notizen).bind(id)
        .execute(pool).await?;
    Ok(())
}

pub async fn delete_kunde(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    // 1. Finde alle Aufträge dieses Kunden
    let auftraege = sqlx::query("SELECT id FROM auftraege WHERE kunde_id = ?").bind(id).fetch_all(pool).await?;
    
    // 2. Lösche jeden Auftrag einzeln (inkl. physischer Dateien)
    for row in auftraege {
        let a_id: i64 = row.get("id");
        delete_auftrag(pool, a_id).await?;
    }

    // 3. Lösche den Kunden selbst
    sqlx::query("DELETE FROM kunden WHERE id = ?").bind(id).execute(pool).await?;
    Ok(())
}

// --- Aufträge ---
pub async fn get_auftrag_by_id(pool: &SqlitePool, id: i64) -> Result<Auftrag, sqlx::Error> {
    let row = sqlx::query("SELECT id, kunde_id, status, beschreibung, basis_pauschale, stundensatz, kilometer_satz, notizen FROM auftraege WHERE id = ?").bind(id).fetch_one(pool).await?;
    let status_str: String = row.get("status");
    let status = match status_str.as_str() {
        "InBearbeitung" => AuftragStatus::InBearbeitung,
        "Abgeschlossen" => AuftragStatus::Abgeschlossen,
        "Storniert" => AuftragStatus::Storniert,
        _ => AuftragStatus::AnfrageLaeuft,
    };
    Ok(Auftrag {
        id: row.get("id"), kunde_id: row.get("kunde_id"), status,
        beschreibung: row.get("beschreibung"), basis_pauschale: row.get("basis_pauschale"),
        stundensatz: row.get("stundensatz"), kilometer_satz: row.get("kilometer_satz"), notizen: row.get("notizen"),
        einsaetze: get_einsaetze_for_auftrag(pool, id).await?,
        dateien: get_dateien_for_auftrag(pool, id).await?,
        rechnungen: get_rechnungen_for_auftrag(pool, id).await?,
        rechnungs_notizen: get_rechnungs_notizen_for_auftrag(pool, id).await?
    })
}

pub async fn get_all_auftraege(pool: &SqlitePool) -> Result<Vec<Auftrag>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, kunde_id, status, beschreibung, basis_pauschale, stundensatz, kilometer_satz, notizen FROM auftraege").fetch_all(pool).await?;
    let mut list = Vec::new();
    for row in rows {
        let id = row.get("id");
        let status_str: String = row.get("status");
        let status = match status_str.as_str() {
            "InBearbeitung" => AuftragStatus::InBearbeitung,
            "Abgeschlossen" => AuftragStatus::Abgeschlossen,
            "Storniert" => AuftragStatus::Storniert,
            _ => AuftragStatus::AnfrageLaeuft,
        };
        list.push(Auftrag {
            id, kunde_id: row.get("kunde_id"), status,
            beschreibung: row.get("beschreibung"), basis_pauschale: row.get("basis_pauschale"),
            stundensatz: row.get("stundensatz"), kilometer_satz: row.get("kilometer_satz"), notizen: row.get("notizen"),
            einsaetze: get_einsaetze_for_auftrag(pool, id).await?,
            dateien: get_dateien_for_auftrag(pool, id).await?,
            rechnungen: get_rechnungen_for_auftrag(pool, id).await?,
            rechnungs_notizen: get_rechnungs_notizen_for_auftrag(pool, id).await?
        });
    }
    Ok(list)
}

pub async fn create_auftrag(pool: &SqlitePool, auftrag: Auftrag) -> Result<i64, sqlx::Error> {
    let status_str = format!("{:?}", auftrag.status);
    let res = sqlx::query("INSERT INTO auftraege (kunde_id, status, beschreibung, basis_pauschale, stundensatz, kilometer_satz, notizen) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind(auftrag.kunde_id)
        .bind(status_str)
        .bind(auftrag.beschreibung)
        .bind(auftrag.basis_pauschale)
        .bind(auftrag.stundensatz)
        .bind(auftrag.kilometer_satz)
        .bind(auftrag.notizen)
        .execute(pool).await?;
    Ok(res.last_insert_rowid())
}

pub async fn update_auftrag(pool: &SqlitePool, id: i64, auftrag: Auftrag) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE auftraege SET status = ?, beschreibung = ?, basis_pauschale = ?, stundensatz = ?, kilometer_satz = ?, notizen = ? WHERE id = ?")
        .bind(format!("{:?}", auftrag.status)).bind(auftrag.beschreibung).bind(auftrag.basis_pauschale).bind(auftrag.stundensatz).bind(auftrag.kilometer_satz).bind(auftrag.notizen).bind(id)
        .execute(pool).await?;
    Ok(())
}

pub async fn delete_auftrag(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    // 1. Hole alle physischen Pfade aus verschiedenen Tabellen
    let mut file_paths = Vec::new();

    // Pfade aus 'dateien'
    let d_rows = sqlx::query("SELECT dateipfad FROM dateien WHERE auftrag_id = ?").bind(id).fetch_all(pool).await?;
    for row in d_rows {
        let path: String = row.get("dateipfad");
        if !path.is_empty() {
            file_paths.push(path);
        }
    }

    // Pfade aus 'rechnungen'
    let r_rows = sqlx::query("SELECT pdf_pfad FROM rechnungen WHERE auftrag_id = ?").bind(id).fetch_all(pool).await?;
    for row in r_rows {
        let path: String = row.get("pdf_pfad");
        if !path.is_empty() {
            file_paths.push(path);
        }
    }

    // Pfade aus 'einsaetze'
    let e_rows = sqlx::query("SELECT signatur_pfad FROM einsaetze WHERE auftrag_id = ?").bind(id).fetch_all(pool).await?;
    for row in e_rows {
        if let Some(path) = row.get::<Option<String>, _>("signatur_pfad") {
            if !path.is_empty() {
                file_paths.push(path);
            }
        }
    }

    // 2. Lösche Dateien physisch von der Festplatte
    for path in file_paths {
        if std::path::Path::new(&path).exists() {
            let _ = std::fs::remove_file(&path);
        }
    }

    // 3. Lösche Auftrag (Kaskade löscht Einsätze, Dateien, Rechnungen etc. in DB)
    sqlx::query("DELETE FROM auftraege WHERE id = ?").bind(id).execute(pool).await?;
    Ok(())
}

// --- Einsätze ---
pub async fn create_einsatz(pool: &SqlitePool, e: Einsatz) -> Result<i64, sqlx::Error> {
    let res = sqlx::query("INSERT INTO einsaetze (auftrag_id, datum, kilometer, stunden, notiz, typ, signatur_pfad) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind(e.auftrag_id).bind(e.datum).bind(e.kilometer).bind(e.stunden).bind(e.notiz).bind(e.typ).bind(e.signatur_pfad).execute(pool).await?;
    Ok(res.last_insert_rowid())
}

pub async fn update_einsatz(pool: &SqlitePool, id: i64, e: Einsatz) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE einsaetze SET datum = ?, kilometer = ?, stunden = ?, notiz = ?, typ = ?, signatur_pfad = ? WHERE id = ?")
        .bind(e.datum).bind(e.kilometer).bind(e.stunden).bind(e.notiz).bind(e.typ).bind(e.signatur_pfad).bind(id).execute(pool).await?;
    Ok(())
}

pub async fn delete_einsatz(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM einsaetze WHERE id = ?").bind(id).execute(pool).await?;
    Ok(())
}

pub async fn get_einsaetze_for_auftrag(pool: &SqlitePool, auftrag_id: i64) -> Result<Vec<Einsatz>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, auftrag_id, datum, kilometer, stunden, notiz, typ, signatur_pfad FROM einsaetze WHERE auftrag_id = ?").bind(auftrag_id).fetch_all(pool).await?;
    Ok(rows.into_iter().map(|row| Einsatz {
        id: row.get("id"), auftrag_id: row.get("auftrag_id"), datum: row.get("datum"),
        kilometer: row.get("kilometer"), stunden: row.get("stunden"), notiz: row.get("notiz"),
        typ: row.get("typ"), signatur_pfad: row.get("signatur_pfad")
    }).collect())
}

// --- Dateien ---
pub async fn create_datei(pool: &SqlitePool, d: Datei) -> Result<i64, sqlx::Error> {
    let res = sqlx::query("INSERT INTO dateien (auftrag_id, dateiname, dateipfad, dateityp, kategorie) VALUES (?, ?, ?, ?, ?)")
        .bind(d.auftrag_id).bind(d.dateiname).bind(d.dateipfad).bind(d.dateityp).bind(d.kategorie).execute(pool).await?;
    Ok(res.last_insert_rowid())
}

pub async fn get_dateien_for_auftrag(pool: &SqlitePool, auftrag_id: i64) -> Result<Vec<Datei>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, auftrag_id, dateiname, dateipfad, dateityp, CAST(hochgeladen_am AS TEXT) as hochgeladen_am, kategorie FROM dateien WHERE auftrag_id = ?").bind(auftrag_id).fetch_all(pool).await?;
    Ok(rows.into_iter().map(|row| Datei {
        id: row.get("id"), auftrag_id: row.get("auftrag_id"), dateiname: row.get("dateiname"),
        dateipfad: row.get("dateipfad"), dateityp: row.get("dateityp"),
        hochgeladen_am: row.get("hochgeladen_am"), kategorie: row.get("kategorie")
    }).collect())
}

pub async fn delete_datei(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM dateien WHERE id = ?").bind(id).execute(pool).await?;
    Ok(())
}

// --- Rechnungen ---
pub async fn create_rechnung(pool: &SqlitePool, r: Rechnung) -> Result<i64, sqlx::Error> {
    let res = sqlx::query("INSERT INTO rechnungen (auftrag_id, rechnungs_nummer, datum, gesamt_netto, gesamt_brutto, pdf_pfad, status) VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind(r.auftrag_id).bind(r.rechnungs_nummer).bind(r.datum).bind(r.gesamt_netto).bind(r.gesamt_brutto).bind(r.pdf_pfad).bind(r.status).execute(pool).await?;
    Ok(res.last_insert_rowid())
}

pub async fn get_rechnungen_for_auftrag(pool: &SqlitePool, auftrag_id: i64) -> Result<Vec<Rechnung>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, auftrag_id, rechnungs_nummer, datum, gesamt_netto, gesamt_brutto, pdf_pfad, status FROM rechnungen WHERE auftrag_id = ?").bind(auftrag_id).fetch_all(pool).await?;
    Ok(rows.into_iter().map(|row| Rechnung {
        id: row.get("id"), auftrag_id: row.get("auftrag_id"), rechnungs_nummer: row.get("rechnungs_nummer"),
        datum: row.get("datum"), gesamt_netto: row.get("gesamt_netto"), gesamt_brutto: row.get("gesamt_brutto"),
        pdf_pfad: row.get("pdf_pfad"), status: row.get("status")
    }).collect())
}

pub async fn get_rechnungs_notizen_for_auftrag(pool: &SqlitePool, auftrag_id: i64) -> Result<Vec<RechnungNotiz>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, auftrag_id, text, auf_rechnung FROM rechnungs_notizen WHERE auftrag_id = ?").bind(auftrag_id).fetch_all(pool).await?;
    Ok(rows.into_iter().map(|row| RechnungNotiz {
        id: row.get("id"), auftrag_id: row.get("auftrag_id"), text: row.get("text"), auf_rechnung: row.get("auf_rechnung")
    }).collect())
}
