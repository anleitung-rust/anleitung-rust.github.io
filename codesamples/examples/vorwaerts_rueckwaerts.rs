//! Beispiel: Vorwärts und rückwärts bewegen
//!
//! Dieses Programm zeigt, wie man die Schildkröte vorwärts und rückwärts bewegt.

use turtle_lib::*;

#[turtle_main]
fn main() {
    turtle.set_pen_color(RED);

    // Gehe vorwärts
    turtle.forward(100.0);

    turtle.set_pen_color(BLUE);

    // Gehe rückwärts
    turtle.backward(50.0);
}
