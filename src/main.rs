mod models;
mod database;
mod error;

use axum::{
    routing::{get, post},
    extract::{State, Path, Multipart},
    Json,
    Router,
};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::fs;
use models::{Kunde, Einsatz, Datei};
use error::AppError;

#[tokio::main]
async fn main() {
    println!("--- Achtsam Entrümpeln: Backend-Server wird gestartet ---");

    // Datenbank initialisieren
    let pool = match database::init_db().await {
        Ok(pool) => {
            println!("Datenbank erfolgreich verbunden und Migrationen ausgeführt.");
            pool
        }
        Err(e) => {
            eprintln!("Fehler beim Initialisieren der Datenbank: {:?}", e);
            return;
        }
    };

    // Router definieren mit Shared State (DB Pool)
    let app = Router::new()
        .route("/", get(|| async { "Willkommen bei der Auftragsverwaltung von Achtsam Entrümpeln!" }))
        .route("/api/kunden", get(list_kunden))
        .route("/api/kunden", post(add_kunde))
        .route("/api/einsaetze", post(add_einsatz))
        .route("/api/auftraege/:id/upload", post(upload_datei))
        .route("/api/auftraege/:id/dateien", get(list_dateien))
        .with_state(pool);

    // Adresse binden
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server läuft auf http://{}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
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

// Handler: Neuen Arbeitseinsatz dokumentieren
async fn add_einsatz(State(pool): State<SqlitePool>, Json(payload): Json<Einsatz>) -> Result<Json<i64>, AppError> {
    let id = database::create_einsatz(&pool, payload).await?;
    Ok(Json(id))
}

// Handler: Dateien hochladen
async fn upload_datei(
    State(pool): State<SqlitePool>,
    Path(auftrag_id): Path<i64>,
    mut multipart: Multipart,
) -> Result<Json<Vec<i64>>, AppError> {
    let mut ids = Vec::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| AppError::BadRequest(e.to_string()))? {
        let name = field.name().unwrap_or("file").to_string();
        let file_name = field.file_name().unwrap_or("unnamed").to_string();
        let content_type = field.content_type().unwrap_or("application/octet-stream").to_string();
        let data = field.bytes().await.map_err(|e| AppError::BadRequest(e.to_string()))?;

        // Dateipfad erstellen (uploads/auftrag_id_dateiname)
        let safe_file_name = format!("{}_{}", auftrag_id, file_name);
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
            hochgeladen_am: String::new(), // Wird von DB gesetzt
        };

        let id = database::create_datei(&pool, neue_datei).await?;
        ids.push(id);
        println!("Datei hochgeladen: {} (ID: {})", name, id);
    }

    Ok(Json(ids))
}

// Handler: Dateien eines Auftrags auflisten
async fn list_dateien(
    State(pool): State<SqlitePool>,
    Path(auftrag_id): Path<i64>,
) -> Result<Json<Vec<Datei>>, AppError> {
    let dateien = database::get_dateien_for_auftrag(&pool, auftrag_id).await?;
    Ok(Json(dateien))
}
