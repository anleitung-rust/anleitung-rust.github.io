# Eingabe vom Benutzer

In den letzten Kapiteln hast du gelernt, wie du mit Daten arbeitest – Strings, Vektoren, Structs und Enums. Aber bisher haben wir nur mit festen Werten gearbeitet, die schon im Code stehen. Richtig spannend wird es, wenn der Benutzer selbst Daten eingeben kann! In diesem Kapitel lernst du, wie du mit **Dialogboxen** Eingaben vom Benutzer bekommst.

## Was sind Dialogboxen?

Eine **Dialogbox** (oder Dialog) ist ein kleines Fenster, das erscheint und den Benutzer um eine Eingabe bittet. Du kennst das sicher von vielen Programmen:
- "Wie heißt du?"
- "Gib eine Zahl ein:"
- "Möchtest du speichern? Ja/Nein"

Stell dir vor:
- Eine Dialogbox ist wie eine Frage, die du jemandem stellst
- Der Benutzer antwortet, indem er etwas eingibt
- Dein Programm bekommt die Antwort und kann damit weiterarbeiten

## Eine einfache Eingabe

Die einfachste Form ist eine Texteingabe. So fragst du nach einem Namen:

```rust
{{#include ../codesamples/examples/eingabe_einfach.rs}}
```

**Was passiert hier?**
- `dialog::Input::new("...")` erstellt eine neue Eingabebox mit der Frage
- `.title("...")` setzt den Titel des Fensters
- `.show()` zeigt die Box an und wartet auf Eingabe
- Der Benutzer gibt etwas ein und drückt OK oder Abbrechen

## Mit der Eingabe arbeiten

Die Eingabe kommt als `Result` zurück. Das kennst du schon aus dem Enums-Kapitel! Wir müssen prüfen, ob die Eingabe erfolgreich war:

```rust
{{#include ../codesamples/examples/eingabe_verarbeiten.rs}}
```

Hier unterscheiden wir drei Fälle:
- `Ok(Some(eingabe))` - Benutzer hat etwas eingegeben
- `Ok(None)` - Benutzer hat abgebrochen
- `Err(_)` - Es gab einen Fehler

## Eine Zahl eingeben

Oft möchtest du eine Zahl vom Benutzer haben. Dafür musst du den Text in eine Zahl umwandeln:

```rust
{{#include ../codesamples/examples/eingabe_zahl.rs}}
```

Mit `.parse()` wandelst du einen String in eine Zahl um. Das kann schiefgehen (z.B. wenn der Benutzer "abc" eingibt), deshalb verwenden wir `match`!

## Eingabe validieren

Manchmal möchtest du nur bestimmte Eingaben akzeptieren. Zum Beispiel nur Buchstaben oder nur Zahlen in einem bestimmten Bereich:

```rust
{{#include ../codesamples/examples/eingabe_validieren.rs}}
```

Hier prüfen wir:
- Ist genau ein Zeichen eingegeben?
- Ist es ein Buchstabe?

Wenn nicht, fragen wir erneut!

## Mehrere Eingaben nacheinander

Du kannst mehrere Dialoge hintereinander zeigen:

```rust
{{#include ../codesamples/examples/eingabe_mehrfach.rs}}
```

Das ist praktisch für Formulare oder Spiele mit mehreren Fragen!

## Nachrichten anzeigen

Manchmal möchtest du dem Benutzer nur etwas mitteilen, ohne eine Eingabe zu erwarten:

```rust
{{#include ../codesamples/examples/nachricht_anzeigen.rs}}
```

Mit `dialog::Message` zeigst du eine einfache Nachricht an.

## Ja/Nein-Frage

Für Entscheidungen kannst du eine Ja/Nein-Frage stellen:

```rust
{{#include ../codesamples/examples/eingabe_janein.rs}}
```

Mit `dialog::Question` bekommst du ein `true` (Ja) oder `false` (Nein) zurück.

## Eingabe mit Standardwert

Du kannst einen Standardwert vorgeben, der schon im Eingabefeld steht:

```rust
{{#include ../codesamples/examples/eingabe_standard.rs}}
```

Mit `.default("...")` wird das Eingabefeld mit einem Wert vorausgefüllt.

## Fehlerbehandlung

Was tun, wenn die Eingabe ungültig ist? Hier ein robustes Beispiel:

```rust
{{#include ../codesamples/examples/eingabe_robust.rs}}
```

Diese Funktion:
- Fragt nach einer Zahl
- Prüft, ob die Eingabe eine gültige Zahl ist
- Fragt bei Fehler erneut
- Gibt nicht auf, bis eine gültige Zahl eingegeben wurde

## Anwendung: Interaktives Ratespiel

