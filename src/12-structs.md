# Strukturen (Structs)

Du kannst jetzt einzelne Werte und Listen speichern. Aber was, wenn zusammengehörige Dinge eine Einheit bilden sollen? Dafür gibt es **Structs**!

## Was ist ein Struct?

Ein **Struct** ist wie eine Karteikarte mit mehreren Feldern. Zum Beispiel für eine Person:

```
┌─────────────────┐
│ Name:   Anna    │
│ Alter:  12      │
│ Stadt:  Berlin  │
└─────────────────┘
```

## Einen Struct definieren

So erstellst du einen eigenen Datentyp:

```rust
struct Person {
    name: String,
    alter: u32,
}
```

- `struct` bedeutet "Struktur"
- `Person` ist der Name (groß geschrieben!)
- Die Felder stehen in geschweiften Klammern

## Einen Struct verwenden

So erstellst du eine Person:

```rust
let person = Person {
    name: String::from("Max"),
    alter: 12,
};

println!("Name: {}", person.name);
println!("Alter: {}", person.alter);
```

Mit dem Punkt `.` greifst du auf die Felder zu.

## Übung: Rechteck mit Struct

Probier dieses Beispiel aus:

```rust
use turtle_lib::*;

struct Rechteck {
    breite: f32,
    hoehe: f32,
}

#[turtle_main]
fn main() {
    let rechteck = Rechteck {
        breite: 100.0,
        hoehe: 60.0,
    };
    
    // Zeichne das Rechteck
    for _ in 0..2 {
        turtle.forward(rechteck.breite);
        turtle.right(90.0);
        turtle.forward(rechteck.hoehe);
        turtle.right(90.0);
    }
}
```

Ändere die Breite und Höhe und schau, was passiert!

## Mehrere Structs

Du kannst mehrere Rechtecke erstellen:

```rust
let rechteck1 = Rechteck { breite: 80.0, hoehe: 40.0 };
let rechteck2 = Rechteck { breite: 120.0, hoehe: 60.0 };
```

## Übung: Drei Rechtecke

Erstelle ein Programm, das:
- Drei verschiedene Rechtecke definiert
- Alle drei nacheinander zeichnet
- Die Turtle dazwischen bewegt

<details>
<summary>Lösung (nur anschauen, wenn du nicht weiterkommst!)</summary>

```rust
use turtle_lib::*;

struct Rechteck {
    breite: f32,
    hoehe: f32,
}

#[turtle_main]
fn main() {
    let rechtecke = vec![
        Rechteck { breite: 50.0, hoehe: 30.0 },
        Rechteck { breite: 70.0, hoehe: 40.0 },
        Rechteck { breite: 90.0, hoehe: 50.0 },
    ];
    
    for rechteck in &rechtecke {
        // Zeichne Rechteck
        for _ in 0..2 {
            turtle.forward(rechteck.breite);
            turtle.right(90.0);
            turtle.forward(rechteck.hoehe);
            turtle.right(90.0);
        }
        
        // Bewege nach rechts
        turtle.pen_up();
        turtle.forward(rechteck.breite + 20.0);
        turtle.pen_down();
    }
}
```
</details>

## Zusammenfassung

- Structs gruppieren zusammengehörige Daten
- Definition: `struct Name { feld: Typ, ... }`
- Verwendung: `instanz.feld`
- Structs machen Code übersichtlicher

Im nächsten Kapitel lernst du **Enums** – für Dinge, die verschiedene Zustände haben können!
