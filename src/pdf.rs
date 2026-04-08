use handlebars::Handlebars;
use serde_json::json;
use headless_chrome::{Browser, LaunchOptions};
use crate::models::{Auftrag, Kunde, Einsatz, RechnungNotiz};
use chrono::Local;
use std::fs;

use base64::{Engine as _, engine::general_purpose};

pub fn generate_dynamic_pdf(
    template_path: &str,
    auftrag: &Auftrag,
    kunde: &Kunde,
    einsaetze: Option<&[Einsatz]>,
    _notizen: Option<&[RechnungNotiz]>,
    rechnungs_nummer: Option<&str>,
    signature_path: Option<&str>,
) -> Result<(Vec<u8>, f64, f64), String> {
    let mut hb = Handlebars::new();
    // Helper für Preisberechnung oder Bedingungen
    hb.register_helper("eq", Box::new(|h: &handlebars::Helper, _: &Handlebars, _: &handlebars::Context, _: &mut handlebars::RenderContext, out: &mut dyn handlebars::Output| -> handlebars::HelperResult {
        let first = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
        let second = h.param(1).and_then(|v| v.value().as_str()).unwrap_or("");
        if first == second {
            out.write("true")?;
        }
        Ok(())
    }));

    let template_content = fs::read_to_string(template_path)
        .map_err(|e| format!("Konnte Vorlage nicht lesen: {}", e))?;

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
        "gesamt_netto": format!("{:.2}", netto_total),
        "mwst": format!("{:.2}", mwst),
        "gesamt_brutto": format!("{:.2}", brutto_total),
        "signatur_pfad": signature_path.unwrap_or("")
    });

    let html = hb.render_template(&template_content, &data)
        .map_err(|e| format!("Fehler beim Render der Vorlage: {}", e))?;

    // PDF Generierung via Headless Chrome
    let pdf_bytes = print_html_to_pdf(html)?;
    Ok((pdf_bytes, netto_total, brutto_total))
}

fn print_html_to_pdf(html: String) -> Result<Vec<u8>, String> {
    let options = LaunchOptions::default_builder()
        .headless(true)
        .build()
        .map_err(|e| e.to_string())?;
    
    let browser = Browser::new(options).map_err(|e| e.to_string())?;
    let tab = browser.new_tab().map_err(|e| e.to_string())?;

    // Wir laden das HTML direkt als Data-URL
    let b64_html = general_purpose::STANDARD.encode(html);
    let data_url = format!("data:text/html;base64,{}", b64_html);
    
    tab.navigate_to(&data_url).map_err(|e| e.to_string())?;
    tab.wait_until_navigated().map_err(|e| e.to_string())?;

    let pdf_options = None; // Default A4
    let pdf_data = tab.print_to_pdf(pdf_options).map_err(|e| e.to_string())?;

    Ok(pdf_data)
}
