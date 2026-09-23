//! Beispiel: Turtle-Quadrat mit Farbe
//!
//! Zeichnet ein Quadrat in gewählter Farbe.

use turtle_lib::*;

enum Farbe {
    Rot,
    Gruen,
    Blau,
}

#[turtle_main]
fn main() {
    // ANCHOR: main
    let farbe = Farbe::Gruen;  // Ändere das!
    
    match farbe {
        Farbe::Rot => turtle.set_pen_color(RED),
        Farbe::Gruen => turtle.set_pen_color(GREEN),
        Farbe::Blau => turtle.set_pen_color(BLUE),
    };
    
    // Zeichne Quadrat
    for _ in 0..4 {
        turtle.forward(80.0);
        turtle.right(90.0);
    }
    // ANCHOR_END: main
}
