//! Beispiel: Vieleck mit berechnetem Winkel
//!
//! Der Drehwinkel wird aus einer Rechenoperation berechnet.

use macroquad::color::PINK;
use turtle_lib::*;

#[turtle_main]
fn main() {
    // ANCHOR: main
    let seitenlaenge = 60.0;
    let anzahl_seiten = 8;
    let winkel = 360.0 / anzahl_seiten as f64;

    turtle.set_pen_color(PINK);

    for _ in 0..anzahl_seiten {
        turtle.forward(seitenlaenge);
        turtle.right(winkel);
    }
    // ANCHOR_END: main
}
