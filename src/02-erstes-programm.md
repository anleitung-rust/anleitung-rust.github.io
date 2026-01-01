# Dein erstes Programm

Jetzt wird es spannend! Wir schreiben unser erstes Programm. Die Schildkröte soll ein Quadrat zeichnen.

## Das Quadrat-Programm

Hier ist der Code für unser erstes Programm:

```rust
{{#include ../codesamples/examples/quadrat.rs}}
```

## Was passiert hier?

Lass uns das Programm Schritt für Schritt durchgehen:

### 1. Die erste Zeile

```rust
use turtle_lib::*;
```

Diese Zeile sagt dem Computer: "Ich möchte die Schildkröten-Bibliothek verwenden." Eine **Bibliothek** ist eine Sammlung von fertigen Befehlen, die wir nutzen können. So müssen wir nicht alles selbst programmieren.

### 2. Die Hauptfunktion

```rust
#[turtle_main]
fn main() {
    // Hier steht unser Code
}
```

Jedes Rust-Programm hat eine `main`-Funktion (englisch für "Haupt"). Das ist der Startpunkt des Programms – hier beginnt der Computer mit der Ausführung.

Das `#[turtle_main]` darüber ist eine spezielle Anweisung, die das Fenster für die Schildkröte vorbereitet.

### 3. Die Stiftfarbe setzen

```rust
turtle.set_pen_color(BLUE);
```

Hier sagen wir der Schildkröte: "Verwende die Farbe Blau zum Zeichnen." `turtle` ist unsere Schildkröte und `set_pen_color` ist der Befehl, um die Farbe zu ändern.

### 4. Das Quadrat zeichnen

```rust
for _ in 0..4 {
    turtle.forward(100.0);
    turtle.right(90.0);
}
```

Das ist eine **Schleife** (ein Befehl, der sich wiederholt). Hier passiert Folgendes:
- `for _ in 0..4` bedeutet: "Wiederhole die folgenden Befehle 4 mal"
- `turtle.forward(100.0)` bedeutet: "Gehe 100 Schritte vorwärts"
- `turtle.right(90.0)` bedeutet: "Drehe dich 90 Grad nach rechts"

Ein Quadrat hat vier gleich lange Seiten und vier rechte Winkel (90 Grad). Deshalb wiederholen wir diese Befehle genau 4 mal!

## Was siehst du, wenn du das Programm startest?

Wenn du dieses Programm ausführst, öffnet sich ein Fenster. Darin siehst du:
- Eine kleine Schildkröte (oft als Dreieck dargestellt)
- Die Schildkröte bewegt sich und zeichnet dabei
- Am Ende entsteht ein blaues Quadrat!

## Zusammenfassung

Du hast gerade gelernt:
- Wie ein einfaches Rust-Programm aufgebaut ist
- Was `use`, `fn main()` und `#[turtle_main]` bedeuten
- Wie man die Schildkröte bewegt (`forward` und `right`)
- Wie man eine Farbe setzt (`set_pen_color`)
- Wie man Befehle wiederholt (mit `for`)

Im nächsten Kapitel schauen wir uns die verschiedenen Bewegungs- und Drehbefehle genauer an.
