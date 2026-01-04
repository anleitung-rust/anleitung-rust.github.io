//! Aufgabe: Ein Käse
//!
//! Dieses Programm zeichnet ein gelbes Käsestück mit drei Löchern.

use turtle_lib::*;

#[turtle_main]
fn main() {
    // Zeichne zuerst das gelbe Viereck (Käse)
    turtle.set_fill_color(YELLOW);
    turtle.set_pen_color(GOLD);

    turtle.begin_fill();
    for _ in 0..4 {
        turtle.forward(120.0);
        turtle.right(90.0);
    }
    turtle.end_fill();

    // Zeichne drei Löcher an verschiedenen Positionen
    zeichne_kreis(turtle, 40.0, -40.0);
    zeichne_kreis(turtle, 80.0, -60.0);
    zeichne_kreis(turtle, 20.0, -90.0);
}
// Funktion zum Zeichnen eines Kreises (Loch)
fn zeichne_kreis(turtle: &mut TurtlePlan, x: f32, y: f32) {
    turtle.pen_up();
    turtle.go_to((x, y));
    turtle.pen_down();

    turtle.set_fill_color(WHITE);
    turtle.set_pen_color(WHITE);

    turtle.begin_fill();
    // Zeichne einen Kreis mit vielen kleinen Seiten
    turtle.circle_right(10.0, 360.0, 36);
    turtle.end_fill();
}
