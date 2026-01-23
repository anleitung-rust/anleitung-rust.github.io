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
{{#include ../codesamples/examples/enum_richtung.rs}}
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
    Spielstand::Laeuft => turtle.write_text("Spiel läuft noch...", 50.0),
    Spielstand::Gewonnen => turtle.write_text("Du hast gewonnen! 🎉", 50.0),
    Spielstand::Verloren => turtle.write_text("Verloren 💀", 50.0),
}
```

## Übung: Turtle-Quadrat mit Farbe

Erstelle ein Programm, das:
- Einen Enum `Farbe` mit `Rot`, `Gruen`, `Blau` hat
- Je nach gewählter Farbe ein Quadrat in dieser Farbe zeichnet

<details>
<summary>Tipp</summary>

```rust
{{#include ../codesamples/examples/enum_farbe.rs}}
```
</details>

## Zusammenfassung

- Enums definieren Auswahlmöglichkeiten
- Definition: `enum Name { Variante1, Variante2, ... }`
- Verwendung: `Name::Variante`
- `match` reagiert auf verschiedene Varianten
- Perfekt für Zustände (Ampel, Spielstand, Richtung)

Du hast jetzt wichtige Grundlagen: Text, Listen, Structs und Enums. Im nächsten Kapitel lernst du, wie der Benutzer selbst Eingaben machen kann!
