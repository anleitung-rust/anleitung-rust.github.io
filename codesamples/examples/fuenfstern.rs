//! Beispiel: Fünfstern
//!
//! Dieses Programm zeichnet einen fünfzackigen Stern ohne Schleifen zu verwenden.

use turtle_lib::*;

#[turtle_main]
fn main() {
    turtle.set_pen_color(GOLD);
    
    // Zeichne einen Stern mit 5 Zacken - jede Zacke einzeln
    
    // Erste Zacke
    turtle.forward(100.0);
    turtle.right(144.0);
    
    // Zweite Zacke
    turtle.forward(100.0);
    turtle.right(144.0);
    
    // Dritte Zacke
    turtle.forward(100.0);
    turtle.right(144.0);
    
    // Vierte Zacke
    turtle.forward(100.0);
    turtle.right(144.0);
    
    // Fünfte Zacke
    turtle.forward(100.0);
    turtle.right(144.0);
}
