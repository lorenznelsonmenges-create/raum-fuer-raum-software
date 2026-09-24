use wendepunkt_software::pdf::generate_dynamic_pdf;
use wendepunkt_software::models::{Auftrag, Kunde, AuftragStatus};

#[tokio::test]
async fn test_rechnung_pdf_generation() {
    let auftrag = Auftrag {
        id: 1,
        kunde_id: 1,
        status: AuftragStatus::AnfrageLaeuft,
        beschreibung: "Test Auftrag".to_string(),
        basis_pauschale: Some(100.0),
        stundensatz: 45.0,
        kilometer_satz: 0.5,
        notizen: "".to_string(),
        ..Default::default()
    };
    let kunde = Kunde {
        id: 1,
        vorname: "Max".to_string(),
        nachname: "Mustermann".to_string(),
        strasse: Some("Teststr.".to_string()),
        hausnummer: Some("1".to_string()),
        plz: Some("12345".to_string()),
        ort: Some("Teststadt".to_string()),
        email: Some("max@mustermann.de".to_string()),
        telefon: Some("0123456789".to_string()),
        notizen: None,
    };

    let result = generate_dynamic_pdf(
        "templates/rechnung.html",
        &auftrag,
        &kunde,
        None,
        None,
        Some("RE-2024-001"),
        None,
        Some("Stefanie Ruf"),
    );

    match result {
        Ok((pdf, netto, brutto)) => {
            println!("PDF generated: {} bytes, netto: {}, brutto: {}", pdf.len(), netto, brutto);
            assert!(pdf.len() > 0);
        },
        Err(e) => panic!("PDF generation failed: {:?}", e),
    }
}
