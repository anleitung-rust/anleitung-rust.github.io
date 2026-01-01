# Einrichtung der Programmierumgebung

Bevor wir mit dem Programmieren beginnen können, müssen wir ein paar Programme auf deinem Computer installieren. Keine Sorge – wir gehen das Schritt für Schritt durch!

## Was brauchen wir?

1. **Rust** – Die Programmiersprache, die wir verwenden
2. **Visual Studio Code** – Ein Programm, in dem wir unseren Code schreiben (wird "Editor" genannt)
3. **Erweiterungen für VS Code** – Zusätzliche Funktionen, die uns beim Programmieren helfen

## Schritt 1: Rust installieren

Rust ist die Programmiersprache, mit der wir arbeiten werden. Zusammen mit Rust wird auch **Cargo** installiert – ein Werkzeug, das uns bei der Verwaltung unserer Programme hilft.

### Installation auf Windows

1. Gehe zu [https://rustup.rs/](https://rustup.rs/)
2. Lade die Installationsdatei herunter
3. Führe die Datei aus und folge den Anweisungen
4. Wenn du gefragt wirst, wähle die Standardinstallation (Option 1)

### Installation auf macOS oder Linux

1. Öffne das Terminal
2. Kopiere diesen Befehl und drücke Enter:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. Folge den Anweisungen auf dem Bildschirm
4. Wähle die Standardinstallation (Option 1)

### Installation überprüfen

Um zu prüfen, ob die Installation geklappt hat:

1. Öffne ein neues Terminal oder eine neue Eingabeaufforderung
2. Tippe `cargo --version` und drücke Enter
3. Du solltest eine Versionsnummer sehen, z.B. `cargo 1.92.0`

## Schritt 2: Visual Studio Code installieren

Visual Studio Code (kurz: VS Code) ist ein kostenloses Programm, in dem wir unseren Code schreiben.

1. Gehe zu [https://code.visualstudio.com/](https://code.visualstudio.com/)
2. Lade VS Code für dein Betriebssystem herunter
3. Installiere das Programm
4. Starte VS Code

## Schritt 3: Wichtige Erweiterungen installieren

Erweiterungen sind wie Apps für VS Code – sie fügen nützliche Funktionen hinzu. Wir brauchen zwei wichtige Erweiterungen:

### rust-analyzer

Diese Erweiterung hilft dir beim Schreiben von Rust-Code. Sie zeigt Fehler an, schlägt Code vor und erklärt Befehle.

**Installation:**
1. Klicke in VS Code auf das Erweiterungs-Symbol in der linken Leiste (vier Quadrate)
2. Suche nach `rust-analyzer`
3. Klicke auf "Installieren"

### CodeLLDB

Diese Erweiterung ermöglicht es, deine Programme zu starten und zu testen.

**Installation:**
1. Suche im Erweiterungs-Bereich nach `CodeLLDB`
2. Klicke auf "Installieren"

## Schritt 4: Projekt einrichten

Jetzt erstellen wir unser erstes Projekt!

### 1. Projekt-Ordner erstellen

Wähle einen Ort auf deinem Computer, wo du deine Programmier-Projekte speichern möchtest, z.B. in deinem Dokumente-Ordner.

### 2. Projekt mit Cargo erstellen

1. Öffne das Terminal in VS Code (Menü: Terminal → Neues Terminal)
2. Gehe zu dem Ordner, wo du dein Projekt erstellen möchtest:
   ```bash
   cd ~/Dokumente
   ```
3. Erstelle ein neues Projekt:
   ```bash
   cargo new mein-turtle-projekt
   cd mein-turtle-projekt
   ```

Cargo hat jetzt automatisch einen Ordner mit allem erstellt, was du brauchst!

### 3. Turtle-Bibliothek hinzufügen

Öffne die Datei `Cargo.toml` in deinem Projekt-Ordner. Das ist die Konfigurationsdatei für dein Projekt.

Füge unter `[dependencies]` folgende Zeilen hinzu:

```toml
[dependencies]
turtle-lib = { git = "https://github.com/enaut/turtlers", package = "turtle-lib" }
macroquad = "0.4"
```

Deine `Cargo.toml` sollte jetzt ungefähr so aussehen:

```toml
[package]
name = "mein-turtle-projekt"
version = "0.1.0"
edition = "2021"

[dependencies]
turtle-lib = { git = "https://github.com/enaut/turtlers", package = "turtle-lib" }
macroquad = "0.4"
```

### 4. Projekt in VS Code öffnen

1. In VS Code: Menü → Datei → Ordner öffnen
2. Wähle den Ordner `mein-turtle-projekt`
3. VS Code lädt jetzt das Projekt

## Schritt 5: Dein erstes Programm schreiben

1. Öffne die Datei `src/main.rs` im Projekt-Ordner
2. Ersetze den Inhalt mit diesem Code:

```rust
use turtle_lib::*;

#[turtle_main]
fn main() {
    turtle.set_pen_color(BLUE);
    turtle.forward(100.0);
    turtle.right(90.0);
    turtle.forward(100.0);
}
```

3. Speichere die Datei (Strg+S oder Cmd+S)

## Schritt 6: Programm starten

Es gibt mehrere Möglichkeiten, dein Programm zu starten:

### Methode 1: Run-Button (empfohlen)

Über der `fn main()` Zeile siehst du kleine Schaltflächen: **Run | Debug**

1. Klicke auf **Run**
2. Warte, während das Programm kompiliert wird (beim ersten Mal dauert es etwas länger)
3. Ein Fenster öffnet sich und zeigt deine Zeichnung!

### Methode 2: Terminal

Im Terminal von VS Code:
```bash
cargo run
```

### Methode 3: Beispiele ausführen

Später werden wir mit Beispielen arbeiten. Diese startest du so:
```bash
cargo run --example quadrat
```

## Was ist Cargo?

**Cargo** ist das Werkzeug, das alle Rust-Entwickler verwenden. Es hilft dir dabei:
- Neue Projekte zu erstellen (`cargo new`)
- Programme zu kompilieren und auszuführen (`cargo run`)
- Bibliotheken zu verwalten (wie unsere turtle-lib)
- Code zu überprüfen (`cargo check`)

## Wo finde ich Rust-Bibliotheken?

Wenn du später weitere Rust-Bibliotheken nutzen möchtest, findest du diese auf:
- **crates.io** – Das offizielle Repository für Rust-Bibliotheken
- Dort kannst du nach Bibliotheken suchen und sie deinem Projekt hinzufügen

## Häufige Probleme

### "rust-analyzer funktioniert nicht"
- Stelle sicher, dass Rust richtig installiert ist (`cargo --version` im Terminal)
- Starte VS Code neu
- Öffne VS Code über "Ordner öffnen" (nicht einzelne Dateien)

### "Das Programm kompiliert nicht"
- Überprüfe, ob `Cargo.toml` die richtigen Dependencies hat
- Schau dir die Fehlermeldungen im Terminal genau an
- rust-analyzer zeigt Fehler auch im Editor mit roten Wellenlinien an

### "Das Fenster öffnet sich nicht"
- Warte einen Moment – beim ersten Mal dauert es länger
- Schau ins Terminal, ob Fehlermeldungen erscheinen

## Zusammenfassung

Du hast jetzt:
- ✅ Rust und Cargo installiert
- ✅ VS Code mit wichtigen Erweiterungen eingerichtet
- ✅ Dein erstes Projekt erstellt
- ✅ Die Turtle-Bibliothek hinzugefügt
- ✅ Gelernt, wie du Programme startest

Jetzt bist du bereit, richtig mit dem Programmieren loszulegen! Im nächsten Kapitel schreiben wir unser erstes richtiges Turtle-Programm.
