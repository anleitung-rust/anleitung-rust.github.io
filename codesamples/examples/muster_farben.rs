//! Aufgabe: Muster mit verschiedenen Farben
//!
//! Dieses Programm zeichnet mehrere Quadrate in unterschiedlichen Farben.

use turtle_lib::*;

#[turtle_main]
fn main() {
    let quadrate = [(50.0, RED), (90.0, BLUE), (130.0, GREEN)];

    for (groesse, farbe) in quadrate {
        turtle.pen_up();
        turtle.go_to((-groesse / 2.0, groesse / 2.0));
        turtle.set_heading(0.0);
        turtle.pen_down();
        turtle.set_pen_color(farbe);

        for _ in 0..4 {
            turtle.forward(groesse);
            turtle.right(90.0);
        }
    }
}
