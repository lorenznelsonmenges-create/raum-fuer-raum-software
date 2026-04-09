use handlebars::{Handlebars, Renderable};
use serde_json::json;
use headless_chrome::{Browser, LaunchOptions};
use crate::models::{Auftrag, Kunde, Einsatz, RechnungNotiz};
use crate::error::AppError;
use chrono::Local;
use std::fs;

use base64::{Engine as _, engine::general_purpose};

pub fn generate_dynamic_pdf(
    template_path: &str,
    auftrag: &Auftrag,
    kunde: &Kunde,
    einsaetze: Option<&[Einsatz]>,
    notizen: Option<&[RechnungNotiz]>,
    rechnungs_nummer: Option<&str>,
    signature_path: Option<&str>,
) -> Result<(Vec<u8>, f64, f64), AppError> {
    let mut hb = Handlebars::new();
    
    // Helper für Preisberechnung oder Bedingungen
    struct EqHelper;
    impl handlebars::HelperDef for EqHelper {
        fn call<'reg: 'rc, 'rc>(
            &self,
            h: &handlebars::Helper<'rc>,
            r: &'reg Handlebars<'reg>,
            ctx: &'rc handlebars::Context,
            rc: &mut handlebars::RenderContext<'reg, 'rc>,
            out: &mut dyn handlebars::Output,
        ) -> handlebars::HelperResult {
            let first = h.param(0).map(|v| v.value());
            let second = h.param(1).map(|v| v.value());
            let is_eq = first == second;

            if is_eq {
                if let Some(template) = h.template() {
                    template.render(r, ctx, rc, out)
                } else {
                    out.write("true").map_err(handlebars::RenderError::from)?;
                    Ok(())
                }
            } else if let Some(inverse) = h.inverse() {
                inverse.render(r, ctx, rc, out)
            } else {
                Ok(())
            }
        }
    }
    hb.register_helper("eq", Box::new(EqHelper));

    let template_content = fs::read_to_string(template_path)
        .map_err(|e| AppError::Internal(format!("Konnte Vorlage '{}' nicht lesen: {}", template_path, e)))?;

    // Berechnungen für Rechnung (falls vorhanden)
    let mut gesamt_netto_einsaetze = 0.0;
    let mut einsaetze_data = Vec::new();
    
    if let Some(ee) = einsaetze {
        for e in ee {
            let typ_upper = e.typ.to_uppercase();
            let ist_arbeit = typ_upper == "ARBEIT";
            let einzelpreis = if ist_arbeit { auftrag.stundensatz } else { auftrag.kilometer_satz };
            let summe = if ist_arbeit { e.stunden * einzelpreis } else { e.kilometer * einzelpreis };
            
            gesamt_netto_einsaetze += summe;
            einsaetze_data.push(json!({
                "datum": e.datum,
                "typ": e.typ,
                "stunden": e.stunden,
                "kilometer": e.kilometer,
                "notiz": e.notiz,
                "einzelpreis": format!("{:.2}", einzelpreis),
                "zeilen_summe": format!("{:.2}", summe)
            }));
        }
    }

    let mut notizen_data = Vec::new();
    if let Some(nn) = notizen {
        for n in nn {
            if n.auf_rechnung {
                notizen_data.push(json!({
                    "text": n.text
                }));
            }
        }
    }
    
    let basis = auftrag.basis_pauschale.unwrap_or(0.0);
    let netto_total = gesamt_netto_einsaetze + basis;
    let mwst = netto_total * 0.19;
    let brutto_total = netto_total + mwst;

    let data = json!({
        "kunde_name": format!("{} {}", kunde.vorname, kunde.nachname),
        "kunde_vorname": kunde.vorname,
        "kunde_nachname": kunde.nachname,
        "kunde_strasse": kunde.strasse.clone().unwrap_or_default(),
        "kunde_hausnummer": kunde.hausnummer.clone().unwrap_or_default(),
        "kunde_plz": kunde.plz.clone().unwrap_or_default(),
        "kunde_ort": kunde.ort.clone().unwrap_or_default(),
        "kunde_id": format!("K{:06}", kunde.id),
        "auftrag_id": format!("A{:06}", auftrag.id),
        "auftrag_beschreibung": auftrag.beschreibung,
        "datum_heute": Local::now().format("%d.%m.%Y").to_string(),
        "basis_pauschale": format!("{:.2}", basis),
        "rechnungs_nummer": rechnungs_nummer.unwrap_or(""),
        "einsaetze": einsaetze_data,
        "rechnungs_notizen": notizen_data,
        "gesamt_netto": format!("{:.2}", netto_total),
        "mwst": format!("{:.2}", mwst),
        "gesamt_brutto": format!("{:.2}", brutto_total),
        "signatur_pfad": signature_path.unwrap_or("")
    });

    let html = hb.render_template(&template_content, &data)
        .map_err(|e| AppError::PdfError(format!("Fehler beim Render der Vorlage: {}", e)))?;

    // PDF Generierung via Headless Chrome
    let pdf_bytes = print_html_to_pdf(html)?;
    Ok((pdf_bytes, netto_total, brutto_total))
}

fn print_html_to_pdf(html: String) -> Result<Vec<u8>, AppError> {
    let options = LaunchOptions::default_builder()
        .headless(true)
        .args(vec![
            std::ffi::OsStr::new("--no-sandbox"),
            std::ffi::OsStr::new("--disable-setuid-sandbox"),
            std::ffi::OsStr::new("--disable-dev-shm-usage"),
        ])
        .build()
        .map_err(|e| {
            let err_msg = format!("Headless Chrome LaunchOptions Fehler: {}", e);
            eprintln!("CRITICAL: {}", err_msg);
            AppError::PdfError(err_msg)
        })?;
    
    let browser = Browser::new(options).map_err(|e| {
        let err_msg = format!("Headless Chrome Browser::new Fehler: {}. Ist Chrome/Chromium installiert?", e);
        eprintln!("CRITICAL: {}", err_msg);
        AppError::PdfError(err_msg)
    })?;

    let tab = browser.new_tab().map_err(|e| {
        let err_msg = format!("Headless Chrome browser.new_tab Fehler: {}", e);
        eprintln!("CRITICAL: {}", err_msg);
        AppError::PdfError(err_msg)
    })?;

    // Wir laden das HTML direkt als Data-URL
    let b64_html = general_purpose::STANDARD.encode(html);
    let data_url = format!("data:text/html;base64,{}", b64_html);
    
    tab.navigate_to(&data_url).map_err(|e| {
        let err_msg = format!("Headless Chrome tab.navigate_to Fehler: {}", e);
        eprintln!("CRITICAL: {}", err_msg);
        AppError::PdfError(err_msg)
    })?;

    tab.wait_until_navigated().map_err(|e| {
        let err_msg = format!("Headless Chrome tab.wait_until_navigated Fehler: {}", e);
        eprintln!("CRITICAL: {}", err_msg);
        AppError::PdfError(err_msg)
    })?;

    let pdf_options = None; // Default A4
    let pdf_data = tab.print_to_pdf(pdf_options).map_err(|e| {
        let err_msg = format!("Headless Chrome tab.print_to_pdf Fehler: {}", e);
        eprintln!("CRITICAL: {}", err_msg);
        AppError::PdfError(err_msg)
    })?;

    Ok(pdf_data)
}
