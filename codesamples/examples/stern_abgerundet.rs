//! Aufgabe: Stern mit abgerundeten Spitzen
//!
//! Dieses Programm zeichnet einen Stern mit schönen abgerundeten Spitzen.

use turtle_lib::*;

#[turtle_main]
fn main() {
    turtle.set_pen_color(GOLD);

    for _ in 0..5 {
        // Abgerundete Spitze
        turtle.circle_right(15.0, 144.0, 20);

        // Linie zur nächsten Spitze
        turtle.forward(120.0);
    }
}
