# Spielschleifen und Zustandsmaschinen

Im letzten Kapitel hast du gelernt, wie du Eingaben vom Benutzer bekommst. Jetzt wird es richtig spannend! In diesem Kapitel lernst du, wie du ein **komplettes Spiel** mit mehreren Runden erstellst. Das Herzstück jedes Spiels ist die **Spielschleife** – sie sorgt dafür, dass das Spiel läuft, bis es vorbei ist.

## Was ist eine Spielschleife?

Eine **Spielschleife** (Game Loop) ist eine Schleife, die immer wieder dieselben Schritte ausführt:

1. **Eingabe** - Was macht der Spieler?
2. **Aktualisieren** - Spiel-Zustand ändern
3. **Prüfen** - Ist das Spiel vorbei?
4. **Wiederholen** - Zurück zu Schritt 1 (wenn nicht vorbei)

Stell dir vor:
- Wie ein Karussell, das sich dreht
- Jede Runde: Spieler macht einen Zug, Spiel reagiert
- Das Karussell stoppt, wenn das Spiel vorbei ist

```
┌──────────────────────┐
│   Spielschleife      │
│                      │
│  1. Eingabe          │
│  2. Aktualisieren    │
│  3. Prüfen           │
│  4. Wiederholen?     │
└──────────────────────┘
```

## Eine einfache Spielschleife

Hier ist die Grundstruktur:

```rust
{{#include ../codesamples/examples/spielschleife_einfach.rs}}
```

Diese Schleife läuft 5 Runden. In jedem Durchlauf:
- Zeigen wir die Rundennummer
- Warten auf Eingabe
- Zählen weiter

## Spielzustand verwalten

Ein Spiel hat einen **Zustand** - das sind alle wichtigen Informationen:
- Wie viele Punkte hat der Spieler?
- Wie viele Leben sind übrig?
- Welche Runde läuft gerade?

Dafür verwenden wir einen Struct (kennst du aus Kapitel 12!):

```rust
{{#include ../codesamples/examples/spielschleife_zustand.rs}}
```

Hier speichern wir:
- `runde` - Welche Runde läuft
- `punkte` - Wie viele Punkte der Spieler hat
- `leben` - Wie viele Leben noch übrig sind

## Mit Bedingungen arbeiten

Die Spielschleife läuft, **solange** bestimmte Bedingungen erfüllt sind:

```rust
{{#include ../codesamples/examples/spielschleife_bedingung.rs}}
```

Hier läuft das Spiel, solange:
- Der Spieler noch Leben hat (`leben > 0`)
- Und nicht mehr als 10 Runden gespielt wurden

## Zustandsmaschine: Was ist das?

Eine **Zustandsmaschine** (State Machine) ist ein Konzept, bei dem dein Spiel verschiedene Zustände haben kann:
- **Startmenu** - Spiel noch nicht gestartet
- **Läuft** - Spiel ist am Laufen
- **Gewonnen** - Spieler hat gewonnen
- **Verloren** - Spieler hat verloren

Erinnerst du dich an Enums aus Kapitel 13? Perfekt dafür!

```rust
{{#include ../codesamples/examples/spielschleife_zustandsmaschine.rs}}
```

Mit `match` entscheiden wir, was in jedem Zustand passiert!

## Auf Eingaben reagieren

In einer Spielschleife fragst du nach einer Eingabe und reagierst darauf:

```rust
{{#include ../codesamples/examples/spielschleife_eingabe.rs}}
```

Je nach Eingabe passiert etwas anderes:
- "a" - Angreifen
- "v" - Verteidigen
- "f" - Fliehen
- Sonst - Ungültige Eingabe

## Das komplette Zahlenratespiel

Jetzt bauen wir ein vollständiges Ratespiel! Der Computer denkt sich eine Zahl zwischen 0 und 100 aus, und du musst sie erraten:

```rust
{{#include ../codesamples/examples/ratespiel_komplett.rs}}
```

**Was passiert hier?**

1. **Spielzustand erstellen**:
   - Zufällige Zahl zwischen 0 und 100
   - Versuchszähler auf 0
   - Spiel läuft

2. **Spielschleife**:
   - Läuft, solange das Spiel nicht vorbei ist
   - Fragt nach Eingabe
   - Prüft die Zahl
   - Gibt Hinweise (zu groß/zu klein)
   - Zählt Versuche

3. **Spielende**:
   - Bei richtiger Zahl: Gewonnen!
   - Zeigt Anzahl der Versuche

## Verbesserungen hinzufügen

Wir können das Spiel noch besser machen:

### 1. Schwierigkeitsgrade

```rust
{{#include ../codesamples/examples/ratespiel_schwierigkeit.rs}}
```

Der Spieler wählt zu Beginn den Schwierigkeitsgrad!

### 2. Highscore speichern

```rust
{{#include ../codesamples/examples/ratespiel_highscore.rs}}
```

Das Spiel merkt sich, wie viele Versuche du beim besten Spiel gebraucht hast!

### 3. Mehrere Runden

