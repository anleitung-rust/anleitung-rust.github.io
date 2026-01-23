# Sammlungen: Listen

Im letzten Kapitel hast du gelernt, wie man einen Text speichert. Aber was, wenn du mehrere Wörter oder Zahlen speichern möchtest? Dafür gibt es **Listen**!

## Was ist eine Liste?

Eine **Liste** ist wie eine Reihe von Schubladen, in denen du mehrere Werte aufbewahren kannst.

In Rust heißt eine Liste **Vec** (kurz für "Vektor"):

```rust
let zahlen = vec![10, 20, 30, 40];
```

## Eine Liste erstellen

So erstellst du eine Liste:

```rust
let farben = vec!["Rot", "Grün", "Blau"];
```

- `vec!` erstellt eine neue Liste
- Die Werte stehen in eckigen Klammern `[]`
- Die Werte werden mit Komma getrennt

## Auf Elemente zugreifen

Jedes Element in der Liste hat eine Nummer (Index). Die Zählung beginnt bei 0!

```rust
let tiere = vec!["Hund", "Katze", "Vogel"];

println!("{}", tiere[0]);  // Gibt aus: Hund
println!("{}", tiere[1]);  // Gibt aus: Katze
println!("{}", tiere[2]);  // Gibt aus: Vogel
```

## Übung: Turtle mit Farben

Probier dieses Programm aus:

```rust
use turtle_lib::*;

#[turtle_main]
fn main() {
    let farben = vec![RED, GREEN, BLUE];
    
    for farbe in &farben {
        turtle.set_pen_color(*farbe);
        turtle.forward(50.0);
        turtle.right(120.0);
    }
}
```

Was zeichnet das Programm? Ändere die Farben und probiere es aus!

## Durch eine Liste gehen

Mit einer `for`-Schleife kannst du alle Elemente durchgehen:

```rust
let zahlen = vec![5, 10, 15];

for zahl in &zahlen {
    println!("{}", zahl);
}
```

Das gibt aus:
```
5
10
15
```

## Übung: Mehrere Quadrate

Erstelle ein Programm, das:
- Eine Liste mit Größen hat: `vec![30.0, 50.0, 70.0]`
- Für jede Größe ein Quadrat zeichnet
- Die Turtle zwischen den Quadraten bewegt

<details>
<summary>Tipp</summary>

```rust
use turtle_lib::*;

#[turtle_main]
fn main() {
    let groessen = vec![30.0, 50.0, 70.0];
    
    for groesse in &groessen {
        // Zeichne ein Quadrat mit dieser Größe
        for _ in 0..4 {
            turtle.forward(*groesse);
            turtle.right(90.0);
        }
        // Bewege dich nach rechts
        turtle.pen_up();
        turtle.forward(groesse + 20.0);
        turtle.pen_down();
    }
}
```
</details>

## Zusammenfassung

- Listen speichern mehrere Werte: `vec![1, 2, 3]`
- Index beginnt bei 0: `liste[0]` ist das erste Element
- `for element in &liste` geht durch alle Elemente
- Listen sind praktisch für mehrere gleiche Dinge

Im nächsten Kapitel lernst du **Structs** – damit kannst du zusammengehörige Daten gruppieren!