Jetzt können wir ein einfaches Ratespiel erstellen! Der Computer denkt sich eine Zahl aus, und du musst sie erraten:

```rust
{{#include ../codesamples/examples/eingabe_ratespiel.rs}}
```

Hier passiert:
1. Computer wählt zufällig eine Zahl (1-10)
2. Spieler gibt eine Zahl ein
3. Programm sagt, ob richtig oder falsch
4. Bei Fehler bekommt man einen Hinweis (zu hoch/zu niedrig)

## Zusammenfassung

Du hast gelernt:
- `dialog::Input::new("...")` - Erstellt eine Eingabebox
- `.title("...")` - Setzt den Fenstertitel
- `.default("...")` - Setzt einen Standardwert
- `.show()` - Zeigt den Dialog an
- `dialog::Message` - Zeigt eine Nachricht an
- `dialog::Question` - Stellt eine Ja/Nein-Frage
- `.parse()` - Wandelt Text in eine Zahl um
- `Result` und `Option` - Behandeln Erfolg/Fehler und Vorhanden/Nicht-Vorhanden
- Validierung - Prüfen, ob die Eingabe gültig ist

## Übungsaufgaben

### Aufgabe 1: Persönliche Begrüßung

Erstelle ein Programm, das:
- Nach dem Namen fragt
- Nach dem Alter fragt
- Eine persönliche Begrüßung ausgibt: "Hallo Max, du bist 12 Jahre alt!"

### Aufgabe 2: Rechentrainer

Erstelle ein Programm, das:
- Zwei zufällige Zahlen wählt (z.B. 5 und 3)
- Fragt: "Was ist 5 + 3?"
- Die Antwort prüft und "Richtig!" oder "Falsch!" sagt

**Tipp:** Verwende `macroquad::rand::gen_range(1, 11)` für Zufallszahlen.

### Aufgabe 3: Passwort-Check

Erstelle ein Programm, das:
- Nach einem Passwort fragt (z.B. "geheim123")
- Die Eingabe mit dem richtigen Passwort vergleicht
- "Zugang gewährt" oder "Zugang verweigert" anzeigt
- Bei falscher Eingabe maximal 3 Versuche erlaubt

### Aufgabe 4: Zahlenraten verbessert

Verbessere das Ratespiel:
- Zahl zwischen 1 und 100 (statt 1-10)
- Zähle die Anzahl der Versuche
- Zeige am Ende: "Du hast X Versuche gebraucht!"
- Frage, ob der Spieler nochmal spielen möchte

### Aufgabe 5: Mini-Quiz

Erstelle ein Quiz mit 3 Fragen:
- Frage 1: "Wie viele Beine hat eine Spinne?"
- Frage 2: "Wie viele Kontinente gibt es?"
- Frage 3: "Wie viele Stunden hat ein Tag?"

Zähle die richtigen Antworten und zeige am Ende das Ergebnis: "Du hast 2 von 3 richtig!"

### Aufgabe 6: Lieblingsfarbe

Erstelle ein Programm, das:
- Nach der Lieblingsfarbe fragt
- Mit der Turtle ein Quadrat in dieser Farbe zeichnet (wenn möglich)
- Wenn die Farbe nicht erkannt wird, in Rot zeichnet

**Tipp:** Verwende `.to_lowercase()` und vergleiche mit "rot", "grün", "blau" usw.

## Wichtige Hinweise

1. **Immer Fehler behandeln**: Benutzer können alles eingeben – sei vorbereitet!
2. **Klare Fragen stellen**: "Gib eine Zahl ein" ist besser als "Eingabe"
3. **Validierung**: Prüfe, ob die Eingabe sinnvoll ist
4. **Feedback geben**: Sage dem Benutzer, was schiefgelaufen ist
5. **Schleife bei Fehlern**: Frage erneut, statt das Programm abstürzen zu lassen

## Tipps für gute Dialoge

1. **Kurze, klare Fragen**: "Wie heißt du?" statt "Könntest du mir vielleicht sagen, wie dein Name lautet?"
2. **Hilfreichе Titel**: Der Titel sollte zeigen, worum es geht
3. **Standardwerte**: Wenn sinnvoll, gib einen Vorschlag
4. **Fehlermeldungen**: Sage genau, was falsch war: "Bitte nur Zahlen eingeben!" statt "Fehler!"

## Was kommt als Nächstes?

Du kannst jetzt mit dem Benutzer interagieren! Im nächsten Kapitel lernst du **Spielschleifen** kennen – damit kannst du ein komplettes Spiel erstellen, das mehrere Runden läuft und auf Eingaben reagiert. Zusammen mit den Dialogen wird das richtig spannend!
