//! Beispiel: Drehen
//!
//! Dieses Programm zeigt, wie man die Schildkröte dreht.

use turtle_lib::*;

#[turtle_main]
fn main() {
    turtle.set_pen_color(GREEN);
    
    // Zeichne eine Linie
    turtle.forward(100.0);
    
    // Drehe nach links
    turtle.left(90.0);
    
    // Zeichne noch eine Linie
    turtle.forward(100.0);
    
    // Drehe nach rechts
    turtle.right(45.0);
    
    // Zeichne eine dritte Linie
    turtle.forward(100.0);
}
