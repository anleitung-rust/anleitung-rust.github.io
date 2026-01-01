//! Beispiel: Farben verwenden
//!
//! Dieses Programm zeigt verschiedene Farben.

use turtle_lib::*;

#[turtle_main]
fn main() {
    // Rote Linie
    turtle.set_pen_color(RED);
    turtle.forward(100.0);
    
    // Drehe und zeichne eine blaue Linie
    turtle.left(90.0);
    turtle.set_pen_color(BLUE);
    turtle.forward(100.0);
    
    // Drehe und zeichne eine grüne Linie
    turtle.left(90.0);
    turtle.set_pen_color(GREEN);
    turtle.forward(100.0);
    
    // Drehe und zeichne eine gelbe Linie
    turtle.left(90.0);
    turtle.set_pen_color(YELLOW);
    turtle.forward(100.0);
}
