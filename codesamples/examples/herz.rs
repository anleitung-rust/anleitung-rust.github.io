//! Aufgabe: Ein Herz
//!
//! Dieses Programm zeichnet ein rotes Herz.

use turtle_lib::*;

#[turtle_main]
fn main() {
    turtle.set_pen_color(RED);

    // Untere spitze
    turtle.left(45.0);
    turtle.forward(80.0);
    // Oberer linker Bogen
    turtle.circle_left(40.0, 180.0, 36);
    turtle.right(90.0);

    // Oberer rechter Bogen
    turtle.circle_left(40.0, 180.0, 36);

    turtle.forward(80.0);
}
