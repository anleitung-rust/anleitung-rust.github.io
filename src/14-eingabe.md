# Eingabe vom Benutzer

Bisher haben alle Werte fest im Code gestanden. Jetzt lernst du, wie der Benutzer selbst etwas eingeben kann!

## Was ist eine Dialogbox?

Eine **Dialogbox** ist ein kleines Fenster, das eine Frage stellt. Du kennst das bestimmt:
- "Wie heißt du?"
- "Gib eine Zahl ein:"

Bevor du sie benutzen kannst, musst du die Bibliothek in deiner `Cargo.toml` ergänzen:

```toml
[dependencies]
rustydialogs = "0.4"
```

## Eine einfache Eingabe

So fragst du nach einem Namen:

```rust
{{#include ../codesamples/examples/eingabe_name.rs:main}}
```

**Was passiert?**
- `TextInput { ... }` erstellt die Frage
- `.show()` zeigt die Box an
- `Some(text)` enthält die Eingabe, `None` bedeutet Abbruch
- Der Benutzer tippt etwas ein
- Wir bekommen den Text zurück

## Eine Zahl eingeben

Oft möchtest du eine Zahl haben:

```rust
{{#include ../codesamples/examples/eingabe_zahl_einfach.rs:main}}
```

Mit `.parse()` wird der Text in eine Zahl umgewandelt.  
Wenn die Eingabe keine Zahl ist, zeigt das Beispiel eine Fehlermeldung an.  
`TextInput::show()` liefert `Option<String>` zurück: `Some(...)` bei gültiger Eingabe oder Abbruch, sonst `None`.

## Übung: Dein Alter

Schreibe ein Programm, das:
1. Nach deinem Namen fragt
2. Nach deinem Alter fragt  
3. Ausgibt: "Hallo [Name], du bist [Alter] Jahre alt!"

<details>
<summary>Lösung</summary>

```rust
use turtle_lib::turtle_main;
use rustydialogs::{MessageBox, MessageButtons, MessageIcon, MessageResult, TextInput, TextInputMode};

#[turtle_main]
fn main() {
    let name = match TextInput {
        title: "Name",
        message: "Wie heißt du?",
        value: "",
        mode: TextInputMode::SingleLine,
        owner: None,
    }
    .show()
    {
        Some(n) => n,
        None => return,
    };

    let alter_text = match TextInput {
        title: "Alter",
        message: "Wie alt bist du?",
        value: "",
        mode: TextInputMode::SingleLine,
        owner: None,
    }
    .show()
    {
        Some(a) => a,
        None => return,
    };

    let alter: u32 = match alter_text.parse() {
        Ok(value) => value,
        Err(_) => {
            let _ = MessageBox {
                title: "Fehler",
                message: "Bitte gib eine gültige Zahl ein!",
                icon: MessageIcon::Warning,
                buttons: MessageButtons::Ok,
                owner: None,
            }
            .show();
            return;
        }
    };

    let text = format!("Hallo {}, du bist {} Jahre alt!", name, alter);
    let _ = MessageBox {
        title: "Ergebnis",
        message: &text,
        icon: MessageIcon::Info,
        buttons: MessageButtons::Ok,
        owner: None,
    }
    .show();
}
```
</details>

## Eine Nachricht anzeigen

Manchmal willst du nur etwas mitteilen:

```rust
use rustydialogs::{MessageBox, MessageButtons, MessageIcon};

fn main() {
    let _ = MessageBox {
        title: "Hallo",
        message: "Willkommen zu meinem Programm!",
        icon: MessageIcon::Info,
        buttons: MessageButtons::Ok,
        owner: None,
    }
    .show();
}
```

## Ja/Nein-Frage

Du kannst auch Ja/Nein fragen:

```rust
use rustydialogs::{MessageBox, MessageButtons, MessageIcon, MessageResult};

fn main() {
    match MessageBox {
        title: "Frage",
        message: "Möchtest du fortfahren?",
        icon: MessageIcon::Question,
        buttons: MessageButtons::YesNo,
        owner: None,
    }
    .show()
    {
        Some(MessageResult::Yes) => {
            println!("Los geht's!");
        }
        _ => {
            println!("Okay, tschüss!");
        }
    }
}
```

## Übung: Ja/Nein mit Turtle

Erstelle ein Programm, das:
1. Fragt: "Möchtest du ein Quadrat sehen?"
2. Bei Ja: Zeichnet ein Quadrat mit der Turtle
3. Bei Nein: Gibt "Okay, bis dann!" aus

## Zusammenfassung

- `TextInput { ... }` fragt nach Text
- `MessageBox { ... }` zeigt eine Nachricht
- `MessageButtons::YesNo` fragt Ja/Nein
- `.parse()` wandelt Text in Zahl um
- `Option::Some(...)` bedeutet eine gültige Eingabe, `None` bedeutet Abbruch

Im nächsten Kapitel baust du dein erstes richtiges Spiel mit Schleifen und Eingaben!
