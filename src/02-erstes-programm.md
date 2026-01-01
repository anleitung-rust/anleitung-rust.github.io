# Dein erstes Programm

Jetzt wird es spannend! Wir schreiben unser erstes Programm. Die Schildkröte soll einfache Linien zeichnen und daraus ein **L** bilden.

## Wo schreibst du den Code?

Wenn du den Anweisungen aus Kapitel 0 (Einrichtung) gefolgt bist, hast du bereits ein Projekt erstellt. Jetzt öffnest du die Datei **`src/main.rs`** in deinem Projekt-Ordner – dort schreibst du dein Programm hinein.

**Wichtig:** Die Datei `src/main.rs` ist die Hauptdatei deines Projekts. Hier beginnt dein Programm, wenn du es startest.

### Mehrere Programme im gleichen Projekt (optional)

Später möchtest du vielleicht mehrere verschiedene Programme ausprobieren, ohne das vorherige zu überschreiben. Dafür kannst du zusätzliche Programmdateien im Ordner `src/bin/` erstellen:

1. Erstelle einen Ordner `bin` in deinem `src`-Verzeichnis (falls er noch nicht existiert)
2. Lege dort neue Dateien an, z.B. `src/bin/mein_programm.rs`
3. Starte sie mit: `cargo run --bin mein_programm`

Für den Anfang reicht aber `src/main.rs` vollkommen aus!

## Das erste Programm

Hier ist der Code für unser erstes Programm:

```rust
{{#include ../codesamples/examples/erstes_programm.rs}}
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

**Tipp:** Über der Zeile `fn main()` siehst du in VS Code einen **Run**-Button. Damit kannst du das Programm starten!

### 3. Die Stiftfarbe setzen

```rust
turtle.set_pen_color(BLUE);
```

Hier sagen wir der Schildkröte: "Verwende die Farbe Blau zum Zeichnen." `turtle` ist unsere Schildkröte und `set_pen_color` ist der Befehl, um die Farbe zu ändern.

### 4. Die erste Linie zeichnen

```rust
turtle.forward(100.0);
```

Dieser Befehl sagt der Schildkröte: "Gehe 100 Schritte vorwärts." Dabei zeichnet sie eine Linie.

### 5. Drehen

```rust
turtle.right(90.0);
```

Jetzt dreht sich die Schildkröte 90 Grad nach rechts. Sie schaut jetzt nach unten!

### 6. Die zweite Linie zeichnen

```rust
turtle.forward(100.0);
```

Die Schildkröte geht wieder 100 Schritte vorwärts – jetzt entsteht eine Linie nach unten.

Das Ergebnis ist ein **L**!

## Programm starten

So startest du dein Programm:

1. **Run-Button verwenden** (einfachste Methode):
   - Klicke auf **Run** über der `fn main()` Zeile
   - Warte einen Moment (beim ersten Mal dauert es etwas länger)
   - Ein Fenster öffnet sich und zeigt deine Zeichnung!

2. **Terminal verwenden**:
   - Öffne das Terminal in VS Code
   - Tippe `cargo run` und drücke Enter

## Was siehst du?

Wenn du das Programm ausführst, öffnet sich ein Fenster. Darin siehst du:
- Eine kleine Schildkröte (oft als Dreieck dargestellt)
- Die Schildkröte bewegt sich und zeichnet dabei
- Am Ende siehst du ein blaues **L**!

## Ein Quadrat zeichnen

Jetzt wollen wir etwas Anspruchsvolleres zeichnen: ein Quadrat! Ein Quadrat hat vier gleich lange Seiten. Wir müssen also viermal das Gleiche machen: vorwärts gehen und nach rechts drehen.

```rust
{{#include ../codesamples/examples/quadrat.rs}}
```

Hier wiederholen wir den Code viermal:
- Erste Seite: vorwärts, dann rechts drehen
- Zweite Seite: vorwärts, dann rechts drehen
- Dritte Seite: vorwärts, dann rechts drehen
- Vierte Seite: vorwärts, dann rechts drehen

Das funktioniert, aber du siehst: Der Code wiederholt sich! Es muss doch einen besseren Weg geben...

Genau! Den gibt es – und den lernst du in Kapitel 4 kennen, wenn wir über **Schleifen** sprechen. Schleifen sind ein Weg, Code-Wiederholungen zu vermeiden.

## Zusammenfassung

Du hast gerade gelernt:
- Wie ein einfaches Rust-Programm aufgebaut ist
- Was `use`, `fn main()` und `#[turtle_main]` bedeuten
- Wie man die Schildkröte bewegt (`forward`)
- Wie man die Schildkröte dreht (`right`)
- Wie man eine Farbe setzt (`set_pen_color`)
- Wie man das Programm mit dem Run-Button startet
- Dass sich Code manchmal wiederholt (das werden wir später eleganter lösen!)

Im nächsten Kapitel schauen wir uns die verschiedenen Bewegungs- und Drehbefehle genauer an.
