# Strukturen (Structs)

In den letzten Kapiteln hast du gelernt, wie man einzelne Werte (Strings, Zahlen) und Listen (Vektoren) speichert. Aber was, wenn du zusammengehörige Daten als eine Einheit behandeln möchtest? Hier kommen **Strukturen** (Structs) ins Spiel! Mit Structs kannst du deine eigenen Datentypen erstellen.

## Was ist ein Struct?

Ein **Struct** (kurz für "Structure" = Struktur) ist wie eine Karteikarte mit mehreren beschrifteten Feldern. Stell dir vor, du hast eine Karteikarte für eine Person:

```
┌─────────────────────┐
│ Name:   Anna        │
│ Alter:  12          │
│ Stadt:  Berlin      │
└─────────────────────┘
```

In Rust sieht das so aus:

```rust
struct Person {
    name: String,
    alter: u32,
    stadt: String,
}
```

## Warum Structs verwenden?

Structs helfen dir:
1. **Zusammengehörige Daten gruppieren**: Name, Alter und Stadt gehören zusammen
2. **Code organisieren**: Alles über eine Person steht an einem Ort
3. **Fehler vermeiden**: Du kannst keine Stadt vergessen, wenn sie Teil des Structs ist
4. **Lesbarkeit**: Der Code wird klarer und verständlicher

## Einen Struct definieren

So definierst du einen eigenen Datentyp:

```rust
{{#include ../codesamples/examples/struct_definition.rs}}
```

- `struct` leitet die Definition ein
- `Punkt` ist der Name des Structs (groß geschrieben!)
- Die Felder stehen in geschweiften Klammern
- Jedes Feld hat einen Namen und einen Typ

## Eine Instanz erstellen

So erstellst du eine konkrete Instanz (ein Exemplar) eines Structs:

```rust
{{#include ../codesamples/examples/struct_instanz.rs}}
```

Hier füllen wir die "Karteikarte" mit konkreten Werten aus.

## Auf Felder zugreifen

Mit einem Punkt `.` greifst du auf die Felder zu:

```rust
{{#include ../codesamples/examples/struct_zugriff.rs}}
```

Das funktioniert wie bei den Methoden, die du schon kennst (z.B. `text.len()`).

## Felder ändern

Um Felder zu ändern, brauchst du `mut`:

```rust
{{#include ../codesamples/examples/struct_aendern.rs}}
```

Ohne `mut` sind alle Felder unveränderlich!

## Struct mit Methoden

Du kannst Funktionen direkt an einen Struct binden. Das nennt man **Methoden**:

```rust
{{#include ../codesamples/examples/struct_methoden.rs}}
```

Mit `impl` (für "implementation") definierst du Methoden für einen Struct.
- `&self` bedeutet "diese Instanz"
- Methoden werden mit dem Punkt aufgerufen: `rechteck.flaeche()`

## Anwendung: Spieler in einem Spiel

Für ein Spiel könntest du einen Spieler so modellieren:

```rust
{{#include ../codesamples/examples/struct_spieler.rs}}
```

Hier haben wir einen Struct, der den Zustand eines Spielers speichert, und Methoden, um damit zu arbeiten.

## Anwendung: Hangman-Spielzustand

Für das Hangman-Spiel brauchen wir verschiedene Informationen:

```rust
{{#include ../codesamples/examples/struct_hangman.rs}}
```

Der Struct organisiert alle wichtigen Daten des Spiels an einem Ort!

## Mehrere Structs kombinieren

Structs können andere Structs enthalten:

```rust
{{#include ../codesamples/examples/struct_kombiniert.rs}}
```

So kannst du komplexe Datenstrukturen aufbauen!

## Structs und Vektoren

Du kannst Vektoren von Structs erstellen:

```rust
{{#include ../codesamples/examples/struct_vektor.rs}}
```

Das ist perfekt für Listen von Spielern, Gegnern, oder anderen Objekten!

## Konstruktor-Funktionen

Oft ist es praktisch, eine Funktion zu haben, die einen Struct erstellt:

