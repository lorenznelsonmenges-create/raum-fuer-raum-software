#set page(paper: "a4", margin: (x: 2cm, y: 2cm))
#set text(font: "DejaVu Sans", size: 10pt)

#grid(columns: (1fr, 1fr), [ #set text(size: 14pt, weight: "bold", fill: rgb("#526447")); Achtsam Entrümpeln \ #set text(size: 10pt, fill: black); Dienstleistungsvertrag ], [ #align(right)[ #image("static/logo.svg", width: 40pt) ] ])
#v(1cm)
#text(size: 18pt, weight: "bold")[Dienstleistungsvertrag]
#v(0.5cm)
Zwischen *Achtsam Entrümpeln* und *{{kunde_name}}*.
#v(1cm)
*§1 Gegenstand:* \ {{beschreibung}}
#v(0.5cm)
*§2 Vergütung:* \ Die Basis-Pauschale beträgt {{basis_pauschale}} €.
#v(4cm)
#grid(columns: (1fr, 1fr), gutter: 2cm, [ #line(length: 100%) Unterschrift Stefanie Ruf ], [ #line(length: 100%) Unterschrift {{kunde_name}} ])
