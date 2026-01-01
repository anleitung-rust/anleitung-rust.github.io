//! Beispiel: Ein Quadrat mit Schleife zeichnen
//!
//! Dieses Programm zeichnet ein Quadrat mit einer Schleife.

use turtle_lib::*;

#[turtle_main]
fn main() {
    // Setze die Stiftfarbe auf Blau
    turtle.set_pen_color(BLUE);
    
    // Zeichne ein Quadrat mit 4 Seiten
    for _ in 0..4 {
        turtle.forward(100.0);  // Gehe 100 Schritte vorwärts
        turtle.right(90.0);     // Drehe 90 Grad nach rechts
    }
}
