# Enums und Zustände

Du kennst jetzt Structs für zusammengehörige Daten. Aber was, wenn etwas verschiedene Zustände haben kann? Dafür gibt es **Enums**!

## Was ist ein Enum?

Ein **Enum** (Aufzählung) definiert, welche Möglichkeiten es gibt. Zum Beispiel eine Ampel:

```rust
enum Ampel {
    Rot,
    Gelb,
    Gruen,
}
```

Eine Ampel ist **entweder** rot **oder** gelb **oder** grün – aber niemals zwei gleichzeitig!

## Einen Enum verwenden

So verwendest du einen Enum:

```rust
let farbe = Ampel::Rot;
```

Mit `match` kannst du auf verschiedene Werte reagieren:

```rust
match farbe {
    Ampel::Rot => println!("Stopp!"),
    Ampel::Gelb => println!("Achtung!"),
    Ampel::Gruen => println!("Fahr!"),
}
```

## Übung: Turtle mit Richtung

Probier dieses Beispiel:

```rust
use turtle_lib::*;

enum Richtung {
    Oben,
    Rechts,
    Unten,
    Links,
}

#[turtle_main]
fn main() {
    let richtung = Richtung::Rechts;
    
    match richtung {
        Richtung::Oben => turtle.right(0.0),
        Richtung::Rechts => turtle.right(90.0),
        Richtung::Unten => turtle.right(180.0),
        Richtung::Links => turtle.right(270.0),
    }
    
    turtle.forward(100.0);
}
```

Ändere die Richtung und schau, was passiert!

## Enum für Spielzustand

Enums sind perfekt für Spielzustände:

```rust
enum Spielstand {
    Laeuft,
    Gewonnen,
    Verloren,
}

let zustand = Spielstand::Laeuft;

match zustand {
    Spielstand::Laeuft => println!("Spiel läuft noch..."),
    Spielstand::Gewonnen => println!("Du hast gewonnen! 🎉"),
    Spielstand::Verloren => println!("Verloren 💀"),
}
```

## Übung: Turtle-Quadrat mit Farbe

Erstelle ein Programm, das:
- Einen Enum `Farbe` mit `Rot`, `Gruen`, `Blau` hat
- Je nach gewählter Farbe ein Quadrat in dieser Farbe zeichnet

<details>
<summary>Tipp</summary>

```rust
use turtle_lib::*;

enum Farbe {
    Rot,
    Gruen,
    Blau,
}

#[turtle_main]
fn main() {
    let farbe = Farbe::Gruen;  // Ändere das!
    
    match farbe {
        Farbe::Rot => turtle.set_pen_color(RED),
        Farbe::Gruen => turtle.set_pen_color(GREEN),
        Farbe::Blau => turtle.set_pen_color(BLUE),
    }
    
    // Zeichne Quadrat
    for _ in 0..4 {
        turtle.forward(80.0);
        turtle.right(90.0);
    }
}
```
</details>

## Zusammenfassung

- Enums definieren Auswahlmöglichkeiten
- Definition: `enum Name { Variante1, Variante2, ... }`
- Verwendung: `Name::Variante`
- `match` reagiert auf verschiedene Varianten
- Perfekt für Zustände (Ampel, Spielstand, Richtung)

Du hast jetzt wichtige Grundlagen: Text, Listen, Structs und Enums. Im nächsten Kapitel lernst du, wie der Benutzer selbst Eingaben machen kann!
