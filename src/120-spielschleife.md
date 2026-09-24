# Spielschleifen: Dein erstes Spiel

Jetzt wird es spannend! Du lernst, wie man ein richtiges Spiel programmiert: Ein Zahlenratespiel!

## Was ist eine Spielschleife?

Eine **Spielschleife** wiederholt immer wieder:
1. Frage den Spieler
2. Reagiere auf die Antwort
3. Ist das Spiel vorbei? Wenn nein, wiederhole

## Das Zahlenratespiel

Wir bauen ein Spiel, bei dem der Computer sich eine Zahl zwischen 0 und 100 ausdenkt und du sie erraten musst!

### Schritt 1: Die Zufallszahl

Zuerst brauchen wir eine Zufallszahl:

```rust
{{#include ../codesamples/examples/ratespiel_schritte.rs:step1}}
```

Wichtig: Macroquad verwendet einen globalen Zufallszahlengenerator. Deshalb seedest du ihn einmal am Anfang mit dem aktuellen Zeitstempel:

```rust
use std::time::{SystemTime, UNIX_EPOCH};

macroquad::rand::srand(
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64,
);
```

Ohne diesen Aufruf bleibt der Generator beim Standardwert stehen und `gen_range(0, 101)` liefert immer wieder `0`.

### Schritt 2: Nach einer Zahl fragen

Jetzt fragen wir den Spieler:

```rust
{{#include ../codesamples/examples/ratespiel_schritte.rs:step2}}
```

### Schritt 3: Prüfen, ob richtig

Jetzt vergleichen wir:

```rust
{{#include ../codesamples/examples/ratespiel_schritte.rs:step3}}
```

### Schritt 4: Die Schleife

Eine Schleife wiederholt einen Block wieder und wieder. Genau das brauchen wir für ein Ratespiel: Solange der Spieler falsch liegt, soll das Programm erneut fragen.

#### Erst eine einfache Schleife

```rust
loop {
    println!("Noch ein Durchlauf!");
    break;
}
```

`loop { ... }` startet eine Endlosschleife. `break;` beendet sie sofort wieder. Ohne `break` würde das Programm für immer weiterlaufen.

#### Jetzt mit Zähler

```rust
let mut versuche = 0;

loop {
    versuche += 1;
    println!("Versuch {}", versuche);

    if versuche == 3 {
        break;
    }
}
```

**Was passiert hier?**
1. `let mut versuche = 0;` erstellt einen Zähler
2. In jedem Durchlauf wird `versuche += 1;` ausgeführt
3. `println!(...)` zeigt den aktuellen Versuch an
4. `if versuche == 3 { break; }` beendet die Schleife nach 3 Versuchen

Das ist die Grundidee der Spielschleife: Zähle mit, mache etwas, prüfe, ob das Spiel vorbei ist.

#### Jetzt mit Tipp und Vergleich

Als Nächstes ersetzen wir nur den `println!` durch eine echte Prüfung:

```rust
let mut versuche = 0;
let geheime_zahl = 42;

loop {
    versuche += 1;

    let tipp = 10; // später wird hier die Eingabe stehen
    println!("Dein Tipp: {}", tipp);

    if tipp == geheime_zahl {
        println!("Richtig!");
        break;
    } else if tipp < geheime_zahl {
        println!("Zu klein!");
    } else {
        println!("Zu groß!");
    }
}
```

Jetzt sieht die Schleife schon sehr nach einem Ratespiel aus:
- Frage nach einer Zahl
- Vergleiche den Tipp mit der geheimen Zahl
- Wenn falsch: gib eine Meldung aus
- Wenn richtig: `break` und Spiel ist beendet

Du musst jetzt nur noch die feste Zahl `let tipp = 10;` durch die echte Benutzereingabe ersetzen. Genau das macht die nächste Übung!

#### Dialoge in der Schleife

Jetzt kommt der letzte wichtige Schritt: In einer echten Spielschleife fragt das Programm nicht mit einer festen Zahl, sondern mit einem Dialogfenster nach der Eingabe des Spielers.

```rust
let mut versuche = 0;
let geheime_zahl = 42;

loop {
    versuche += 1;

    let eingabe = rustydialogs::TextInput {
        title: "Dein Tipp",
        message: "Rate die Zahl (0-100):",
        value: "",
        mode: rustydialogs::TextInputMode::SingleLine,
        owner: None,
    }
    .show();

    let tipp = match eingabe {
        Some(text) => text.parse::<i32>().unwrap(),
        None => break,
    };

    if tipp == geheime_zahl {
        println!("Richtig!");
        break;
    } else if tipp < geheime_zahl {
        println!("Zu klein!");
    } else {
        println!("Zu groß!");
    }
}
```

**Schritt für Schritt:**
1. `TextInput { ... }` zeigt einen Dialog an
2. `.show()` liefert den eingegebenen Text oder `None`, wenn der Spieler abbricht
3. `text.parse::<i32>()` wandelt den Text in eine Zahl um
4. Danach prüft das Programm mit `if`/`else if` wieder, ob die Zahl richtig ist
5. Wenn nicht, läuft die Schleife erneut und fragt erneut

Damit hast du das Muster der Spielschleife mit echten Dialogen verstanden. Jetzt kannst du daraus das komplette Zahlenratespiel bauen.

## Übung: Eigene Verbesserungen

Verändere das Spiel:
- Ändere den Zahlenbereich (z.B. 1-50)
- Verwende auch für die Antworten Dialog-Fenster
- Füge eine Willkommensnachricht am Anfang hinzu
- Füge auch eine Endnachricht hinzu
- *Schwieriger: Füge wenn man gewonnen hat die möglichkeit neu zu starten hinzu*
- *Schwieriger: Zeige nach jedem Gewinn einen Highscore an*

## Zusammenfassung

- `loop { }` ist eine Endlosschleife
- `break` verlässt die Schleife
- `macroquad::rand::srand(...)` initialisiert den Zufallszahlengenerator einmal am Anfang
- `gen_range(a, b)` gibt eine Zufallszahl
- `unwrap()` holt den Wert heraus
- Mit `if-else` reagierst du auf verschiedene Situationen

**Glückwunsch!** Du hast dein erstes richtiges Spiel programmiert! 🎉

Im nächsten Teil des Kurses lernst du fortgeschrittenere Konzepte, um noch komplexere Spiele wie Hangman zu bauen.
