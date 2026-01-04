//! Beispiel: Halbkreis nach rechts
//!
//! Dieses Programm zeichnet einen blauen Halbkreis nach rechts.

use turtle_lib::*;

#[turtle_main]
fn main() {
    turtle.set_pen_color(BLUE);
    turtle.circle_right(50.0, 180.0, 36);
    // Dass die Bilder nicht so riesig sind, fügen wir hier noch ein paar Linien hinzu
    turtle.set_pen_color(WHITE);
    turtle.pen_up();
    turtle.backward(200.0);
    turtle.pen_down();
    turtle.forward(1.0);
    turtle.pen_up();
    turtle.forward(400.0);
    turtle.pen_down();
    turtle.forward(1.0);
}
