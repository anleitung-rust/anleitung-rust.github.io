//! Beispiel: Haus vom Nikolaus
//!
//! Dieses Programm zeichnet das bekannte "Haus vom Nikolaus" - eine klassische Zeichenübung.

use turtle_lib::*;

#[turtle_main]
fn main() {
    turtle.set_pen_color(RED);
    
    // Das klassische "Haus vom Nikolaus" - gezeichnet in einem Zug
    
    // Starte unten links, zeichne das Quadrat
    turtle.forward(100.0);      // Unten nach rechts
    turtle.left(90.0);
    turtle.forward(100.0);      // Rechts nach oben
    turtle.left(90.0);
    turtle.forward(100.0);      // Oben nach links
    turtle.left(90.0);
    turtle.forward(100.0);      // Links nach unten (zurück zum Start)
    
    // Jetzt die Diagonalen und das Dach
    turtle.left(45.0);          // Drehe zur ersten Diagonale
    turtle.forward(141.42);     // Diagonale: ca. 141 (≈ 100×√2 für ein 100×100 Quadrat)
    turtle.left(90.0);
    turtle.forward(70.71);      // Dach: ca. 71 (≈ 50×√2)
    turtle.left(90.0);
    turtle.forward(70.71);      // Dach: ca. 71 (≈ 50×√2)
    turtle.left(90.0);
    turtle.forward(141.42);     // Diagonale: ca. 141 (≈ 100×√2)
}
