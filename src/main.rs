use achtsam_entruempeln_software::{models, database, pdf, files};
use achtsam_entruempeln_software::models::{Kunde, Auftrag, Einsatz, Datei, DashboardStats, Settings, LoginRequest, User};
use achtsam_entruempeln_software::error::AppError;

use axum::{
    routing::{get, post},
    extract::{State, Path, Multipart, FromRequestParts, Request},
    http::{request::Parts, StatusCode},
    middleware::{self, Next},
    response::{Html, IntoResponse, Redirect},
    Json,
    Router,
    async_trait,
};
use tower_http::services::ServeDir;
use sqlx::SqlitePool;
use std::net::SocketAddr;
use std::fs;
use chrono::Local;
use tower_sessions::{Session, SessionManagerLayer, Expiry};
use tower_sessions_sqlx_store::SqliteStore;
use bcrypt::verify;

#[tokio::main]
async fn main() {
    let pool = database::init_db().await.expect("Failed to initialize database");

    // Session Store initialisieren
    let session_store = SqliteStore::new(pool.clone());
    session_store.migrate().await.expect("Failed to run session migrations");

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(true) // Muss auf true sein bei HTTPS
        .with_expiry(Expiry::OnInactivity(time::Duration::hours(24)));

    // WICHTIG: Axum-Router-Reihenfolge
    // Statische Pfade muessen in einem Router definiert werden, 
    // bevor parametrisierte Pfade dazukommen, die sie sonst "verschlucken".
    
    let public_routes = Router::new()
        .route("/login", post(login_handler))
        .route("/logout", get(logout_handler));

    let protected_routes = Router::new()
        .route("/check_auth", get(check_auth))
        // 1. Statische API-Pfade (keine Parameter)
        .route("/stats", get(get_stats))
        .route("/settings", get(get_settings).post(update_settings))
        .route("/templates", get(list_templates).post(upload_template))
        .route("/kunden", get(list_kunden).post(add_kunde))
        .route("/auftraege", get(list_auftraege).post(add_auftrag))
        .route("/einsaetze", post(add_einsatz))
        
        // 2. Parametrisierte API-Pfade
        .route("/templates/:name", get(get_template).post(save_template).delete(delete_template))
        .route("/kunden/:id", get(get_kunde).post(update_kunde))
        .route("/kunden/:id/delete", post(delete_kunde_handler))
        .route("/auftraege/:id", get(get_auftrag).post(update_auftrag).delete(delete_auftrag_handler))
        .route("/auftraege/:id/einsaetze", get(list_einsaetze))
        .route("/auftraege/:id/dateien", get(list_dateien))
        .route("/auftraege/:id/upload", post(files::upload_datei))
        .route("/auftraege/:id/rechnung", post(create_rechnung))
        .route("/auftraege/:id/send_nachweis", post(send_stundennachweis))
        .route("/auftraege/:id/generate_doc", post(generate_doc_handler))
        .route("/einsaetze/:id", post(update_einsatz))
        .route("/einsaetze/:id/delete", post(delete_einsatz_handler))
        .route("/dateien/:id/delete", post(delete_datei_handler))
        .route_layer(middleware::from_fn(auth_middleware));

    let app = Router::new()
        .nest("/api", public_routes.merge(protected_routes))
        .nest_service("/uploads", ServeDir::new("./uploads"))
        .nest_service("/static", ServeDir::new("./static"))
        .fallback(get(serve_index))
        .layer(session_layer)
        .with_state(pool);

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub struct AuthUser(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|_| AppError::AuthError("Session error".into()))?;

        let user: Option<User> = session
            .get("user")
            .await
            .map_err(|_| AppError::AuthError("Session error".into()))?;

        if let Some(user) = user {
            Ok(AuthUser(user))
        } else {
            Err(AppError::AuthError("Not logged in".into()))
        }
    }
}

async fn serve_index() -> Html<String> {
    Html(std::fs::read_to_string("static/index.html").unwrap_or_default())
}

