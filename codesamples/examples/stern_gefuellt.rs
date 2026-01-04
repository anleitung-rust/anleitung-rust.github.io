//! Aufgabe: Ein ausgefüllter Stern
//!
//! Dieses Programm zeichnet einen fünfzackigen Stern und füllt ihn mit Farbe aus.

use turtle_lib::*;

#[turtle_main]
fn main() {
    // Setze die Füllfarbe und Stiftfarbe
    turtle.set_fill_color(GOLD);
    turtle.set_pen_color(ORANGE);

    // Beginne mit dem Füllen
    turtle.begin_fill();

    // Zeichne einen Stern mit 5 Zacken
    for _ in 0..5 {
        turtle.forward(100.0);
        turtle.right(144.0); // 720 / 5 = 144 Grad für einen Stern
    }

    // Beende das Füllen
    turtle.end_fill();
}
