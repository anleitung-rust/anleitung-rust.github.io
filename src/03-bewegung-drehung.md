# Bewegung und Drehung

Die Schildkröte kann sich auf verschiedene Arten bewegen und drehen. In diesem Kapitel lernst du alle wichtigen Bewegungsbefehle kennen.

## Vorwärts und rückwärts gehen

Die Schildkröte kann sich vorwärts und rückwärts bewegen:

### Vorwärts gehen

```rust
turtle.forward(100.0);
```

Dieser Befehl bewegt die Schildkröte 100 Schritte vorwärts in die Richtung, in die sie gerade schaut.

### Rückwärts gehen

```rust
turtle.backward(50.0);
```

Dieser Befehl bewegt die Schildkröte 50 Schritte rückwärts, ohne dass sie sich umdreht.

### Beispiel: Vor und zurück

Hier ist ein Programm, das beides zeigt:

```rust
{{#include ../codesamples/examples/vorwaerts_rueckwaerts.rs}}
```

Die Schildkröte geht zuerst 100 Schritte vorwärts, dann 50 Schritte zurück. Die Linie wird rot sein.

## Nach links und rechts drehen

Die Schildkröte kann sich drehen, ohne sich zu bewegen:

### Nach links drehen

```rust
turtle.left(90.0);
```

Dreht die Schildkröte 90 Grad nach links (gegen den Uhrzeigersinn).

### Nach rechts drehen

```rust
turtle.right(90.0);
```

Dreht die Schildkröte 90 Grad nach rechts (im Uhrzeigersinn).

## Was sind Grade?

Grade sind ein Maß für Winkel. Ein voller Kreis hat 360 Grad:
- 90 Grad = ein rechter Winkel (ein Viertel eines Kreises)
- 180 Grad = ein halber Kreis (die Schildkröte schaut in die entgegengesetzte Richtung)
- 360 Grad = ein ganzer Kreis (zurück zur Ausgangsrichtung)

### Beispiel: Drehen und zeichnen

```rust
{{#include ../codesamples/examples/drehen.rs}}
```

In diesem Programm:
1. Die Schildkröte geht vorwärts und zeichnet eine grüne Linie
2. Sie dreht sich 90 Grad nach links
3. Sie zeichnet eine weitere Linie
4. Sie dreht sich 45 Grad nach rechts
5. Sie zeichnet eine dritte Linie

So entsteht eine interessante Form!

## Die Startposition der Schildkröte

Wenn ein Programm startet:
- Die Schildkröte ist in der Mitte des Bildschirms
- Sie schaut nach rechts (das ist 0 Grad)
- Der Stift ist unten (sie zeichnet also)

## Zusammenfassung

Du hast gelernt:
- `forward(schritte)` - bewegt die Schildkröte vorwärts
- `backward(schritte)` - bewegt die Schildkröte rückwärts
- `left(grad)` - dreht die Schildkröte nach links
- `right(grad)` - dreht die Schildkröte nach rechts
- Grade messen Winkel (90° = rechter Winkel, 360° = voller Kreis)

Im nächsten Kapitel lernst du, wie du Befehle wiederholen kannst, um interessante Formen zu zeichnen.