```rust
impl Spieler {
    fn neu(name: String) -> Spieler {
        Spieler {
            name,
            punkte: 0,
            leben: 3,
        }
    }
}

// Verwendung:
let spieler = Spieler::neu("Anna".to_string());
```

Das nennt man einen **Konstruktor** (englisch "constructor").

## Der Unterschied zu Variablen

**Variable**:
```rust
let x = 10;
let name = "Anna";
```
→ Einzelne, unabhängige Werte

**Struct**:
```rust
let person = Person {
    name: "Anna".to_string(),
    alter: 12,
};
```
→ Mehrere zusammengehörige Werte als Einheit

## Wichtige Begriffe

- **Struct**: Die Definition (das "Rezept")
- **Instanz**: Ein konkretes Exemplar (die "fertige Torte nach Rezept")
- **Feld**: Eine Variable innerhalb des Structs
- **Methode**: Eine Funktion, die zu einem Struct gehört
- **impl**: Block, in dem Methoden definiert werden

## Zusammenfassung

Du hast gelernt:
- `struct Name { felder }` - Definiert einen eigenen Datentyp
- `let instanz = Name { werte }` - Erstellt eine Instanz
- `instanz.feld` - Greift auf ein Feld zu
- `impl Name { fn methode() }` - Definiert Methoden
- `&self` in Methoden - Bezieht sich auf die Instanz
- Structs gruppieren zusammengehörige Daten
- Structs machen Code organisierter und lesbarer

## Übungsaufgaben

### Aufgabe 1: Buch-Struct

Erstelle einen Struct `Buch` mit den Feldern:
- `titel` (String)
- `autor` (String)
- `seiten` (u32)

Erstelle dann 2-3 Bücher und gib ihre Informationen aus.

### Aufgabe 2: Auto mit Methoden

Erstelle einen Struct `Auto` mit:
- `marke` (String)
- `geschwindigkeit` (u32)

Implementiere Methoden:
- `beschleunigen(&mut self, wert: u32)` - Erhöht die Geschwindigkeit
- `bremsen(&mut self, wert: u32)` - Verringert die Geschwindigkeit
- `anzeigen(&self)` - Gibt die Informationen aus

### Aufgabe 3: Rechteck-Rechner

Erstelle einen Struct `Rechteck` mit:
- `breite` (f32)
- `hoehe` (f32)

Implementiere Methoden:
- `flaeche(&self) -> f32` - Berechnet die Fläche
- `umfang(&self) -> f32` - Berechnet den Umfang
- `ist_quadrat(&self) -> bool` - Prüft, ob es ein Quadrat ist

### Aufgabe 4: Kontakt-Liste

Erstelle einen Struct `Kontakt` mit:
- `name` (String)
- `telefon` (String)

Erstelle einen Vektor von Kontakten und gib alle aus.

### Aufgabe 5: Punkteverwaltung

Erstelle einen Struct `Spielstand` mit:
- `spieler_name` (String)
- `punkte` (u32)
- `highscore` (u32)

Implementiere Methoden:
- `punkte_hinzufuegen(&mut self, punkte: u32)` - Fügt Punkte hinzu
- `neuer_highscore(&mut self)` - Aktualisiert den Highscore, falls die aktuellen Punkte höher sind

### Aufgabe 6: Monster-Kampf

Erstelle einen Struct `Monster` mit:
- `name` (String)
- `leben` (u32)
- `angriff` (u32)

Implementiere eine Methode `angreifen(&self, gegner: &mut Monster)`, die dem Gegner Schaden zufügt.

## Wichtige Hinweise

1. **Struct-Namen groß**: `Person`, nicht `person`
2. **Feldnamen klein**: `name`, nicht `Name`
3. **mut für Änderungen**: Struct braucht `mut`, wenn Felder geändert werden
4. **&self in Methoden**: Für Lesezugriff; `&mut self` für Änderungen
5. **Alle Felder angeben**: Beim Erstellen musst du alle Felder ausfüllen

Im nächsten Kapitel lernst du **Enums** kennen – damit kannst du verschiedene Zustände oder Varianten modellieren. Das ist perfekt, um zu prüfen, ob ein Spiel gewonnen, verloren oder noch läuft!
