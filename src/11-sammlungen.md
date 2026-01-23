# Sammlungen: Vektoren und Arrays

Im letzten Kapitel hast du gelernt, wie man mit einzelnen Texten arbeitet. Aber was, wenn du mehrere Wörter, viele Zahlen oder eine ganze Liste von Werten speichern möchtest? Hier kommen **Sammlungen** ins Spiel! Sie sind wie Schubladen mit mehreren Fächern, in denen du viele Werte gleichzeitig aufbewahren kannst.

## Was ist eine Liste?

Eine **Liste** (oder Sammlung) ist eine geordnete Folge von Werten. Stell dir vor:
- Ein Regal mit nummerierten Fächern (0, 1, 2, 3...)
- Jedes Fach enthält einen Wert
- Du kannst auf jedes Fach über seine Nummer zugreifen

In Rust gibt es zwei wichtige Arten von Listen:
1. **Vec** (Vektor) - Eine Liste, die wachsen und schrumpfen kann
2. **Array** - Eine Liste mit fester Größe

## Was ist ein Vektor (Vec)?

Ein **Vektor** ist eine flexible Liste. Du kannst Elemente hinzufügen oder entfernen, und die Liste passt sich automatisch an.

So erstellst du einen leeren Vektor:

```rust
let mut zahlen: Vec<i32> = Vec::new();
```

- `Vec<i32>` bedeutet "Vektor von ganzen Zahlen"
- `Vec::new()` erstellt einen neuen, leeren Vektor
- `mut` ist wichtig, damit wir Elemente hinzufügen können

## Einen Vektor mit Werten erstellen

Meist ist es einfacher, direkt Werte anzugeben:

```rust
{{#include ../codesamples/examples/vektor_erstellen.rs}}
```

Mit `vec![]` erstellst du einen Vektor und gibst die Werte in eckigen Klammern an.

## Auf Elemente zugreifen

Du kannst auf einzelne Elemente über ihre **Indexnummer** zugreifen:

```rust
{{#include ../codesamples/examples/vektor_zugriff.rs}}
```

**Wichtig:** Die Zählung beginnt bei 0!
- Erstes Element: Index 0
- Zweites Element: Index 1
- Drittes Element: Index 2
- usw.

## Elemente hinzufügen

Mit `.push()` fügst du ein Element am Ende hinzu:

```rust
{{#include ../codesamples/examples/vektor_push.rs}}
```

Der Vektor wächst automatisch, wenn du Elemente hinzufügst!

## Durch einen Vektor iterieren

Du kannst mit einer Schleife durch alle Elemente gehen:

```rust
{{#include ../codesamples/examples/vektor_schleife.rs}}
```

Hier geht die Schleife durch jedes Element im Vektor. `&farben` bedeutet "alle Elemente von farben".

## Anzahl der Elemente

Mit `.len()` erfährst du, wie viele Elemente ein Vektor hat:

```rust
{{#include ../codesamples/examples/vektor_laenge.rs}}
```

## Einen zufälligen Wert auswählen

Für Spiele ist es oft nützlich, ein zufälliges Element auszuwählen:

```rust
{{#include ../codesamples/examples/vektor_zufaellig.rs}}
```

Hier verwenden wir die Funktion `gen_range` aus der `macroquad`-Bibliothek, um einen zufälligen Index zu erzeugen.

## Vektoren mit Text

Vektoren können auch Strings enthalten:

```rust
{{#include ../codesamples/examples/vektor_strings.rs}}
```

Mit `.to_string()` wandelst du `&str` in `String` um.

## Anwendung: Wortliste für Hangman

Für ein Hangman-Spiel brauchen wir eine Liste von Wörtern, aus der wir zufällig eines auswählen:

```rust
{{#include ../codesamples/examples/vektor_hangman.rs}}
```

Das ist perfekt für ein Ratespiel! Bei jedem Start wird ein neues, zufälliges Wort gewählt.

## Arrays: Listen mit fester Größe

Ein **Array** ist eine Liste mit fester Größe. Die Anzahl der Elemente steht von Anfang an fest und kann nicht geändert werden.

```rust
{{#include ../codesamples/examples/array_beispiel.rs}}
```

