//! Beispiel: Geometrische Übung
//!
//! Dieses Programm zeichnet eine einfache geometrische Form mit verschiedenen Winkeln.

use turtle_lib::*;

#[turtle_main]
fn main() {
    turtle.set_pen_color(BLUE);

    // Zeichne ein Dreieck
    turtle.forward(100.0);
    turtle.right(90.0);
    turtle.forward(100.0);
    turtle.right(90.0);
    turtle.forward(100.0);
    turtle.right(90.0);
    turtle.forward(100.0);

    // Box
    turtle.right(45.0);
    turtle.forward(50.0);
    turtle.left(90.0);
    turtle.forward(100.0);
    turtle.left(90.0);
    turtle.forward(50.0);
    turtle.left(90.0);
    turtle.forward(100.0);

    // Dreieck
    turtle.right(135.0);
    turtle.forward(100.0);
    turtle.left(120.0);
    turtle.forward(100.0);
    turtle.left(120.0);
    turtle.forward(100.0);
    turtle.left(120.0);
}
