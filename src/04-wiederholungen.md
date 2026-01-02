# Wiederholungen (Schleifen)

Du kennst jetzt alle wichtigen Bewegungs- und Drehbefehle der Schildkröte. Erinnerst du dich an das Quadrat aus Kapitel 2? Wir mussten den Code viermal schreiben:

```rust
turtle.forward(100.0);
turtle.right(90.0);

turtle.forward(100.0);
turtle.right(90.0);

turtle.forward(100.0);
turtle.right(90.0);

turtle.forward(100.0);
turtle.right(90.0);
```

Das war ziemlich mühsam! Stell dir vor, du möchtest 100 mal den gleichen Befehl ausführen. Es wäre sehr mühsam, diesen Befehl 100 mal zu schreiben! Hier kommen **Schleifen** ins Spiel.

## Was ist eine Schleife?

Eine **Schleife** ist ein Programmierkonstrukt, das Befehle mehrmals wiederholt. Du sagst dem Computer: "Führe diese Befehle X-mal aus."

## Die for-Schleife

Die häufigste Art von Schleife ist die `for`-Schleife. Hier ist die Grundstruktur:

```rust
for _ in 0..anzahl {
    // Befehle, die wiederholt werden
}
```

- `for` bedeutet "für jede Wiederholung"
- `0..anzahl` gibt an, wie oft wiederholt wird (von 0 bis anzahl-1)
- Der Unterstrich `_` bedeutet, dass wir die Zählvariable nicht brauchen
- Die Befehle in den geschweiften Klammern `{ }` werden wiederholt

## Beispiel: Ein Quadrat – jetzt mit Schleife!

Erinnere dich an unser Quadrat von vorhin. Mit einer Schleife wird der Code viel kürzer und übersichtlicher:

```rust
{{#include ../codesamples/examples/quadrat_schleife.rs}}
```

Wow! Statt 8 Zeilen Code haben wir jetzt nur noch 3 Zeilen in der Schleife. Das macht genau das Gleiche wie der lange Code vorher, ist aber viel einfacher zu lesen und zu verstehen!

## Beispiel: Ein Achteck

Ein Achteck hat 8 gleich lange Seiten. Wir können es mit einer Schleife zeichnen:

```rust
{{#include ../codesamples/examples/achteck.rs}}
```

### Wie funktioniert das?

- Die Schleife wiederholt sich 8 mal (`0..8`)
- In jeder Wiederholung:
  - Geht die Schildkröte 50 Schritte vorwärts
  - Dreht sie sich 45 Grad nach rechts (360 ÷ 8 = 45)
- Nach 8 Wiederholungen ist das Achteck fertig!

## Die Formel für regelmäßige Vielecke

Für ein Vieleck mit `n` Seiten ist der Drehwinkel:

**360 ÷ n**

Beispiele:
- Dreieck (3 Seiten): 360 ÷ 3 = 120 Grad
- Quadrat (4 Seiten): 360 ÷ 4 = 90 Grad
- Fünfeck (5 Seiten): 360 ÷ 5 = 72 Grad
- Sechseck (6 Seiten): 360 ÷ 6 = 60 Grad
- Achteck (8 Seiten): 360 ÷ 8 = 45 Grad

## Beispiel: Ein Stern

Wollen wir etwas Komplexeres zeichnen? Wie wäre es mit einem Stern?

```rust
{{#include ../codesamples/examples/stern.rs}}
```

Ein fünfzackiger Stern braucht einen anderen Winkel als ein regelmäßiges Fünfeck. Hier verwenden wir 144 Grad (das ist 720 ÷ 5).

## Warum sind Schleifen nützlich?

Schleifen sind sehr nützlich, weil:
1. **Weniger Code**: Statt 8 mal den gleichen Befehl zu schreiben, schreibst du ihn nur einmal
2. **Weniger Fehler**: Du kannst dich nicht vertippen, wenn du nur einmal schreibst
3. **Leichter zu ändern**: Willst du ein Zehneck statt eines Achtecks? Ändere einfach die `8` zu `10`!

## Probiere es selbst!

Versuche, diese Formen zu zeichnen:
- Ein Dreieck (3 Seiten, 120 Grad)
- Ein Sechseck (6 Seiten, 60 Grad)
- Ein Zwölfeck (12 Seiten, 30 Grad)

## Zusammenfassung

Du hast gelernt:
- Was eine Schleife ist (ein Befehl, der sich wiederholt)
- Wie man eine `for`-Schleife schreibt
- Die Formel für regelmäßige Vielecke: 360 ÷ anzahl_seiten
- Dass Schleifen den Code kürzer und einfacher machen

Im nächsten Kapitel lernst du, wie du mit Farben arbeitest und den Stift heben und senken kannst.
