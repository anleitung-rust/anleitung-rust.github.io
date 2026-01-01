//! Beispiel: Mehrere Variablen
//!
//! Dieses Programm verwendet mehrere Variablen.

use turtle_lib::*;

#[turtle_main]
fn main() {
    // Variablen für verschiedene Werte
    let seitenlaenge = 60.0;
    let anzahl_seiten = 6;
    let winkel = 60.0;
    
    turtle.set_pen_color(ORANGE);
    
    // Zeichne ein Sechseck
    for _ in 0..anzahl_seiten {
        turtle.forward(seitenlaenge);
        turtle.right(winkel);
    }
}
