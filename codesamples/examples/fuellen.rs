//! Beispiel: Fläche ausfüllen
//!
//! Dieses Programm zeichnet ein ausgefülltes Dreieck.

use turtle_lib::*;

#[turtle_main]
fn main() {
    // Setze die Füllfarbe
    turtle.set_fill_color(BLUE);
    turtle.set_pen_color(BLACK);

    // Beginne mit dem Füllen
    turtle.begin_fill();

    // Zeichne ein Dreieck
    turtle.forward(100.0);
    turtle.left(120.0);
    turtle.forward(100.0);
    turtle.left(120.0);
    turtle.forward(100.0);
    turtle.left(120.0);

    // Beende das Füllen
    turtle.end_fill();
}
