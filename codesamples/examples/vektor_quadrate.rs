//! Beispiel: Mehrere Quadrate mit Vec
//!
//! Zeichnet Quadrate in verschiedenen Größen.

use turtle_lib::*;

#[turtle_main]
fn main() {
    // ANCHOR: main
    let groessen = vec![30.0, 50.0, 70.0];
    
    for groesse in &groessen {
        // Zeichne ein Quadrat
        for _ in 0..4 {
            turtle.forward(*groesse);  // Das * holt den Wert
            turtle.right(90.0);
        }
        // Bewege dich nach rechts
        turtle.pen_up();
        turtle.forward(groesse + 20.0);
        turtle.pen_down();
    }
    // ANCHOR_END: main
}
