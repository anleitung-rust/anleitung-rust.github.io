//! Beispiel: Ein Quadrat zeichnen (ohne Schleife)
//!
//! Dieses Programm zeichnet ein einfaches Quadrat auf dem Bildschirm.

use turtle_lib::*;

#[turtle_main]
fn main() {
    // Setze die Stiftfarbe auf Blau
    turtle.set_pen_color(BLUE);
    
    // Zeichne die erste Seite
    turtle.forward(100.0);  // Gehe 100 Schritte vorwärts
    turtle.right(90.0);     // Drehe 90 Grad nach rechts
    
    // Zeichne die zweite Seite
    turtle.forward(100.0);
    turtle.right(90.0);
    
    // Zeichne die dritte Seite
    turtle.forward(100.0);
    turtle.right(90.0);
    
    // Zeichne die vierte Seite
    turtle.forward(100.0);
    turtle.right(90.0);
}
