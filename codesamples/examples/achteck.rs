//! Beispiel: Achteck mit Schleife
//!
//! Dieses Programm zeichnet ein Achteck mit einer Schleife.

use turtle_lib::*;

#[turtle_main]
fn main() {
    turtle.set_pen_color(PURPLE);
    
    // Zeichne ein Achteck (8 Seiten)
    for _ in 0..8 {
        turtle.forward(50.0);
        turtle.right(45.0);  // 360 / 8 = 45 Grad
    }
}
