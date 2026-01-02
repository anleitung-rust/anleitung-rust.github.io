//! Beispiel: Mehrere Formen
//!
//! Dieses Programm zeichnet mehrere verschiedene Formen.

use turtle_lib::*;

fn zeichne_dreieck(turtle: &mut TurtlePlan, groesse: f32) {
    for _ in 0..3 {
        turtle.forward(groesse);
        turtle.left(120.0);
    }
}

fn zeichne_quadrat(turtle: &mut TurtlePlan, groesse: f32) {
    for _ in 0..4 {
        turtle.forward(groesse);
        turtle.right(90.0);
    }
}

#[turtle_main]
fn main() {
    // Zeichne ein Dreieck
    turtle.set_pen_color(RED);
    zeichne_dreieck(turtle, 80.0);
    
    // Bewege zur nächsten Position
    turtle.pen_up();
    turtle.forward(150.0);
    turtle.pen_down();
    
    // Zeichne ein Quadrat
    turtle.set_pen_color(BLUE);
    zeichne_quadrat(turtle, 70.0);
}
