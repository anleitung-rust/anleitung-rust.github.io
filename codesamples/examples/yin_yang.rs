//! Aufgabe: Yin und Yang
//!
//! Dieses Programm zeichnet das klassische Yin-Yang Symbol.

use turtle_lib::*;

#[turtle_main]
fn main() {
    turtle.set_speed(1000);
    // Schwarze Hälfte
    turtle.circle_right(100.0, 180.0, 36);
    turtle.circle_right(50.0, 180.0, 36);
    turtle.circle_left(50.0, 180.0, 36);

    turtle.circle_left(100.0, 180.0, 36);

    turtle.left(90.0);
    turtle.pen_up();
    turtle.forward(45.0);
    turtle.right(90.0);
    turtle.pen_down();
    turtle.circle_left(5.0, 360.0, 12);

    turtle.left(90.0);
    turtle.pen_up();
    turtle.forward(100.0);
    turtle.right(90.0);
    turtle.pen_down();
    turtle.circle_left(5.0, 360.0, 12);
}
