//! Beispiel: Stift heben und senken
//!
//! Dieses Programm zeigt, wie man den Stift hebt und senkt.

use turtle_lib::*;

#[turtle_main]
fn main() {
    turtle.set_pen_color(RED);
    
    // Zeichne eine Linie
    turtle.forward(50.0);
    
    // Hebe den Stift (keine Linie mehr)
    turtle.pen_up();
    turtle.forward(50.0);
    
    // Senke den Stift wieder (zeichne wieder)
    turtle.pen_down();
    turtle.forward(50.0);
}
