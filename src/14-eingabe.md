# Eingabe vom Benutzer

Bisher haben alle Werte fest im Code gestanden. Jetzt lernst du, wie der Benutzer selbst etwas eingeben kann!

## Was ist eine Dialogbox?

Eine **Dialogbox** ist ein kleines Fenster, das eine Frage stellt. Du kennst das bestimmt:
- "Wie heißt du?"
- "Gib eine Zahl ein:"

## Eine einfache Eingabe

So fragst du nach einem Namen:

```rust
{{#include ../codesamples/examples/eingabe_name.rs:main}}
```

**Was passiert?**
- `dialog::Input::new("...")` erstellt die Frage
- `.show()` zeigt die Box an
- Der Benutzer tippt etwas ein
- Wir bekommen den Text zurück

## Eine Zahl eingeben

Oft möchtest du eine Zahl haben:

```rust
{{#include ../codesamples/examples/eingabe_zahl.rs}}
```

Mit `.parse()` wird der Text in eine Zahl umgewandelt.  
Mit `.unwrap()` sagen wir "Das muss klappen!" - wenn der Text keine Zahl ist, stoppt das Programm.

## Übung: Dein Alter

Schreibe ein Programm, das:
1. Nach deinem Namen fragt
2. Nach deinem Alter fragt  
3. Ausgibt: "Hallo [Name], du bist [Alter] Jahre alt!"

<details>
<summary>Lösung</summary>

```rust
use dialog::DialogBox;

fn main() {
    // Frage nach Name
    let name = match dialog::Input::new("Wie heißt du?")
        .title("Name")
        .show()
    {
        Ok(Some(n)) => n,
        _ => return,
    };
    
    // Frage nach Alter
    let alter_text = match dialog::Input::new("Wie alt bist du?")
        .title("Alter")
        .show()
    {
        Ok(Some(a)) => a,
        _ => return,
    };
    
    let alter: u32 = alter_text.parse().unwrap();
    
    // Zeige Ergebnis
    println!("Hallo {}, du bist {} Jahre alt!", name, alter);
}
```
</details>

## Eine Nachricht anzeigen

Manchmal willst du nur etwas mitteilen:

```rust
use dialog::DialogBox;

fn main() {
    let _ = dialog::Message::new("Willkommen zu meinem Programm!")
        .title("Hallo")
        .show();
}
```

## Ja/Nein-Frage

Du kannst auch Ja/Nein fragen:

```rust
use dialog::DialogBox;

fn main() {
    match dialog::Question::new("Möchtest du fortfahren?")
        .title("Frage")
        .show()
    {
        Ok(dialog::Choice::Yes) => {
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

- `dialog::Input` fragt nach Text
- `dialog::Message` zeigt eine Nachricht
- `dialog::Question` fragt Ja/Nein
- `.parse()` wandelt Text in Zahl um
- `.unwrap()` sagt "Muss klappen!"

Im nächsten Kapitel baust du dein erstes richtiges Spiel mit Schleifen und Eingaben!
