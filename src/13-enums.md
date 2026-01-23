# Enums und Zustände

Im letzten Kapitel hast du gelernt, wie man mit Structs zusammengehörige Daten gruppiert. Aber manchmal brauchst du etwas anderes: Du möchtest sagen "Es kann **entweder** dies **oder** das sein". Hier kommen **Enums** ins Spiel! Mit Enums kannst du verschiedene Varianten oder Zustände modellieren.

## Was ist ein Enum?

Ein **Enum** (kurz für "Enumeration" = Aufzählung) ist ein Typ mit mehreren möglichen Varianten. Stell dir vor:

Ein Ampelzustand kann sein:
- Rot
- Gelb
- Grün

Aber niemals zwei gleichzeitig! Ein Enum ist perfekt dafür:

```rust
enum Ampel {
    Rot,
    Gelb,
    Gruen,
}
```

## Warum Enums verwenden?

Enums helfen dir:
1. **Zustände modellieren**: Ein Spiel kann "Läuft", "Gewonnen" oder "Verloren" sein
2. **Fehler vermeiden**: Der Compiler prüft, dass du alle Fälle behandelst
3. **Klarheit**: Der Code zeigt deutlich, welche Möglichkeiten es gibt
4. **Sicherheit**: Du kannst keine ungültigen Zustände erstellen

## Einen Enum definieren

So definierst du einen Enum:

```rust
{{#include ../codesamples/examples/enum_definition.rs}}
```

- `enum` leitet die Definition ein
- `Richtung` ist der Name des Enums (groß geschrieben!)
- Die Varianten stehen in geschweiften Klammern
- Jede Variante wird mit Komma getrennt

## Enum-Werte erstellen

So verwendest du eine Variante:

```rust
{{#include ../codesamples/examples/enum_werte.rs}}
```

Mit `::` wählst du eine Variante aus.

## Match: Entscheidungen treffen

Das Besondere an Enums ist `match` – damit kannst du für jede Variante etwas anderes tun:

```rust
{{#include ../codesamples/examples/enum_match.rs}}
```

**Match ist super stark!**
- Du musst alle Varianten behandeln (der Compiler prüft das!)
- Du kannst für jede Variante anderen Code ausführen
- Es ist sicherer als viele `if-else`

## Enums mit Daten

Enums können auch Daten enthalten:

```rust
{{#include ../codesamples/examples/enum_mit_daten.rs}}
```

Hier hat jede Variante eigene Daten! Das ist sehr mächtig.

## Anwendung: Spielzustand

Für ein Spiel ist ein Enum perfekt, um den Zustand zu modellieren:

```rust
{{#include ../codesamples/examples/enum_spielzustand.rs}}
```

So kannst du klar zwischen verschiedenen Spielzuständen unterscheiden!

## Anwendung: Hangman-Ergebnis

Für Hangman können wir das Spielergebnis mit einem Enum modellieren:

```rust
{{#include ../codesamples/examples/enum_hangman.rs}}
```

Mit `match` entscheiden wir, was zu tun ist – je nach Spielzustand!

## Option: Ein spezieller Enum

Rust hat einen eingebauten Enum `Option`, der sehr häufig verwendet wird:

```rust
enum Option<T> {
    Some(T),    // Es gibt einen Wert
    None,       // Es gibt keinen Wert
}
```

Das ist nützlich, wenn etwas da sein kann – oder eben nicht:

```rust
{{#include ../codesamples/examples/enum_option.rs}}
```

Mit `Option` vermeidest du Fehler durch fehlende Werte!

## Result: Erfolg oder Fehler

Ein anderer wichtiger Enum ist `Result`:

```rust
enum Result<T, E> {
    Ok(T),      // Erfolg mit Wert
    Err(E),     // Fehler
}
```

Das verwendest du, wenn etwas schiefgehen kann:

```rust
{{#include ../codesamples/examples/enum_result.rs}}
```

## Match mit Mustern

Du kannst in `match` auch Daten extrahieren:

```rust
{{#include ../codesamples/examples/enum_match_muster.rs}}
```

Mit `nachricht` extrahieren wir den Text aus der `Nachricht`-Variante!

## If let: Eine einfache Alternative

Wenn du nur eine Variante prüfen willst, ist `if let` kürzer als `match`:

```rust
{{#include ../codesamples/examples/enum_if_let.rs}}
```

