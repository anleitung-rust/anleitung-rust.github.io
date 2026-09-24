//! Aufgabe: Ein Regenbogen
//!
//! Dieses Programm zeichnet einen Regenbogen aus mehreren Halbkreisen in
//! verschiedenen Farben.

use turtle_lib::*;

#[turtle_main]
fn main() {
    turtle.set_pen_width(8.0);

    let farben = [RED, ORANGE, YELLOW, GREEN, BLUE, PURPLE];
    let radien = [80.0, 100.0, 120.0, 140.0, 160.0, 180.0];

    for (radius, farbe) in radien.iter().zip(farben.iter()) {
        turtle.pen_up();
        turtle.go_to((0.0, -100.0));
        turtle.set_heading(0.0);
        turtle.backward(*radius);
        turtle.left(90);
        turtle.pen_down();
        turtle.set_pen_color(*farbe);
        turtle.circle_right(*radius, 180.0, 36);
    }
}
