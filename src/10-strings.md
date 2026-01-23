# Strings und Text

In den bisherigen Kapiteln hast du gelernt, wie man mit Zahlen und Grafiken arbeitet. Aber Programme müssen auch mit **Text** umgehen können! Ob es um ein Wort in einem Spiel, eine Nachricht an den Benutzer oder einen Namen geht – Text ist überall. In diesem Kapitel lernst du, wie man in Rust mit Text arbeitet.

## Was ist ein String?

Ein **String** ist einfach gesagt eine Folge von Buchstaben, Zahlen und Zeichen. In Rust gibt es zwei wichtige Arten von Text:

1. **`String`** - Ein Text, der sich ändern kann und dir "gehört"
2. **`&str`** - Ein Text, den du nur ausleihen darfst (eine "Textscheibe")

Stell dir vor:
- `String` ist wie ein Notizbuch, in das du schreiben und das du ändern kannst
- `&str` ist wie ein Blick auf eine Seite in einem Buch – du kannst lesen, aber nicht ändern

## Einen String erstellen

So erstellst du einen einfachen Text:

```rust
let wort = String::from("Hallo");
```

- `String::from("...")` erstellt einen neuen String
- Die Anführungszeichen `"..."` zeigen, dass es sich um Text handelt
- `wort` ist der Name unserer Variable

## Text ausgeben

Um Text zu sehen, verwenden wir `println!`:

```rust
{{#include ../codesamples/examples/strings_ausgeben.rs}}
```

**Wichtig:** `println!` ist ein **Makro** (erkennbar am `!`). Es gibt Text im Terminal aus.

## Text zusammenfügen

Du kannst mehrere Texte zusammenfügen:

```rust
{{#include ../codesamples/examples/strings_zusammenfuegen.rs}}
```

Mit `format!` kannst du Texte elegant kombinieren:

```rust
let begruessung = format!("{} {}!", gruss, name);
```

Die `{}` sind Platzhalter, die durch die Variablen ersetzt werden.

## Text vergleichen

Oft musst du prüfen, ob zwei Texte gleich sind:

```rust
{{#include ../codesamples/examples/strings_vergleichen.rs}}
```

**Wichtig:** Bei Vergleichen wird zwischen Groß- und Kleinschreibung unterschieden!
- `"Hallo"` ist **nicht gleich** `"hallo"`

## Groß- und Kleinschreibung

Manchmal möchtest du Text in Großbuchstaben oder Kleinbuchstaben umwandeln:

```rust
{{#include ../codesamples/examples/strings_gross_klein.rs}}
```

Mit `.to_lowercase()` kannst du alles in Kleinbuchstaben umwandeln. Das ist nützlich, wenn du Eingaben vergleichen willst!

## Textlänge herausfinden

So erfährst du, wie viele Zeichen ein Text hat:

```rust
{{#include ../codesamples/examples/strings_laenge.rs}}
```

`.len()` gibt die Anzahl der Bytes zurück. Bei einfachen lateinischen Buchstaben entspricht das der Anzahl der Zeichen.

## Einzelne Zeichen prüfen

Du kannst prüfen, ob ein Text ein bestimmtes Zeichen enthält:

```rust
{{#include ../codesamples/examples/strings_enthaelt.rs}}
```

`.contains()` prüft, ob ein Text einen anderen Text enthält.

## Strings sind nicht wie Zahlen

**Wichtig:** Du kannst Strings nicht einfach addieren wie Zahlen!

```rust
// Das funktioniert NICHT:
// let summe = "Hallo" + "Welt";  // Fehler!

// Stattdessen:
let text = format!("{} {}", "Hallo", "Welt");  // Funktioniert!
```

## &str vs String

Du wirst beiden Typen begegnen. Hier ist der Unterschied:

**`&str`** (gesprochen: "String-Scheibe"):
- Text in Anführungszeichen: `"Hallo"`
- Kann nicht geändert werden
- Braucht weniger Speicher
- Wird "geliehen" (ausgeliehen)

**`String`**:
- Mit `String::from("...")` erstellt
- Kann geändert werden (mit `mut`)
- Braucht mehr Speicher
- Gehört dir

```rust
let unveraenderlich: &str = "Hallo";           // &str
let veraenderlich = String::from("Hallo");     // String
```

Meist kannst du beide austauschbar verwenden – Rust konvertiert automatisch, wo nötig.

## Strings ändern

Um einen String zu ändern, brauchst du `mut`:

```rust
{{#include ../codesamples/examples/strings_aendern.rs}}
```

Mit `push_str()` fügst du Text hinzu.

## Anwendung: Geheimes Wort für ein Spiel

Stell dir vor, wir programmieren ein Ratespiel. Das Spiel hat ein geheimes Wort:

```rust
{{#include ../codesamples/examples/strings_spiel.rs}}
```

Hier verwenden wir Strings, um:
- Das geheime Wort zu speichern
- Eine Rateeingabe zu simulieren
- Zu prüfen, ob das Wort richtig geraten wurde

Das ist die Grundlage für Spiele wie Hangman!

## Zusammenfassung

Du hast gelernt:
- `String::from("...")` - Erstellt einen neuen String
- `println!("{}", text)` - Gibt Text aus
- `format!()` - Fügt Texte zusammen
- `text1 == text2` - Vergleicht zwei Texte
- `.to_lowercase()` - Wandelt in Kleinbuchstaben um
- `.len()` - Gibt die Länge zurück
- `.contains()` - Prüft, ob ein Text enthalten ist
- `&str` vs `String` - Ausgeliehener vs eigener Text
- `.push_str()` - Fügt Text an einen String an

## Übungsaufgaben

### Aufgabe 1: Persönliche Begrüßung

Erstelle ein Programm, das deinen Namen und dein Alter in einer Begrüßung ausgibt:
```
Hallo, ich bin Max und ich bin 12 Jahre alt!
```

**Tipp:** Verwende `format!()` mit Platzhaltern `{}`.

### Aufgabe 2: Passwortprüfung

Schreibe ein Programm, das ein geheimes Passwort speichert und eine Eingabe vergleicht. Gib aus, ob das Passwort korrekt ist.

**Tipp:** Verwende `==` zum Vergleichen und `if` für die Bedingung (kennst du schon aus den Schleifen).

### Aufgabe 3: Wortlänge

Erstelle ein Programm, das verschiedene Wörter ausgibt und jeweils anzeigt, wie viele Buchstaben sie haben.

**Tipp:** Verwende `.len()` für die Länge.

### Aufgabe 4: Suchwort

Schreibe ein Programm, das prüft, ob ein bestimmter Buchstabe in einem Wort vorkommt.

**Tipp:** Verwende `.contains()` zum Prüfen.

## Wichtige Hinweise

1. **Anführungszeichen**: Text steht immer in Anführungszeichen `"..."`
2. **Formatierung**: Verwende `format!()` oder `println!()` zum Kombinieren
3. **Vergleichen**: Mit `==` kannst du Texte vergleichen
4. **Ändern**: Nur mit `mut` kannst du Strings ändern

Im nächsten Kapitel lernst du **Sammlungen** kennen – Listen von Werten, mit denen du mehrere Wörter oder Zahlen gleichzeitig verwalten kannst. Das ist wichtig für unser Hangman-Spiel, bei dem wir eine Liste von möglichen Wörtern brauchen!
