//! Beispiel: Haus vom Nikolaus
//!
//! Dieses Programm zeichnet das bekannte "Haus vom Nikolaus" - eine klassische Zeichenübung.

use turtle_lib::*;

#[turtle_main]
fn main() {
    turtle.set_pen_color(RED);
    
    // Startposition: unten links
    // Zeichne das Haus in einem Zug ohne Stift heben
    
    // Untere horizontale Linie nach rechts
    turtle.forward(100.0);
    
    // Diagonale nach oben links (Dach rechts)
    turtle.left(135.0);
    turtle.forward(70.71);  // sqrt(50^2 + 50^2) ≈ 70.71
    
    // Dach-Spitze nach rechts oben
    turtle.right(90.0);
    turtle.forward(70.71);
    
    // Diagonale nach unten links zurück
    turtle.right(135.0);
    turtle.forward(100.0);
    
    // Linke vertikale Seite nach oben
    turtle.left(90.0);
    turtle.forward(100.0);
    
    // Obere horizontale Linie nach rechts
    turtle.left(90.0);
    turtle.forward(100.0);
    
    // Rechte vertikale Seite nach unten
    turtle.left(90.0);
    turtle.forward(100.0);
    
    // Diagonale Linie von unten rechts nach oben links
    turtle.left(135.0);
    turtle.forward(141.42);  // sqrt(100^2 + 100^2) ≈ 141.42
}
