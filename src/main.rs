mod models;
mod database;
mod error;
mod pdf;

use axum::{
    routing::{get, post, delete},
    extract::{State, Path, Multipart},
    response::Html,
    Json,
    Router,
};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use std::fs;
use crate::models::{Kunde, Auftrag, Einsatz, Datei};
use crate::error::AppError;
use chrono::Local;

#[tokio::main]
async fn main() {
    let pool = database::init_db().await.expect("Failed to initialize database");

    let app = Router::new()
        .route("/api/kunden", get(list_kunden).post(add_kunde))
        .route("/api/kunden/:id", get(get_kunde).post(update_kunde))
        .route("/api/kunden/:id/delete", post(delete_kunde_handler))
        .route("/api/auftraege", get(list_auftraege).post(add_auftrag))
        .route("/api/auftraege/:id", get(get_auftrag).post(update_auftrag))
        .route("/api/auftraege/:id/einsaetze", get(list_einsaetze))
        .route("/api/auftraege/:id/dateien", get(list_dateien))
        .route("/api/auftraege/:id/upload", post(upload_datei))
        .route("/api/auftraege/:id/rechnung", post(create_rechnung))
        .route("/api/auftraege/:id/send_nachweis", post(send_stundennachweis))
        .route("/api/auftraege/:id/generate_doc", post(generate_doc_handler))
        .route("/api/templates", get(list_templates).post(upload_template))
        .route("/api/templates/:name", get(get_template).post(save_template).delete(delete_template))
        .route("/api/einsaetze", post(add_einsatz))
        .route("/api/einsaetze/:id", post(update_einsatz))
        .route("/api/einsaetze/:id/delete", post(delete_einsatz_handler))
        .route("/api/dateien/:id/delete", post(delete_datei_handler))
        .fallback(get(serve_index))
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn serve_index() -> Html<String> {
    Html(std::fs::read_to_string("static/index.html").unwrap_or_default())
}

// Template Handlers
async fn list_templates() -> Result<Json<Vec<String>>, AppError> {
    let mut tpls = Vec::new();
    if let Ok(entries) = std::fs::read_dir("templates") {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".html") { tpls.push(name.to_string()); }
            }
        }
    }
    Ok(Json(tpls))
}

async fn get_template(Path(name): Path<String>) -> Result<String, AppError> {
    fs::read_to_string(format!("templates/{}", name)).map_err(|e| AppError::Internal(e.to_string()))
}

async fn save_template(Path(name): Path<String>, body: String) -> Result<(), AppError> {
    fs::write(format!("templates/{}", name), body).map_err(|e| AppError::Internal(e.to_string()))
}

async fn delete_template(Path(name): Path<String>) -> Result<(), AppError> {
    fs::remove_file(format!("templates/{}", name)).map_err(|e| AppError::Internal(e.to_string()))
}

async fn upload_template(mut multipart: Multipart) -> Result<(), AppError> {
    while let Some(field) = multipart.next_field().await.map_err(|e| AppError::Internal(e.to_string()))? {
        let filename = field.file_name().map(|f| f.to_string());
        if let Some(name) = filename {
            let data = field.bytes().await.map_err(|e| AppError::Internal(e.to_string()))?;
            fs::write(format!("templates/{}", name), data).map_err(|e| AppError::Internal(e.to_string()))?;
        }
    }
    Ok(())
}

// Handlers
async fn list_kunden(State(pool): State<SqlitePool>) -> Result<Json<Vec<Kunde>>, AppError> {
    Ok(Json(database::get_all_kunden(&pool).await?))
}

async fn add_kunde(State(pool): State<SqlitePool>, Json(mut kunde): Json<Kunde>) -> Result<Json<i64>, AppError> {
    kunde.id = 0;
    Ok(Json(database::create_kunde(&pool, kunde).await?))
}