```rust
{{#include ../codesamples/examples/ratespiel_mehrere_runden.rs}}
```

Nach jedem Spiel wird gefragt, ob man nochmal spielen möchte!

## Wichtige Konzepte

### Die while-Schleife

Für Spielschleifen verwenden wir meist `while`:

```rust
while bedingung {
    // Spiel-Code
}
```

Die Schleife läuft, **solange** die Bedingung `true` ist.

### Loop mit Break

Alternativ kannst du `loop` mit `break` verwenden:

```rust
loop {
    // Spiel-Code
    
    if spiel_vorbei {
        break;  // Schleife verlassen
    }
}
```

### Match für Entscheidungen

`match` ist perfekt, um auf verschiedene Situationen zu reagieren:

```rust
match spielzustand {
    Spielzustand::Laeuftnoch => { /* weiterspielen */ }
    Spielzustand::Gewonnen => { /* gewonnen */ }
    Spielzustand::Verloren => { /* verloren */ }
}
```

## Zusammenfassung

Du hast gelernt:
- **Spielschleife** - Das Herzstück jedes Spiels
- `while bedingung { }` - Schleife mit Bedingung
- `loop { }` mit `break` - Endlosschleife mit Ausgang
- **Spielzustand** - Wichtige Informationen in einem Struct
- **Zustandsmaschine** - Verschiedene Zustände mit Enums
- `match` - Auf verschiedene Situationen reagieren
- Das komplette Zahlenratespiel als Beispiel

## Übungsaufgaben

### Aufgabe 1: Münzwurf-Spiel

Erstelle ein Spiel, bei dem:
- Der Computer eine Münze wirft (zufällig Kopf oder Zahl)
- Der Spieler raten muss
- Das Spiel 5 Runden läuft
- Am Ende die Punktzahl angezeigt wird

**Tipp:** Verwende `gen_range(0, 2)` für 0 (Kopf) oder 1 (Zahl).

### Aufgabe 2: Zahlenraten mit Leben

Erweitere das Zahlenratespiel:
- Der Spieler hat 7 Leben
- Bei falschem Tipp: Ein Leben verloren
- Bei 0 Leben: Spiel verloren
- Zeige die verbleibenden Leben nach jedem Tipp

### Aufgabe 3: Mathe-Trainer

Erstelle ein Programm, das:
- Zufällige Mathe-Aufgaben stellt (z.B. "7 + 5 = ?")
- Die Antwort prüft
- Bei richtig: +1 Punkt
- Bei falsch: Richtige Lösung zeigen
- Nach 10 Aufgaben: Punktzahl anzeigen

### Aufgabe 4: Höher oder Tiefer

Erstelle ein Spiel:
- Starte mit Zahl 50
- Frage: "Höher oder Tiefer?"
- Computer wählt zufällig höher (+10) oder tiefer (-10)
- Wenn richtig geraten: +1 Punkt
- Spiele 10 Runden
- Zeige Endscore

### Aufgabe 5: Wort-Raten

Erstelle ein einfaches Wort-Ratespiel:
- Computer wählt ein Wort aus einer Liste
- Zeige nur die Länge: "Das Wort hat 5 Buchstaben"
- Spieler rät das ganze Wort
- Maximal 5 Versuche
- Bei Erfolg: "Gewonnen!", sonst: "Verloren! Das Wort war: ..."

### Aufgabe 6: Text-Abenteuer

Erstelle ein kleines Text-Abenteuer:
- Mehrere Räume (z.B. Wald, Höhle, Burg)
- In jedem Raum: Beschreibung und Entscheidung
- Spieler wählt: "Wohin gehen? (1) Nord, (2) Süd"
- Verschiedene Enden möglich (Schatz gefunden, Monster besiegt, ...)

## Wichtige Hinweise

1. **Immer einen Ausgang**: Stelle sicher, dass die Schleife irgendwann endet!
2. **Zustand klar halten**: Verwende Structs für übersichtlichen Code
3. **Kleine Schritte**: Teste jede Funktion einzeln
4. **Feedback geben**: Sage dem Spieler immer, was passiert ist
5. **Fehler abfangen**: Was, wenn der Spieler "abc" eingibt statt einer Zahl?

## Tipps für gute Spielschleifen

1. **Übersichtlich bleiben**: Teile komplexe Logik in Funktionen auf
2. **Zustand prüfen**: Ist das Spiel vorbei? Prüfe am Anfang der Schleife
3. **Klare Regeln**: Der Spieler muss wissen, was er tun kann
4. **Fortschritt zeigen**: "Versuch 3 von 10" oder "5 Leben übrig"

## Was kommt als Nächstes?

Du kannst jetzt vollständige Spiele mit Schleifen erstellen! In den nächsten Kapiteln lernst du fortgeschrittene Konzepte wie **Threads** (Nebenläufigkeit) kennen. Damit kann dein Programm mehrere Dinge gleichzeitig machen – zum Beispiel Grafiken zeichnen, während das Spiel auf Eingaben wartet. Das brauchen wir für das Hangman-Projekt!
