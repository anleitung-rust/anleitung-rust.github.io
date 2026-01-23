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
use macroquad::rand::gen_range;

fn main() {
    let geheime_zahl = gen_range(0, 101);  // 0 bis 100
    println!("Ich habe mir eine Zahl ausgedacht!");
}
```

### Schritt 2: Nach einer Zahl fragen

Jetzt fragen wir den Spieler:

```rust
use dialog::DialogBox;
use macroquad::rand::gen_range;

fn main() {
    let geheime_zahl = gen_range(0, 101);
    
    let eingabe = dialog::Input::new("Rate die Zahl (0-100):")
        .title("Ratespiel")
        .show()
        .unwrap()
        .unwrap();
    
    let tipp: i32 = eingabe.parse().unwrap();
    
    println!("Du hast {} geraten", tipp);
}
```

### Schritt 3: Prüfen, ob richtig

Jetzt vergleichen wir:

```rust
if tipp == geheime_zahl {
    println!("Richtig!");
} else if tipp < geheime_zahl {
    println!("Zu klein!");
} else {
    println!("Zu groß!");
}
```

### Schritt 4: Die Schleife

Damit man mehrmals raten kann, packen wir alles in eine Schleife:

```rust
use dialog::DialogBox;
use macroquad::rand::gen_range;

fn main() {
    let geheime_zahl = gen_range(0, 101);
    
    loop {
        let eingabe = dialog::Input::new("Rate die Zahl (0-100):")
            .title("Ratespiel")
            .show()
            .unwrap()
            .unwrap();
        
        let tipp: i32 = eingabe.parse().unwrap();
        
        if tipp == geheime_zahl {
            let _ = dialog::Message::new(&format!("Richtig! Die Zahl war {}!", geheime_zahl))
                .title("Gewonnen!")
                .show();
            break;  // Schleife verlassen
        } else if tipp < geheime_zahl {
            let _ = dialog::Message::new("Zu klein! Versuche eine größere Zahl.")
                .title("Hinweis")
                .show();
        } else {
            let _ = dialog::Message::new("Zu groß! Versuche eine kleinere Zahl.")
                .title("Hinweis")
                .show();
        }
    }
    
    println!("Spiel beendet!");
}
```

**Das komplette Spiel!** Probier es aus:
1. Der Computer denkt sich eine Zahl aus
2. Du rätst
3. Der Computer sagt "zu klein" oder "zu groß"
4. Du rätst wieder, bis du richtig liegst

## Erweiterung: Versuche zählen

Füge einen Zähler hinzu:

```rust
let mut versuche = 0;

loop {
    versuche += 1;
    
    // ... (der Rest wie oben)
    
    if tipp == geheime_zahl {
        let _ = dialog::Message::new(&format!("Richtig in {} Versuchen!", versuche))
            .title("Gewonnen!")
            .show();
        break;
    }
    // ...
}
```

## Übung: Eigene Verbesserungen

Verändere das Spiel:
- Ändere den Zahlenbereich (z.B. 1-50)
- Zähle die Versuche und zeige sie an
- Füge eine Willkommensnachricht am Anfang hinzu

## Zusammenfassung

- `loop { }` ist eine Endlosschleife
- `break` verlässt die Schleife
- `gen_range(a, b)` gibt eine Zufallszahl
- `unwrap()` holt den Wert heraus
- Mit `if-else` reagierst du auf verschiedene Situationen

**Glückwunsch!** Du hast dein erstes richtiges Spiel programmiert! 🎉

Im nächsten Teil des Kurses lernst du fortgeschrittenere Konzepte, um noch komplexere Spiele wie Hangman zu bauen.
