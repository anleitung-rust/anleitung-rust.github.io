//! Beispiel: Drei Rechtecke
//!
//! Zeichnet drei Rechtecke nebeneinander.

use turtle_lib::*;

struct Rechteck {
    breite: f32,
    hoehe: f32,
}

#[turtle_main]
fn main() {
    // ANCHOR: main
    let rechtecke = vec![
        Rechteck { breite: 50.0, hoehe: 30.0 },
        Rechteck { breite: 70.0, hoehe: 40.0 },
        Rechteck { breite: 90.0, hoehe: 50.0 },
    ];
    
    for rechteck in &rechtecke {
        // Zeichne Rechteck
        for _ in 0..2 {
            turtle.forward(rechteck.breite);
            turtle.right(90.0);
            turtle.forward(rechteck.hoehe);
            turtle.right(90.0);
        }
        
        // Bewege nach rechts
        turtle.pen_up();
        turtle.forward(rechteck.breite + 20.0);
        turtle.pen_down();
    }
    // ANCHOR_END: main
}
