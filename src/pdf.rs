use typst_as_lib::TypstTemplate;
use typst::eval::Tracer;
use typst::foundations::Smart;
use typst_pdf;
use crate::models::{Auftrag, Kunde, Einsatz, RechnungsNotiz};
use chrono::Local;

pub fn generate_invoice_pdf(
    auftrag: &Auftrag,
    kunde: &Kunde,
    einsaetze: &[Einsatz],
    notizen: &[RechnungsNotiz],
    rechnungs_nummer: &str,
) -> Result<Vec<u8>, String> {
    let datum = Local::now().format("%d.%m.%Y").to_string();
    
    // Berechnungen
    let mut gesamt_stunden = 0.0;
    let mut gesamt_km = 0.0;
    for e in einsaetze {
        gesamt_stunden += e.stunden;
        gesamt_km += e.kilometer;
    }

    // Preise (Platzhalter - sollten konfigurierbar sein)
    let stundensatz = 45.0; // Beispiel
    let km_satz = 0.50;    // Beispiel
    let basis = auftrag.basis_pauschale.unwrap_or(0.0);
    
    let netto_stunden = gesamt_stunden * stundensatz;
    let netto_km = gesamt_km * km_satz;
    let netto_gesamt = netto_stunden + netto_km + basis;
    let mwst = netto_gesamt * 0.19;
    let brutto_gesamt = netto_gesamt + mwst;

    // Typst Template
    let template_str = r#"
#set page(paper: "a4", margin: (x: 2cm, y: 2cm))
#set text(font: "DejaVu Sans", size: 10pt)

#grid(
  columns: (1fr, 1fr),
  [
    #set text(size: 14pt, weight: "bold")
    Achtsam Entrümpeln \
    #set text(size: 10pt, weight: "regular")
    Stefanie Ruf \
    Musterstraße 1 \
    12345 Musterstadt \
    \
    Steuernummer: 123/456/78901
  ],
  [
    #align(right)[
      #image("static/logo.svg", width: 40pt)
    ]
  ]
)

#v(2cm)

#grid(
  columns: (1fr),
  [
    *Rechnungsempfänger:* \
    {{KUNDE_NAME}} \
    {{KUNDE_STRASSE}} {{KUNDE_NR}} \
    {{KUNDE_PLZ}} {{KUNDE_ORT}}
  ]
)

#v(1cm)

#grid(
  columns: (1fr, 1fr),
  [
    *Rechnungsnummer:* {{RE_NR}}
  ],
  [
    #align(right)[
      *Datum:* {{DATUM}}
    ]
  ]
)

#v(1cm)

#table(
  columns: (1fr, 100pt, 80pt, 80pt),
  inset: 8pt,
  align: (left, right, right, right),
  [*Leistung*], [*Menge*], [*Einzelpreis*], [*Gesamt*],
  {{POSTEN}}
)

#v(0.5cm)

#align(right)[
  #grid(
    columns: (100pt, 80pt),
    row-gutter: 8pt,
    [Netto Gesamt:], [{{NETTO}} €],
    [USt. 19%:], [{{MWST}} €],
    [*Brutto Gesamt:*], [*{{BRUTTO}} €*]
  )
]

#v(1cm)
#line(length: 100%, stroke: 0.5pt + gray)
#v(0.5cm)

*Hinweise:* \
{{NOTIZEN}}
\
Das Leistungsdatum entspricht dem Rechnungsdatum, sofern nicht anders angegeben.
Bitte überweisen Sie den Betrag innerhalb von 14 Tagen auf das unten stehende Konto.
"#;

    // Platzhalter ersetzen
    let mut posten = String::new();
    if basis > 0.0 {
        posten.push_str(&format!("[Basis-Pauschale], [1], [{:.2} €], [{:.2} €],\n", basis, basis));
    }
    if gesamt_stunden > 0.0 {
        posten.push_str(&format!("[Arbeitszeit (Stunden)], [{:.2}], [{:.2} €], [{:.2} €],\n", gesamt_stunden, stundensatz, netto_stunden));
    }
    if gesamt_km > 0.0 {
        posten.push_str(&format!("[Fahrtkosten (Kilometer)], [{:.2}], [{:.2} €], [{:.2} €],\n", gesamt_km, km_satz, netto_km));
    }

    let rechnungs_notizen = notizen.iter()
        .filter(|n| n.auf_rechnung)
        .map(|n| format!("- {}", n.text))
        .collect::<Vec<_>>()
        .join("\n");

    let filled_template = template_str
        .replace("{{KUNDE_NAME}}", &format!("{} {}", kunde.vorname, kunde.nachname))
        .replace("{{KUNDE_STRASSE}}", &kunde.strasse.clone().unwrap_or_default())
        .replace("{{KUNDE_NR}}", &kunde.hausnummer.clone().unwrap_or_default())
        .replace("{{KUNDE_PLZ}}", &kunde.plz.clone().unwrap_or_default())
        .replace("{{KUNDE_ORT}}", &kunde.ort.clone().unwrap_or_default())
        .replace("{{RE_NR}}", rechnungs_nummer)
        .replace("{{DATUM}}", &datum)
        .replace("{{POSTEN}}", &posten)
        .replace("{{NETTO}}", &format!("{:.2}", netto_gesamt))
        .replace("{{MWST}}", &format!("{:.2}", mwst))
        .replace("{{BRUTTO}}", &format!("{:.2}", brutto_gesamt))
        .replace("{{NOTIZEN}}", if rechnungs_notizen.is_empty() { "Keine besonderen Hinweise." } else { &rechnungs_notizen });

    // Typst Compiler aufrufen
    let mut tracer = Tracer::new();
    let template = TypstTemplate::new(vec![], filled_template);
    let doc = template.compile(&mut tracer)
        .map_err(|e| format!("Fehler bei PDF-Generierung: {:?}", e))?;
    let pdf = typst_pdf::pdf(&doc, Smart::Auto, None);

    Ok(pdf)
}