Arrays sind schneller als Vektoren, aber weniger flexibel.

## Vec vs Array: Was ist der Unterschied?

**Vektor (Vec)**:
- ✅ Kann wachsen und schrumpfen
- ✅ Flexibel
- ❌ Etwas langsamer
- Verwendung: Wenn du nicht weißt, wie viele Elemente du brauchst

**Array**:
- ✅ Feste Größe
- ✅ Etwas schneller
- ❌ Kann nicht wachsen
- Verwendung: Wenn die Anzahl von Anfang an feststeht

**Faustregel:** Wenn du dir unsicher bist, verwende einen Vektor!

## Mehrdimensionale Listen

Du kannst auch Listen in Listen speichern:

```rust
{{#include ../codesamples/examples/vektor_2d.rs}}
```

Das ist nützlich für Raster, Spielfelder oder Tabellen.

## Wichtige Methoden für Vektoren

Hier sind die wichtigsten Operationen auf einen Blick:

```rust
let mut v = vec![1, 2, 3];

v.push(4);              // Element hinzufügen
v.len();                // Anzahl der Elemente
v[0];                   // Erstes Element
v.is_empty();           // Prüfen, ob leer
v.contains(&2);         // Prüfen, ob Element enthalten ist
v.clear();              // Alle Elemente entfernen
```

## Zusammenfassung

Du hast gelernt:
- `Vec::new()` oder `vec![...]` - Erstellt einen Vektor
- `vektor[index]` - Greift auf ein Element zu (Index beginnt bei 0!)
- `.push(wert)` - Fügt ein Element hinzu
- `for element in &vektor { }` - Durchläuft alle Elemente
- `.len()` - Gibt die Anzahl der Elemente zurück
- Arrays `[1, 2, 3]` - Listen mit fester Größe
- Vektoren sind flexibler, Arrays sind schneller

## Übungsaufgaben

### Aufgabe 1: Lieblingszahlen

Erstelle einen Vektor mit deinen 5 Lieblingszahlen und gib sie alle aus.

**Tipp:** Verwende `vec![]` zum Erstellen und eine Schleife zum Ausgeben.

### Aufgabe 2: Namen sammeln

Erstelle einen leeren Vektor und füge 3 Namen deiner Freunde hinzu. Gib dann alle Namen aus.

**Tipp:** Verwende `Vec::new()`, dann `.push()` für jeden Namen.

### Aufgabe 3: Summe berechnen

Erstelle einen Vektor mit Zahlen und berechne die Summe aller Zahlen.

**Tipp:** Verwende eine Schleife und eine Variable, um die Summe aufzuaddieren.

### Aufgabe 4: Tier-Liste

Erstelle einen Vektor mit Tiernamen. Lass das Programm ausgeben:
- Wie viele Tiere es gibt
- Das erste und letzte Tier
- Alle Tiere nacheinander

### Aufgabe 5: Würfel simulieren

Erstelle ein Programm, das einen sechsseitigen Würfel simuliert:
- Erstelle einen Vektor mit den Zahlen 1-6
- Wähle zufällig eine Zahl aus
- Gib die gewürfelte Zahl aus

**Tipp:** Verwende `macroquad::rand::gen_range(0, 6)` für einen zufälligen Index.

### Aufgabe 6: Einkaufsliste

Erstelle eine Einkaufsliste als Vektor mit mindestens 5 Produkten. Gib die Liste nummeriert aus:
```
1. Äpfel
2. Brot
3. Milch
...
```

**Tipp:** Du kannst in der Schleife einen Zähler mitführen.

## Wichtige Hinweise

1. **Index beginnt bei 0**: Das erste Element ist `vektor[0]`, nicht `vektor[1]`!
2. **mut bei Vektoren**: Wenn du Elemente hinzufügen willst, brauchst du `mut`
3. **Typen sind einheitlich**: Alle Elemente müssen den gleichen Typ haben
4. **Grenzen beachten**: `vektor[10]` stürzt ab, wenn der Vektor nur 5 Elemente hat!

Im nächsten Kapitel lernst du **Strukturen (Structs)** kennen – damit kannst du eigene, komplexere Datentypen erstellen. Das ist wichtig, um den Zustand eines Spiels zu organisieren!
