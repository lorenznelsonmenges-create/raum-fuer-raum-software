#set page(paper: "a4", margin: (x: 2cm, y: 2cm))
#set text(font: "DejaVu Sans", size: 10pt)

#grid(columns: (1fr, 1fr), [ #set text(size: 14pt, weight: "bold", fill: rgb("#526447")); Achtsam Entrümpeln \ #set text(size: 10pt, weight: "regular", fill: black); Stefanie Ruf ], [ #align(right)[ #image("static/logo.svg", width: 40pt) ] ])
#v(1cm)
#align(center)[#text(size: 16pt, weight: "bold", fill: rgb("#526447"))[Einwilligung zur Datenverarbeitung]]
#v(0.5cm)
Hiermit willige ich, *{{kunde_name}}*, wohnhaft in {{kunde_adresse}}, darin ein, dass *Achtsam Entrümpeln* meine Daten verarbeitet.
#v(1cm)
#grid(columns: (1fr, 1fr), gutter: 2cm, [ Musterstadt, den {{datum}} \ #v(1cm) #line(length: 100%, stroke: 0.5pt) Ort, Datum ], [ #v(0.4cm) #line(length: 100%, stroke: 0.5pt) Unterschrift Kunde ])
