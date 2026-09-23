//! Beispiel: Text mit Turtle
//!
//! Zeigt, wie man Text mit der Turtle kombiniert.

use turtle_lib::*;

#[turtle_main]
fn main() {
    // ANCHOR: main
    let name = "..."; // Dein Name hier

    // Zeichne ein Quadrat
    for _ in 0..4 {
        turtle.write_text(&format!("Hallo, {}!", name), 30.0);
        turtle.forward(200.0);
        turtle.right(90.0);
    }
    // ANCHOR_END: main
}
