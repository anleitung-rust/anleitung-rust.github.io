# Anhang: VS Code installieren (optional)

Zed ist der empfohlene Editor für diese Anleitung. Wenn du lieber mit VS Code arbeitest, kannst du hier die Einrichtung nachlesen.

## Warum VS Code?

Visual Studio Code (kurz: VS Code) ist ein kostenloser und sehr beliebter Editor für Programmieren. Er ist leicht zu benutzen und unterstützt Rust gut mit Erweiterungen.

## Schritt 1: VS Code installieren

1. Gehe zu [https://code.visualstudio.com/](https://code.visualstudio.com/)
2. Lade VS Code für dein Betriebssystem herunter
3. Installiere das Programm
4. Starte VS Code

## Schritt 2: Wichtige Erweiterungen installieren

Erweiterungen sind wie Apps für VS Code – sie fügen nützliche Funktionen hinzu. Für Rust brauchen wir zwei wichtige Erweiterungen:

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

## Projekt in VS Code öffnen

Wenn du ein Projekt starten möchtest:

1. Öffne VS Code
2. Klicke auf **Datei → Ordner öffnen**
3. Wähle dein Rust-Projekt aus
4. VS Code lädt die Dateien und zeigt dir die Rust-Hilfen an

## Häufige Probleme

### "rust-analyzer funktioniert nicht"
- Stelle sicher, dass Rust richtig installiert ist (`cargo --version` im Terminal)
- Starte VS Code neu
- Öffne den Ordner über "Datei → Ordner öffnen" (nicht nur einzelne Dateien)

### "Das Programm kompiliert nicht"
- Überprüfe, ob `Cargo.toml` die richtigen Abhängigkeiten hat
- Schau dir die Fehlermeldungen im Terminal genau an
- `rust-analyzer` zeigt Fehler auch im Editor mit roten Wellenlinien an

### "Das Fenster öffnet sich nicht"
- Warte einen Moment – beim ersten Mal dauert es länger
- Schau ins Terminal, ob Fehlermeldungen auftauchen

## Fazit

VS Code ist ein gutes Alternativ-Setup, aber für diese Anleitung ist Zed der bevorzugte Editor. Wenn du also mit dem empfohlenen Setup starten willst, kannst du direkt zu Kapitel 0 zurückkehren.
