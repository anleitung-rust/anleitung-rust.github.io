# Strings und Text

In den bisherigen Kapiteln hast du mit Zahlen und Grafiken gearbeitet. Jetzt lernst du, wie man mit **Text** arbeitet. Text brauchst du für Namen, Nachrichten oder Wörter in Spielen.

## Was ist ein String?

Ein **String** ist einfach Text – eine Folge von Buchstaben und Zeichen.

```rust
let name = "Anna";
let wort = "Schildkröte";
```

Text steht immer in Anführungszeichen `"..."`.

## Text mit der Turtle anzeigen

Probier das aus:

```rust
use turtle_lib::*;

#[turtle_main]
fn main() {
    let name = "Max";
    println!("Hallo, {}!", name);
    
    turtle.forward(100.0);
}
```

Das `{}` ist ein Platzhalter für den Text aus der Variable `name`.

## Übung: Dein Name

Ändere den Code so, dass dein eigener Name ausgegeben wird!

## Text zusammenfügen

Du kannst Text kombinieren:

```rust
let vorname = "Anna";
let nachname = "Müller";
let ganzer_name = format!("{} {}", vorname, nachname);
println!("{}", ganzer_name);  // Gibt aus: Anna Müller
```

## Text vergleichen

Um zu prüfen, ob zwei Texte gleich sind, verwendest du `==`:

```rust
let wort1 = "Katze";
let wort2 = "Katze";

if wort1 == wort2 {
    println!("Die Wörter sind gleich!");
}
```

**Wichtig:** `"Katze"` ist nicht gleich `"katze"` – Groß-/Kleinschreibung zählt!

## Übung: Turtle mit deinem Namen

Erstelle ein Programm, das:
1. Deinen Namen in einer Variable speichert
2. "Hallo, [dein Name]!" ausgibt
3. Ein Quadrat mit der Turtle zeichnet

<details>
<summary>Tipp</summary>

```rust
use turtle_lib::*;

#[turtle_main]
fn main() {
    let name = "...";  // Dein Name hier
    println!("Hallo, {}!", name);
    
    // Zeichne ein Quadrat
    for _ in 0..4 {
        turtle.forward(80.0);
        turtle.right(90.0);
    }
}
```
</details>

## Zusammenfassung

- Text steht in Anführungszeichen: `"Hallo"`
- Mit `{}` kannst du Text einfügen
- Mit `==` vergleichst du Text
- Text kann in Variablen gespeichert werden

Im nächsten Kapitel lernst du, wie man mehrere Werte in Listen speichert!
