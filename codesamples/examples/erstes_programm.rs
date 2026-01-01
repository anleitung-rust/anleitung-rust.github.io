//! Erstes Beispiel: Einfache Linien zeichnen
//!
//! Dieses Programm zeichnet ein einfaches L auf dem Bildschirm.

use turtle_lib::*;

#[turtle_main]
fn main() {
    // Setze die Stiftfarbe auf Blau
    turtle.set_pen_color(BLUE);
    
    // Zeichne eine Linie nach rechts
    turtle.forward(100.0);
    
    // Drehe nach unten
    turtle.right(90.0);
    
    // Zeichne eine Linie nach unten
    turtle.forward(100.0);
}
