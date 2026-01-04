//! Aufgabe: Eine Blume
//!
//! Dieses Programm zeichnet eine bunte Blume mit mehreren Blütenblättern.

use turtle_lib::*;

#[turtle_main]
fn main() {
    turtle.set_pen_color(GOLD);

    // Zeichne 5 Blütenblätter
    for _ in 0..5 {
        // Oberer Bogen des Blütenblatts
        turtle.circle_left(40.0, 360.0, 36);
        // Drehe dich für das nächste Blütenblatt
        turtle.right(72.0); // 360 / 5 = 72 Grad
    }

    // Zeichne die Blütenmitte (kleine gelbe Kugel)
    turtle.pen_up();
    turtle.forward(20.0);
    turtle.pen_down();

    turtle.set_pen_color(YELLOW);
    turtle.right(90.0);

    turtle.circle_right(20.0, 360.0, 36);
}
