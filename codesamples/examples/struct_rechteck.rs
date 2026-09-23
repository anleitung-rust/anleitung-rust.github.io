//! Beispiel: Rechteck mit Struct
//!
//! Verwendet einen Struct für Rechteck-Eigenschaften.

use turtle_lib::*;

struct Rechteck {
    breite: f32,
    hoehe: f32,
}

#[turtle_main]
fn main() {
    // ANCHOR: main
    let rechteck = Rechteck {
        breite: 100.0,
        hoehe: 60.0,
    };
    
    // Zeichne das Rechteck
    for _ in 0..2 {
        turtle.forward(rechteck.breite);
        turtle.right(90.0);
        turtle.forward(rechteck.hoehe);
        turtle.right(90.0);
    }
    // ANCHOR_END: main
}
