# Variablen

Variablen sind wie Schubladen, in denen du Werte aufbewahren kannst. Sie machen deine Programme flexibler und leichter zu verstehen.

## Was ist eine Variable?

Eine **Variable** ist ein Name, der einen Wert speichert. Stell dir vor, du hast eine Schublade mit dem Etikett "Seitenlänge". In dieser Schublade liegt eine Zahl, zum Beispiel 80. Immer wenn du diese Zahl brauchst, schaust du in die Schublade "Seitenlänge".

In Rust sieht das so aus:

```rust
let seitenlaenge = 80.0;
```

- `let` bedeutet "lass" oder "erstelle" (eine neue Variable)
- `seitenlaenge` ist der Name der Variable
- `=` bedeutet "bekommt den Wert"
- `80.0` ist der Wert, den wir speichern

## Warum Variablen verwenden?

Variablen haben mehrere Vorteile:

1. **Lesbarkeit**: Der Code ist leichter zu verstehen
2. **Wiederverwendung**: Du kannst den Wert mehrmals verwenden
3. **Änderbarkeit**: Du musst nur an einer Stelle ändern

## Beispiel: Quadrat mit Variable

```rust
{{#include ../codesamples/examples/variablen.rs}}
```

Hier verwenden wir eine Variable `seitenlaenge` für die Größe des Quadrats. Wenn wir die Größe ändern wollen, müssen wir nur die Zeile `let seitenlaenge = 80.0;` ändern, statt alle vier `forward`-Befehle!

## Mehrere Variablen

Du kannst so viele Variablen verwenden, wie du möchtest:

```rust
{{#include ../codesamples/examples/mehrere_variablen.rs}}
```

Hier haben wir drei Variablen:
- `seitenlaenge` - für die Länge jeder Seite
- `anzahl_seiten` - für die Anzahl der Seiten
- `winkel` - für den Drehwinkel

Das macht den Code sehr übersichtlich!

## Variablen in Schleifen verwenden

Du kannst auch Variablen in Schleifen verwenden:

```rust
for _ in 0..anzahl_seiten {
    turtle.forward(seitenlaenge);
    turtle.right(winkel);
}
```

Hier verwenden wir `anzahl_seiten` als Anzahl der Wiederholungen!

## Variablen ändern

Manchmal möchtest du den Wert einer Variable ändern. Dafür brauchst du das Wort `mut` (von "mutable" = veränderbar):

```rust
let mut laenge = 5.0;
laenge = laenge + 3.0;  // Jetzt ist laenge 8.0
```

## Beispiel: Spirale mit sich ändernder Variable

```rust
{{#include ../codesamples/examples/spirale.rs}}
```

Hier beginnt `laenge` bei 5.0 und wird in jedem Schritt um 3.0 erhöht. So entsteht eine Spirale, bei der die Linien immer länger werden!

## Rechenoperationen mit Variablen

Du kannst mit Variablen rechnen:

```rust
let a = 10.0;
let b = 5.0;
let summe = a + b;        // 15.0 (Addition)
let differenz = a - b;    // 5.0 (Subtraktion)
let produkt = a * b;      // 50.0 (Multiplikation)
let quotient = a / b;     // 2.0 (Division)
```

## Zahlentypen

In Rust gibt es verschiedene Arten von Zahlen:

- **Ganze Zahlen** (ohne Komma): `let anzahl = 5;`
- **Kommazahlen** (mit Punkt): `let laenge = 10.0;`

Für die Turtle-Befehle brauchst du meist Kommazahlen (mit `.0` am Ende).

## Gute Namen für Variablen

Wähle Namen, die beschreiben, was in der Variable gespeichert ist:

**Gut:**
- `seitenlaenge`
- `anzahl_wiederholungen`
- `drehwinkel`

**Nicht so gut:**
- `x`
- `zahl1`
- `dings`

Gute Namen machen deinen Code viel leichter verständlich!

## Zusammenfassung

Du hast gelernt:
- `let name = wert;` - Erstellt eine Variable
- `let mut name = wert;` - Erstellt eine veränderbare Variable
- Variablen machen Code lesbarer und flexibler
- Du kannst mit Variablen rechnen
- Gute Variablennamen sind wichtig für verständlichen Code

Im nächsten Kapitel lernst du **Funktionen** kennen – eine Möglichkeit, Code zu organisieren und wiederzuverwenden!