async fn get_kunde(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<Kunde>, AppError> {
    Ok(Json(database::get_kunde_by_id(&pool, id).await?))
}

async fn update_kunde(State(pool): State<SqlitePool>, Path(id): Path<i64>, Json(kunde): Json<Kunde>) -> Result<(), AppError> {
    database::update_kunde(&pool, id, kunde).await?;
    Ok(())
}

async fn delete_kunde_handler(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<(), AppError> {
    database::delete_kunde(&pool, id).await?;
    Ok(())
}

async fn list_auftraege(State(pool): State<SqlitePool>) -> Result<Json<Vec<Auftrag>>, AppError> {
    Ok(Json(database::get_all_auftraege(&pool).await?))
}

async fn add_auftrag(State(pool): State<SqlitePool>, Json(auftrag): Json<Auftrag>) -> Result<Json<i64>, AppError> {
    Ok(Json(database::create_auftrag(&pool, auftrag).await?))
}

async fn get_auftrag(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<Auftrag>, AppError> {
    Ok(Json(database::get_auftrag_by_id(&pool, id).await?))
}

async fn update_auftrag(State(pool): State<SqlitePool>, Path(id): Path<i64>, Json(auftrag): Json<Auftrag>) -> Result<(), AppError> {
    database::update_auftrag(&pool, id, auftrag).await?;
    Ok(())
}

async fn add_einsatz(State(pool): State<SqlitePool>, Json(einsatz): Json<Einsatz>) -> Result<Json<i64>, AppError> {
    Ok(Json(database::create_einsatz(&pool, einsatz).await?))
}

async fn update_einsatz(State(pool): State<SqlitePool>, Path(id): Path<i64>, Json(einsatz): Json<Einsatz>) -> Result<(), AppError> {
    database::update_einsatz(&pool, id, einsatz).await?;
    Ok(())
}

async fn delete_einsatz_handler(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<(), AppError> {
    database::delete_einsatz(&pool, id).await?;
    Ok(())
}

async fn delete_datei_handler(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<(), AppError> {
    database::delete_datei(&pool, id).await?;
    Ok(())
}

async fn list_einsaetze(State(pool): State<SqlitePool>, Path(auftrag_id): Path<i64>) -> Result<Json<Vec<Einsatz>>, AppError> {
    Ok(Json(database::get_einsaetze_for_auftrag(&pool, auftrag_id).await?))
}

async fn list_dateien(State(pool): State<SqlitePool>, Path(auftrag_id): Path<i64>) -> Result<Json<Vec<Datei>>, AppError> {
    Ok(Json(database::get_dateien_for_auftrag(&pool, auftrag_id).await?))
}

async fn upload_datei(State(pool): State<SqlitePool>, Path(auftrag_id): Path<i64>, mut multipart: Multipart) -> Result<Json<Vec<i64>>, AppError> {
    let mut ids = Vec::new();
    let mut category = "SONSTIGES".to_string();
    while let Some(field) = multipart.next_field().await.map_err(|e| AppError::Internal(e.to_string()))? {
        let name = field.name().unwrap_or_default().to_string();
        if name == "kategorie" { category = field.text().await.unwrap_or_else(|_| "SONSTIGES".to_string()); }
        else if name == "file" {
            let filename = field.file_name().unwrap_or("upload").to_string();
            let content_type = field.content_type().unwrap_or("application/octet-stream").to_string();
            let data = field.bytes().await.map_err(|e| AppError::Internal(e.to_string()))?;
            let final_name = format!("{}_{}", auftrag_id, filename);
            let filepath = format!("uploads/{}", final_name);
            fs::write(&filepath, &data).map_err(|e| AppError::Internal(e.to_string()))?;
            if category == "SIGNATUR" {
                let auftrag = database::get_auftrag_by_id(&pool, auftrag_id).await?;
                let kunde = database::get_kunde_by_id(&pool, auftrag.kunde_id).await?;
                let abs_sig = fs::canonicalize(&filepath)
                    .map_err(|e| AppError::Internal(e.to_string()))?
                    .to_str()
                    .ok_or_else(|| AppError::Internal("Konnte Signaturpfad nicht konvertieren".to_string()))?
                    .to_string();
                let (pdf_content, _, _) = pdf::generate_dynamic_pdf("templates/datenschutz.html", &auftrag, &kunde, None, None, None, Some(&abs_sig)).map_err(AppError::PdfError)?;
                let pdf_name = format!("Datenschutz_{}_{}.pdf", auftrag_id, Local::now().format("%Y%m%d"));
                let pdf_path = format!("uploads/{}", pdf_name);
                fs::write(&pdf_path, pdf_content).map_err(|e| AppError::Internal(e.to_string()))?;
                ids.push(database::create_datei(&pool, Datei { id: 0, auftrag_id, dateiname: pdf_name, dateipfad: pdf_path, dateityp: "application/pdf".into(), hochgeladen_am: Local::now().to_rfc3339(), kategorie: "DATENSCHUTZ".into() }).await?);
            }
            ids.push(database::create_datei(&pool, Datei { id: 0, auftrag_id, dateiname: final_name, dateipfad: filepath, dateityp: content_type, hochgeladen_am: Local::now().to_rfc3339(), kategorie: category.clone() }).await?);
        }
    }
    Ok(Json(ids))
}

async fn create_rechnung(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<i64>, AppError> {
    let auftrag = database::get_auftrag_by_id(&pool, id).await?;
    let kunde = database::get_kunde_by_id(&pool, auftrag.kunde_id).await?;
    let einsaetze = database::get_einsaetze_for_auftrag(&pool, id).await?;
    let notizen = database::get_rechnungs_notizen_for_auftrag(&pool, id).await?;
    let re_nr = format!("RE-{}-{}", Local::now().format("%Y"), id);
    
    // Verzeichnis sicherstellen
    if !std::path::Path::new("uploads/rechnungen").exists() {
        fs::create_dir_all("uploads/rechnungen").map_err(|e| AppError::Internal(e.to_string()))?;
    }

    let (pdf_content, netto, brutto) = pdf::generate_dynamic_pdf("templates/rechnung.html", &auftrag, &kunde, Some(&einsaetze), Some(&notizen), Some(&re_nr), None).map_err(AppError::PdfError)?;
    let filename = format!("rechnung_{}.pdf", id);
    let filepath = format!("uploads/rechnungen/{}", filename);
    fs::write(&filepath, pdf_content).map_err(|e| AppError::Internal(e.to_string()))?;
    
    // In Tabelle `dateien` speichern
    database::create_datei(&pool, Datei { 
        id: 0, 
        auftrag_id: id, 
        dateiname: filename.clone(), 
        dateipfad: filepath.clone(), 
        dateityp: "application/pdf".into(), 
        hochgeladen_am: Local::now().to_rfc3339(), 
        kategorie: "RECHNUNG".into() 
    }).await?;

    // In Tabelle `rechnungen` speichern
    let re_id = database::create_rechnung(&pool, crate::models::Rechnung { 
        id: 0, 
        auftrag_id: id, 
        rechnungs_nummer: re_nr, 
        datum: Local::now().format("%Y-%m-%d").to_string(), 
        gesamt_netto: netto, 
        gesamt_brutto: brutto, 
        status: "Offen".into(), 
        pdf_pfad: filepath 
    }).await?;
    
    Ok(Json(re_id))
}

async fn generate_doc_handler(State(pool): State<SqlitePool>, Path(id): Path<i64>, Json(payload): Json<serde_json::Value>) -> Result<Json<i64>, AppError> {
    let template_name = payload["template"].as_str().unwrap_or("vertrag.html");
    let auftrag = database::get_auftrag_by_id(&pool, id).await?;
    let kunde = database::get_kunde_by_id(&pool, auftrag.kunde_id).await?;
    let (pdf_content, _, _) = pdf::generate_dynamic_pdf(&format!("templates/{}", template_name), &auftrag, &kunde, None, None, None, None).map_err(AppError::PdfError)?;
    let filename = format!("{}_{}_{}.pdf", template_name.replace(".html", ""), id, Local::now().format("%Y%m%d"));
    let filepath = format!("uploads/{}", filename);
    fs::write(&filepath, pdf_content).map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(database::create_datei(&pool, Datei { id: 0, auftrag_id: id, dateiname: filename, dateipfad: filepath, dateityp: "application/pdf".into(), hochgeladen_am: Local::now().to_rfc3339(), kategorie: if template_name.contains("vertrag") { "VERTRAG" } else { "DATENSCHUTZ" }.into() }).await?))
}

async fn send_stundennachweis(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<(), AppError> {
    let auftrag = database::get_auftrag_by_id(&pool, id).await?;
    let kunde = database::get_kunde_by_id(&pool, auftrag.kunde_id).await?;
    println!("Simulierter Versand: Stundennachweis für Auftrag {} an {} gesendet.", auftrag.id, kunde.email.unwrap_or_default());
    Ok(())
}
