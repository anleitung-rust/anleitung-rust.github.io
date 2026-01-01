//! Beispiel: Variablen verwenden
//!
//! Dieses Programm verwendet Variablen für die Größe des Quadrats.

use turtle_lib::*;

#[turtle_main]
fn main() {
    turtle.set_pen_color(GREEN);
    
    // Eine Variable für die Seitenlänge
    let seitenlaenge = 80.0;
    
    // Zeichne ein Quadrat mit dieser Seitenlänge
    for _ in 0..4 {
        turtle.forward(seitenlaenge);
        turtle.right(90.0);
    }
}
