//! Beispiel: Spirale
//!
//! Dieses Programm zeichnet eine Spirale.

use turtle_lib::*;

#[turtle_main]
fn main() {
    turtle.set_pen_color(PURPLE);
    
    let mut laenge = 5.0;
    
    // Zeichne eine Spirale
    for _ in 0..50 {
        turtle.forward(laenge);
        turtle.right(90.0);
        laenge = laenge + 3.0;  // Erhöhe die Länge bei jedem Schritt
    }
}
