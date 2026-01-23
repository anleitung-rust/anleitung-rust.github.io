//! Beispiel: Turtle mit Farben
//!
//! Zeichnet ein Dreieck mit verschiedenen Farben.

use turtle_lib::*;

#[turtle_main]
fn main() {
    // ANCHOR: main
    turtle.set_pen_color(RED);
    turtle.forward(50.0);
    turtle.right(120.0);
    
    turtle.set_pen_color(GREEN);
    turtle.forward(50.0);
    turtle.right(120.0);
    
    turtle.set_pen_color(BLUE);
    turtle.forward(50.0);
    turtle.right(120.0);
    // ANCHOR_END: main
}
