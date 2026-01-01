//! Beispiel: Stern
//!
//! Dieses Programm zeichnet einen fünfzackigen Stern.

use turtle_lib::*;

#[turtle_main]
fn main() {
    turtle.set_pen_color(GOLD);
    
    // Zeichne einen Stern mit 5 Zacken
    for _ in 0..5 {
        turtle.forward(100.0);
        turtle.right(144.0);  // 720 / 5 = 144 Grad für einen Stern
    }
}
