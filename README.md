# Rustanleitung - Programmieren lernen mit Rust und Turtle-Grafik

Eine deutschsprachige Einführung in das Programmieren für Schülerinnen und Schüler, basierend auf der Programmiersprache Rust und der Turtle-Grafik-Bibliothek.

## Überblick

Diese Anleitung wurde entwickelt, um Programmier-Anfängern einen einfachen und motivierenden Einstieg in die Welt des Programmierens zu bieten. Durch die Verwendung von Turtle-Grafik können Lernende sofort visuelle Ergebnisse ihrer Programme sehen, was das Verständnis erleichtert und Spaß macht.

## Zielgruppe

- Schülerinnen und Schüler ab der 11. Klasse
- Keine Vorkenntnisse in Programmierung erforderlich
- Alle Inhalte sind auf Deutsch und schülerfreundlich erklärt

## Inhalt

Die Anleitung umfasst folgende Themen:

1. **Einleitung** - Was ist Programmieren und warum ist es wichtig?
2. **Was ist Programmieren?** - Grundlegende Konzepte
3. **Dein erstes Programm** - Ein einfaches Quadrat zeichnen
4. **Bewegung und Drehung** - Die Schildkröte steuern
5. **Wiederholungen (Schleifen)** - Code effizient wiederverwenden
6. **Farben und Stift** - Bunte Zeichnungen erstellen
7. **Variablen** - Werte speichern und verwenden
8. **Funktionen** - Code organisieren und wiederverwenden
9. **Weitere Beispiele** - Kreative Projekte zum Ausprobieren

## Verwendete Technologie

- **Rust** - Eine moderne, sichere Programmiersprache
- **Turtle-Grafik** - Aus der [turtlers](https://github.com/enaut/turtlers) Bibliothek
- **mdBook** - Für die Erstellung der Online-Dokumentation

## Aufbau des Projekts

```
rustanleitung.github.io/
├── src/                    # Markdown-Dateien für die Kapitel
│   ├── einleitung.md
│   ├── 01-was-ist-programmieren.md
│   ├── 02-erstes-programm.md
│   └── ...
├── codesamples/           # Rust-Projekt mit allen Beispielen
│   ├── examples/          # Ausführbare Beispielprogramme
│   │   ├── quadrat.rs
│   │   ├── stern.rs
│   │   └── ...
│   └── Cargo.toml
├── book.toml              # mdBook-Konfiguration
└── README.md              # Diese Datei
```

## Lokale Verwendung

### Voraussetzungen

- Rust (Installation: https://rustup.rs/)
- mdBook (wird automatisch installiert)

### Das Buch bauen

```bash
# mdBook installieren (einmalig)
cargo install mdbook

# Buch erstellen
mdbook build

# Buch mit Live-Reload anschauen
mdbook serve --open
```

Das Buch wird dann unter `http://localhost:3000` verfügbar sein.

### Beispiele ausführen

```bash
cd codesamples

# Ein bestimmtes Beispiel ausführen
cargo run --example quadrat

# Alle Beispiele kompilieren
cargo build --examples

# Liste aller verfügbaren Beispiele
cargo run --example
```

## Verfügbare Beispiele

Die folgenden ausführbaren Beispiele sind verfügbar:

- `quadrat` - Ein einfaches Quadrat
- `vorwaerts_rueckwaerts` - Vorwärts und rückwärts bewegen
- `drehen` - Links und rechts drehen
- `achteck` - Ein Achteck mit Schleife
- `stern` - Ein fünfzackiger Stern
- `farben` - Verschiedene Farben verwenden
- `stift_heben` - Stift heben und senken
- `fuellen` - Formen ausfüllen
- `variablen` - Variablen verwenden
- `mehrere_variablen` - Mehrere Variablen
- `spirale` - Eine Spirale zeichnen
- `funktionen` - Funktionen erstellen und verwenden
- `mehrere_formen` - Verschiedene Formen kombinieren

## Beitragen

Verbesserungsvorschläge und Beiträge sind willkommen! Bitte erstelle ein Issue oder einen Pull Request.

## Inspiration

Diese Anleitung basiert auf dem Konzept von [pythonanleitung.github.io](https://github.com/pythonanleitung/pythonanleitung.github.io), wurde aber für Rust und die moderne Turtle-Grafik-Bibliothek angepasst.

## Lizenz

Diese Anleitung ist offen verfügbar und kann frei verwendet werden.

## Kontakt

Bei Fragen oder Anregungen, bitte erstelle ein Issue in diesem Repository.