async fn login_handler(
    State(pool): State<SqlitePool>,
    session: Session,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<User>, AppError> {
    let username = payload.username.trim();
    let password = payload.password.trim();

    println!("DEBUG: Login-Versuch für Benutzer: '{}'", username);
    
    let user = match database::get_user_by_username(&pool, username).await {
        Ok(u) => u,
        Err(e) => {
            eprintln!("DEBUG: Benutzer '{}' nicht gefunden oder DB-Fehler: {:?}", username, e);
            return Err(AppError::AuthError("Ungültiger Benutzername oder Passwort".into()));
        }
    };

    match verify(password, &user.password_hash) {
        Ok(true) => {
            println!("DEBUG: Login erfolgreich für Benutzer: '{}'", username);
            session.insert("user", user.clone()).await.map_err(|e| {
                eprintln!("DEBUG: Session-Insert Fehler: {:?}", e);
                AppError::Internal(e.to_string())
            })?;
            Ok(Json(user))
        },
        Ok(false) => {
            eprintln!("DEBUG: Passwort falsch für Benutzer: '{}'", username);
            Err(AppError::AuthError("Ungültiger Benutzername oder Passwort".into()))
        },
        Err(e) => {
            eprintln!("DEBUG: Bcrypt-Verifikationsfehler: {:?}", e);
            Err(AppError::Internal("Bcrypt error".into()))
        }
    }
}

async fn logout_handler(session: Session) -> impl IntoResponse {
    session.clear().await;
    Redirect::to("/static/login.html")
}

async fn check_auth(_auth: AuthUser) -> impl IntoResponse {
    StatusCode::OK
}

async fn auth_middleware(
    session: Session,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    let user: Option<User> = session
        .get("user")
        .await
        .map_err(|_| AppError::AuthError("Session-Fehler".into()))?;

    if user.is_some() {
        Ok(next.run(request).await)
    } else {
        Err(AppError::AuthError("Nicht angemeldet".into()))
    }
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
async fn get_stats(State(pool): State<SqlitePool>) -> Result<Json<DashboardStats>, AppError> {
    Ok(Json(database::get_dashboard_stats(&pool).await?))
}

async fn get_settings(State(pool): State<SqlitePool>) -> Result<Json<Settings>, AppError> {
    Ok(Json(database::get_settings(&pool).await?))
}

async fn update_settings(State(pool): State<SqlitePool>, Json(settings): Json<Settings>) -> Result<(), AppError> {
    database::update_settings(&pool, settings).await?;
    Ok(())
}

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

async fn add_auftrag(State(pool): State<SqlitePool>, Json(mut auftrag): Json<Auftrag>) -> Result<Json<i64>, AppError> {
    if auftrag.stundensatz == 0.0 || auftrag.kilometer_satz == 0.0 {
        let settings = database::get_settings(&pool).await?;
        if auftrag.stundensatz == 0.0 {
            auftrag.stundensatz = settings.stundensatz;
        }
        if auftrag.kilometer_satz == 0.0 {
            auftrag.kilometer_satz = settings.kilometer_satz;
        }
    }
    Ok(Json(database::create_auftrag(&pool, auftrag).await?))
}

async fn get_auftrag(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<Auftrag>, AppError> {
    Ok(Json(database::get_auftrag_by_id(&pool, id).await?))
}

async fn update_auftrag(State(pool): State<SqlitePool>, Path(id): Path<i64>, Json(auftrag): Json<Auftrag>) -> Result<(), AppError> {
    database::update_auftrag(&pool, id, auftrag).await?;
    Ok(())
}

async fn delete_auftrag_handler(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<(), AppError> {
    println!("Versuche Auftrag zu löschen: ID={}", id);
    match database::delete_auftrag(&pool, id).await {
        Ok(_) => {
            println!("Auftrag ID={} erfolgreich gelöscht", id);
            Ok(())
        },
        Err(e) => {
            eprintln!("Fehler beim Löschen von Auftrag ID={}: {:?}", id, e);
            Err(e.into())
        }
    }
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

async fn create_rechnung(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Json<i64>, AppError> {
    let auftrag = database::get_auftrag_by_id(&pool, id).await?;
    let kunde = database::get_kunde_by_id(&pool, auftrag.kunde_id).await?;
    let einsaetze = database::get_einsaetze_for_auftrag(&pool, id).await?;
    let notizen = database::get_rechnungs_notizen_for_auftrag(&pool, id).await?;
    
    let total_count = database::get_total_rechnung_count(&pool).await?;
    let re_nr = format!("R{:06}", total_count);
    
    // Verzeichnis sicherstellen
    if !std::path::Path::new("uploads/rechnungen").exists() {
        fs::create_dir_all("uploads/rechnungen").map_err(|e| AppError::Internal(e.to_string()))?;
    }

    let (pdf_content, netto, brutto) = pdf::generate_dynamic_pdf("templates/rechnung.html", &auftrag, &kunde, Some(&einsaetze), Some(&notizen), Some(&re_nr), None)?;
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let filename = format!("rechnung_{}_{}.pdf", id, timestamp);
    let filepath = format!("uploads/rechnungen/{}", filename);
    fs::write(&filepath, pdf_content).map_err(|e| AppError::Internal(e.to_string()))?;
    
    // In Tabelle `dateien` speichern
    database::create_datei(&pool, Datei { 
        id: 0, 
        auftrag_id: id, 
        dateiname: re_nr.clone(), 
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
    let einsaetze = database::get_einsaetze_for_auftrag(&pool, id).await?;
    let notizen = database::get_rechnungs_notizen_for_auftrag(&pool, id).await?;

    let (pdf_content, _, _) = pdf::generate_dynamic_pdf(
        &format!("templates/{}", template_name), 
        &auftrag, 
        &kunde, 
        Some(&einsaetze), 
        Some(&notizen), 
        None, 
        None
    )?;

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
