//! Beispiel: Turtle mit Farben
//!
//! Zeichnet ein Dreieck mit verschiedenen Farben.

use turtle_lib::*;

#[turtle_main]
fn main() {
    // ANCHOR: main
    let farben = vec![RED, GREEN, BLUE];
    for farbe in &farben {
        turtle.set_pen_color(*farbe);
        turtle.forward(100.0);
        turtle.right(120.0);
    }
    // ANCHOR_END: main
}