`if let` ist wie ein `match`, der nur einen Fall behandelt.

## Enums in Structs

Du kannst Enums in Structs verwenden:

```rust
{{#include ../codesamples/examples/enum_in_struct.rs}}
```

So kombinierst du die Stärken von Structs und Enums!

## While let: Schleifen mit Pattern Matching

Du kannst `while let` für Schleifen verwenden:

```rust
let mut option = Some(5);

while let Some(wert) = option {
    println!("Wert: {}", wert);
    if wert > 0 {
        option = Some(wert - 1);
    } else {
        option = None;
    }
}
```

## Enum-Methoden

Wie bei Structs kannst du auch Enums Methoden geben:

```rust
{{#include ../codesamples/examples/enum_methoden.rs}}
```

## Zusammenfassung

Du hast gelernt:
- `enum Name { Variante1, Variante2 }` - Definiert einen Enum
- `Name::Variante` - Wählt eine Variante
- `match wert { Variante => ... }` - Entscheidet basierend auf Variante
- Enums können Daten enthalten
- `Option<T>` - Für optionale Werte (Some/None)
- `Result<T, E>` - Für Operationen, die fehlschlagen können (Ok/Err)
- `if let` - Kurzform für einfache Matches
- Enums modellieren "entweder-oder"-Situationen

## Übungsaufgaben

### Aufgabe 1: Wochentag

Erstelle einen Enum `Wochentag` mit allen sieben Tagen. Schreibe eine Funktion, die mit `match` ausgibt, ob es ein Werktag oder Wochenende ist.

### Aufgabe 2: Verkehrsmittel

Erstelle einen Enum `Verkehrsmittel` mit:
- `Auto(String)` - mit Marke
- `Fahrrad`
- `Bahn(u32)` - mit Liniennummer

Schreibe Code, der verschiedene Verkehrsmittel ausgibt.

### Aufgabe 3: Taschenrechner

Erstelle einen Enum `Operation` mit:
- `Addition(f32, f32)`
- `Subtraktion(f32, f32)`
- `Multiplikation(f32, f32)`
- `Division(f32, f32)`

Schreibe eine Funktion `berechne(op: Operation) -> f32`, die mit `match` das Ergebnis berechnet.

### Aufgabe 4: Anmeldestatus

Erstelle einen Enum `AnmeldeStatus` mit:
- `Angemeldet(String)` - mit Benutzernamen
- `Abgemeldet`

Schreibe Code, der prüft, ob jemand angemeldet ist, und ggf. den Namen ausgibt.

### Aufgabe 5: Spielaktion

Erstelle einen Enum `Aktion` für ein Spiel mit:
- `Bewegen(i32, i32)` - mit x- und y-Koordinaten
- `Angreifen(String)` - mit Ziel
- `Heilen`
- `Warten`

Schreibe eine Funktion, die die Aktion ausführt (simuliert durch Ausgaben).

### Aufgabe 6: Wettervorhersage

Erstelle einen Enum `Wetter` mit:
- `Sonnig(u32)` - mit Temperatur
- `Regnerisch`
- `Schnee`
- `Bewoelkt`

Schreibe Code, der passende Kleidungs-Empfehlungen gibt.

## Wichtige Hinweise

1. **Enum-Namen groß**: `Spielzustand`, nicht `spielzustand`
2. **Varianten groß**: `Gewonnen`, nicht `gewonnen`
3. **Match vollständig**: Behandle alle Varianten oder verwende `_` für "Rest"
4. **Daten extrahieren**: In `match` kannst du Werte aus Varianten holen
5. **Option und Result**: Diese Enums sind in Rust allgegenwärtig!

## Der Unterschied: Struct vs Enum

**Struct** (UND):
```rust
struct Person {
    name: String,    // UND
    alter: u32,      // UND
    stadt: String,   // UND
}
```
→ Hat alle Felder gleichzeitig

**Enum** (ODER):
```rust
enum Zustand {
    Laedt,           // ODER
    Bereit,          // ODER
    Fehler,          // ODER
}
```
→ Ist genau eine Variante

## Was kommt als Nächstes?

Du hast jetzt die Grundlagen für komplexe Datenstrukturen gelernt! Im nächsten Kapitel geht es darum, wie du **Benutzereingaben** verarbeitest – ein wichtiger Schritt für interaktive Programme wie unser Hangman-Spiel!
