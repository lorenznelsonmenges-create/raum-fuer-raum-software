mod models;
mod database;
mod error;
mod pdf;

use axum::{
    routing::{get, post},
    extract::{State, Path, Multipart},
    response::Html,
    Json,
    Router,
};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::fs;
use crate::models::{Kunde, Auftrag, Einsatz, Datei, Rechnung};
use crate::error::AppError;
use chrono::Datelike;

#[tokio::main]
async fn main() {
    // Datenbank initialisieren
    let pool = database::init_db().await.expect("Datenbank konnte nicht initialisiert werden");

    let app = Router::new()
        .route("/", get(index))
        .route("/api/kunden", get(list_kunden))
        .route("/api/kunden", post(add_kunde))
        .route("/api/kunden/:id", get(get_kunde))
        .route("/api/kunden/:id", post(update_kunde))
        .route("/api/auftraege", get(list_auftraege))
        .route("/api/auftraege", post(add_auftrag))
        .route("/api/auftraege/:id", get(get_auftrag))
        .route("/api/auftraege/:id", post(update_auftrag))
        .route("/api/auftraege/:id/rechnung", post(create_rechnung))
        .route("/api/auftraege/:id/einsaetze", get(list_einsaetze_for_auftrag))
        .route("/api/einsaetze", post(add_einsatz))
        .route("/api/einsaetze/:id", post(update_einsatz_handler))
        .route("/api/einsaetze/:id/delete", post(delete_einsatz_handler))
        .route("/api/auftraege/:id/upload", post(upload_datei))
        .route("/api/auftraege/:id/dateien", get(list_dateien))
        .nest_service("/uploads", tower_http::services::ServeDir::new("uploads"))
        .nest_service("/static", tower_http::services::ServeDir::new("static"))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server läuft auf http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<String> {
    match fs::read_to_string("static/index.html").await {
        Ok(content) => Html(content),
        Err(_) => Html("<h1>Index nicht gefunden</h1>".to_string()),
    }
}

// Handler: Alle Aufträge abfragen
async fn list_auftraege(State(pool): State<SqlitePool>) -> Result<Json<Vec<Auftrag>>, AppError> {
    let auftraege = database::get_all_auftraege(&pool).await?;
    Ok(Json(auftraege))
}

// Handler: Neuen Auftrag anlegen
async fn add_auftrag(State(pool): State<SqlitePool>, Json(payload): Json<Auftrag>) -> Result<Json<i64>, AppError> {
    let id = database::create_auftrag(&pool, payload).await?;
    Ok(Json(id))
}

// Handler: Auftrag abfragen (Einzeln)
async fn get_auftrag(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Auftrag>, AppError> {
    let auftrag = database::get_auftrag_by_id(&pool, id).await?;
    Ok(Json(auftrag))
}

// Handler: Auftrag aktualisieren
async fn update_auftrag(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<Auftrag>,
) -> Result<(), AppError> {
    database::update_auftrag(&pool, id, payload).await?;
    Ok(())
}

// Handler: Rechnung erstellen
async fn create_rechnung(
    State(pool): State<SqlitePool>,
    Path(auftrag_id): Path<i64>,
) -> Result<Json<i64>, AppError> {
    // 1. Daten laden
    let auftrag = database::get_auftrag_by_id(&pool, auftrag_id).await?;
    let kunde = database::get_kunde_by_id(&pool, auftrag.kunde_id).await?;
    let einsaetze = database::get_einsaetze_for_auftrag(&pool, auftrag_id).await?;
    let notizen = database::get_rechnungs_notizen_for_auftrag(&pool, auftrag_id).await?;

    // 2. Rechnungsnummer generieren
    let last_nr = database::get_last_rechnungs_nummer(&pool).await?;
    let year = chrono::Local::now().year();
    let next_id = match last_nr {
        Some(nr) => {
            let parts: Vec<&str> = nr.split('-').collect();
            if parts.len() == 3 {
                parts[2].parse::<i32>().unwrap_or(0) + 1
            } else {
                1
            }
        }
        None => 1,
    };
    let re_nr = format!("RE-{}-{:03}", year, next_id);

    // 3. PDF generieren
    let pdf_data = pdf::generate_invoice_pdf(&auftrag, &kunde, &einsaetze, &notizen, &re_nr)
        .map_err(|e| AppError::BadRequest(e))?;

    // 4. PDF speichern
    let file_name = format!("{}_rechnung.pdf", re_nr);
    let file_path = std::path::Path::new("uploads").join(&file_name);
    fs::write(&file_path, pdf_data).await.map_err(|e| AppError::BadRequest(e.to_string()))?;

    // 5. In DB speichern
    let rechnung = Rechnung {
        id: 0,
        auftrag_id,
        rechnungs_nummer: re_nr,
        datum: chrono::Local::now().to_rfc3339(),
        gesamt_netto: 0.0, 
        gesamt_brutto: 0.0,
        pdf_pfad: file_path.to_string_lossy().to_string(),
        status: "OFFEN".to_string(),
    };

    let id = database::create_rechnung(&pool, rechnung).await?;
    Ok(Json(id))
}

// Handler: Alle Kunden abfragen
async fn list_kunden(State(pool): State<SqlitePool>) -> Result<Json<Vec<Kunde>>, AppError> {
    let kunden = database::get_all_kunden(&pool).await?;
    Ok(Json(kunden))
}

// Handler: Neuen Kunden anlegen
async fn add_kunde(State(pool): State<SqlitePool>, Json(payload): Json<Kunde>) -> Result<Json<i64>, AppError> {
    let id = database::create_kunde(&pool, payload).await?;
    Ok(Json(id))
}

// Handler: Kunden abfragen (Einzeln)
async fn get_kunde(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Kunde>, AppError> {
    let kunde = database::get_kunde_by_id(&pool, id).await?;
    Ok(Json(kunde))
}

// Handler: Kunden aktualisieren
async fn update_kunde(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<Kunde>,
) -> Result<(), AppError> {
    database::update_kunde(&pool, id, payload).await?;
    Ok(())
}

// Handler: Einsätze für Auftrag abfragen
async fn list_einsaetze_for_auftrag(
    State(pool): State<SqlitePool>,
    Path(auftrag_id): Path<i64>,
) -> Result<Json<Vec<Einsatz>>, AppError> {
    let einsaetze = database::get_einsaetze_for_auftrag(&pool, auftrag_id).await?;
    Ok(Json(einsaetze))
}

// Handler: Neuen Einsatz anlegen
async fn add_einsatz(State(pool): State<SqlitePool>, Json(payload): Json<Einsatz>) -> Result<Json<i64>, AppError> {
    let id = database::create_einsatz(&pool, payload).await?;
    Ok(Json(id))
}

// Handler: Einsatz aktualisieren
async fn update_einsatz_handler(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<Einsatz>,
) -> Result<(), AppError> {
    database::update_einsatz(&pool, id, payload).await?;
    Ok(())
}

// Handler: Einsatz löschen
async fn delete_einsatz_handler(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    database::delete_einsatz(&pool, id).await?;
    Ok(())
}

// Handler: Dateien hochladen
async fn upload_datei(
    State(pool): State<SqlitePool>,
    Path(auftrag_id): Path<i64>,
    mut multipart: Multipart,
) -> Result<Json<Vec<i64>>, AppError> {
    let mut ids = Vec::new();
    let mut kategorie = "SONSTIGES".to_string();

    while let Some(field) = multipart.next_field().await.map_err(|e| AppError::BadRequest(e.to_string()))? {
        let name = field.name().unwrap_or("file").to_string();

        if name == "kategorie" {
            if let Ok(value) = field.text().await {
                kategorie = value;
            }
            continue;
        }

        let file_name = field.file_name().unwrap_or("unnamed").to_string();
        let content_type = field.content_type().unwrap_or("application/octet-stream").to_string();
        let data = field.bytes().await.map_err(|e| AppError::BadRequest(e.to_string()))?;

        // Dateipfad erstellen (uploads/auftrag_id_kategorie_dateiname)
        let safe_file_name = format!("{}_{}_{}", auftrag_id, kategorie, file_name);
        let path = std::path::Path::new("uploads").join(&safe_file_name);

        // Datei auf Disk speichern
        fs::write(&path, data).await.map_err(|e| {
            eprintln!("Fehler beim Speichern der Datei: {:?}", e);
            AppError::BadRequest("Datei konnte nicht gespeichert werden".to_string())
        })?;

        // Metadaten in DB speichern
        let neue_datei = Datei {
            id: 0,
            auftrag_id,
            dateiname: file_name,
            dateipfad: path.to_string_lossy().to_string(),
            dateityp: content_type,
            hochgeladen_am: String::new(), 
            kategorie: kategorie.clone(),
        };

        let id = database::create_datei(&pool, neue_datei).await?;
        ids.push(id);
    }

    Ok(Json(ids))
}

// Handler: Dateien für Auftrag abfragen
async fn list_dateien(
    State(pool): State<SqlitePool>,
    Path(auftrag_id): Path<i64>,
) -> Result<Json<Vec<Datei>>, AppError> {
    let dateien = database::get_dateien_for_auftrag(&pool, auftrag_id).await?;
    Ok(Json(dateien))
}
