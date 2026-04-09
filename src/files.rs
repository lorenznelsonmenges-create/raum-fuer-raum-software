use axum::{
    extract::{State, Path, Multipart},
    Json,
};
use sqlx::SqlitePool;
use std::fs;
use crate::models::Datei;
use crate::error::AppError;
use crate::{database, pdf};
use chrono::Local;

pub async fn upload_datei(
    State(pool): State<SqlitePool>,
    Path(auftrag_id): Path<i64>,
    mut multipart: Multipart,
) -> Result<Json<Vec<i64>>, AppError> {
    let mut ids = Vec::new();
    let mut category = "SONSTIGES".to_string();
    while let Some(field) = multipart.next_field().await.map_err(|e| AppError::Internal(e.to_string()))? {
        let name = field.name().unwrap_or_default().to_string();
        if name == "kategorie" {
            category = field.text().await.unwrap_or_else(|_| "SONSTIGES".to_string());
        } else if name == "file" {
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

                let (pdf_content, _, _) = pdf::generate_dynamic_pdf(
                    "templates/datenschutz.html",
                    &auftrag,
                    &kunde,
                    None,
                    None,
                    None,
                    Some(&abs_sig),
                )?;

                let pdf_name = format!("Datenschutz_{}_{}.pdf", auftrag_id, Local::now().format("%Y%m%d"));
                let pdf_path = format!("uploads/{}", pdf_name);
                fs::write(&pdf_path, pdf_content).map_err(|e| AppError::Internal(e.to_string()))?;

                ids.push(
                    database::create_datei(
                        &pool,
                        Datei {
                            id: 0,
                            auftrag_id,
                            dateiname: pdf_name,
                            dateipfad: pdf_path,
                            dateityp: "application/pdf".into(),
                            hochgeladen_am: Local::now().to_rfc3339(),
                            kategorie: "DATENSCHUTZ".into(),
                        },
                    )
                    .await?,
                );
            }

            ids.push(
                database::create_datei(
                    &pool,
                    Datei {
                        id: 0,
                        auftrag_id,
                        dateiname: final_name,
                        dateipfad: filepath,
                        dateityp: content_type,
                        hochgeladen_am: Local::now().to_rfc3339(),
                        kategorie: category.clone(),
                    },
                )
                .await?,
            );
        }
    }
    Ok(Json(ids))
}
