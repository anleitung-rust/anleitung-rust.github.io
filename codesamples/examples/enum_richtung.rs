//! Beispiel: Turtle mit Richtung
//!
//! Verwendet einen Enum für Richtungen.

use turtle_lib::*;

enum Richtung {
    Oben,
    Rechts,
    Unten,
    Links,
}

#[turtle_main]
fn main() {
    // ANCHOR: main
    let richtung = Richtung::Rechts;
    
    match richtung {
        Richtung::Oben => turtle.right(0.0),
        Richtung::Rechts => turtle.right(90.0),
        Richtung::Unten => turtle.right(180.0),
        Richtung::Links => turtle.right(270.0),
    };
    
    turtle.forward(100.0);
    // ANCHOR_END: main
}
