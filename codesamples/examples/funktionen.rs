//! Beispiel: Funktionen verwenden
//!
//! Dieses Programm zeigt, wie man eigene Funktionen erstellt.

use turtle_lib::*;

// Eine Funktion, die ein Quadrat zeichnet
fn zeichne_quadrat(turtle: &mut TurtlePlan, groesse: f32) {
    for _ in 0..4 {
        turtle.forward(groesse);
        turtle.right(90.0);
    }
}

#[turtle_main]
fn main() {
    turtle.set_pen_color(BLUE);
    
    // Zeichne ein kleines Quadrat
    zeichne_quadrat(turtle, 50.0);
    
    // Bewege die Schildkröte
    turtle.pen_up();
    turtle.forward(120.0);
    turtle.pen_down();
    
    // Zeichne ein größeres Quadrat
    turtle.set_pen_color(RED);
    zeichne_quadrat(turtle, 80.0);
}
