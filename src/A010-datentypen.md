# Mehr über Daten

Bis jetzt hast du bereits mit Zahlen, Text und einfachen Befehlen gearbeitet. Jetzt kommt ein wichtiges Grundprinzip der Programmierung: jedes Stück Daten hat einen **Datentyp**.

## Was ist ein Datentyp?

Ein **Datentyp** beschreibt, welche Art von Information ein Wert ist. Er sagt dem Computer zum Beispiel:

- ist das etwas mit Zahlen?
- ist das Text?
- ist das wahr oder falsch?
- ist das eine Liste von Dingen?

Stell dir Datentypen wie Etiketten auf Gegenständen vor:

- `12` ist eine Zahl
- `3.14` ist eine Zahl mit Komma
- `"Hallo"` ist Text
- `true` ist ein Wahrheitswert

Rust ist eine **starke, statische Sprache**. Das heißt: Der Typ eines Werts wird meistens von Rust erkannt und überprüft. Dadurch werden viele Fehler schon vorher entdeckt.

## Warum brauchen wir Datentypen?

Datentypen helfen dem Computer, zu verstehen, was du tun willst.

Der Computer speichert Daten im Endeffekt nur als Nullen und Einsen. Ein einzelnes Byte ist 8 Bits, zum Beispiel:

```text
01000001
```

Diese Bitfolge kann auf verschiedene Weise interpretiert werden:

- als ASCII-Buchstabe: `A`
- als Zahl vom Typ `u8`: `65`

Das Wichtigste ist: Die Bits sind gleich, aber der **Datentyp** sagt dem Computer, wie er sie lesen soll.

Ohne Datentypen könnte der Computer nicht wissen, ob `01000001` ein Buchstabe, eine Zahl oder ein Teil einer Kommazahl ist.

```rust
let alter = 12;
let preis = 9.99;
let ist_aktiv = true;
let name = "Anna";
```

Diese Werte sind alle unterschiedlich:

- `alter` ist eine ganze Zahl
- `preis` ist eine Komman-Zahl mit Nachkommastellen
- `ist_aktiv` ist ein Wahrheitswert (`true` oder `false`)
- `name` ist Text, also mehrere Buchstaben hintereinander.

## Rust erkennt Typen oft automatisch

In vielen Fällen musst du den Typ nicht selbst schreiben:

```rust
let zahl = 42;
let text = "Hallo";
```

Rust erkennt automatisch, dass `42` eine Zahl ist und `"Hallo"` Text.

Manchmal willst du aber den Typ bewusst festlegen, zum Beispiel wenn du ein bestimmtes Verhalten brauchst:

```rust
let alter: u32 = 12;
let preis: f64 = 9.99;
```

Das `u32` und `f64` sind Typen aus Rust. Du wirst sie später noch genauer kennenlernen.

## Ein wichtiger Gedanke

Datentypen sind die Grundlage dafür, dass Programme zuverlässig funktionieren. Sie helfen dabei, Daten sauber zu speichern und zu verarbeiten.

Einige typische Datentypen, die du später kennen wirst, sind:

- Text: Strings
- Liste von Werten: Sammlungen
- zusammengesetzte Daten: Structs
- verschiedene Zustände: Enums


## Zusammenfassung

- Jeder Wert hat einen Datentyp
- Der Datentyp sagt, welche Art von Daten gespeichert werden
- Rust prüft Typen oft automatisch
- Datentypen verhindern viele Fehler

In den nächsten Kapiteln lernst du die wichtigsten Datentypen in Rust genauer kennen: Strings, Sammlungen, Structs und Enums.
